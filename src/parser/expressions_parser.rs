// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{
    lexer::Token::*,
    lexer::{ParseSession, Token},
    parser::parse_any_in_region,
};
use core::str::Split;
use plc_ast::{
    ast::{AstFactory, AstId, AstNode, AstStatement, DirectAccessType, Operator},
    literals::{AstLiteral, Time},
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use regex::{Captures, Regex};
use std::{ops::Range, str::FromStr};

use super::parse_hardware_access;

macro_rules! parse_left_associative_expression {
    ($lexer: expr, $action : expr,
        $( $pattern:pat_param )|+,
    ) => {
        {
            let mut left = $action($lexer);
            while matches!($lexer.token, $( $pattern )|+)  {
                let operator = match to_operator(&$lexer.token) {
                    Some(operator) => operator,
                    None => break,
                };
                $lexer.advance();
                let right = $action($lexer);
                left = AstFactory::create_binary_expression(left, operator, right, $lexer.next_id());
            }
            left
        }
    };
}

/// parse_expression(): returns expression as Statement. if a parse error
/// is encountered, the erroneous part of the AST will consist of an
/// EmptyStatement and a diagnostic will be logged. That case is different from
/// only an EmptyStatement returned, which does not denote an error condition.
pub fn parse_expression(lexer: &mut ParseSession) -> AstNode {
    if lexer.token == KeywordSemicolon {
        AstFactory::create_empty_statement(lexer.location(), lexer.next_id())
    } else {
        parse_expression_list(lexer)
    }
}

pub fn parse_expression_list(lexer: &mut ParseSession) -> AstNode {
    let start = lexer.location();
    let left = parse_range_statement(lexer);
    if lexer.token == KeywordComma {
        let mut expressions = vec![];
        // this starts an expression list
        while lexer.token == KeywordComma {
            lexer.advance();
            if !lexer.closes_open_region(&lexer.token) {
                expressions.push(parse_range_statement(lexer));
            }
        }
        // we may have parsed no additional expression because of trailing comma
        if !expressions.is_empty() {
            expressions.insert(0, left);
            return AstFactory::create_expression_list(
                expressions,
                start.span(&lexer.last_location()),
                lexer.next_id(),
            );
        }
    }
    left
}

pub(crate) fn parse_range_statement(lexer: &mut ParseSession) -> AstNode {
    let start = parse_or_expression(lexer);

    if lexer.token == KeywordDotDot {
        lexer.advance();
        let end = parse_or_expression(lexer);
        return AstFactory::create_range_statement(start, end, lexer.next_id());
    }
    start
}

// OR
fn parse_or_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(lexer, parse_xor_expression, OperatorOr,)
}

// XOR
fn parse_xor_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(lexer, parse_and_expression, OperatorXor,)
}

// AND
fn parse_and_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(lexer, parse_equality_expression, OperatorAmp | OperatorAnd,)
}

//EQUALITY  =, <>
fn parse_equality_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(lexer, parse_compare_expression, OperatorEqual | OperatorNotEqual,)
}

//COMPARE <, >, <=, >=
fn parse_compare_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(
        lexer,
        parse_additive_expression,
        OperatorLess | OperatorGreater | OperatorLessOrEqual | OperatorGreaterOrEqual,
    )
}

// Addition +, -
fn parse_additive_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(lexer, parse_multiplication_expression, OperatorPlus | OperatorMinus,)
}

// Multiplication *, /, MOD
fn parse_multiplication_expression(lexer: &mut ParseSession) -> AstNode {
    parse_left_associative_expression!(
        lexer,
        parse_exponent_expression,
        OperatorMultiplication | OperatorDivision | OperatorModulo,
    )
}

// Expoent **
fn parse_exponent_expression(lexer: &mut ParseSession) -> AstNode {
    //This is always parsed as a function call to the EXPT function
    //Parse left
    let mut left = parse_unary_expression(lexer);
    while matches!(lexer.token, OperatorExponent) {
        lexer.advance();
        let right = parse_unary_expression(lexer);
        let span = left.get_location().span(&right.get_location());
        left =
            AstFactory::create_call_to_with_ids("EXPT", vec![left, right], &span, lexer.id_provider.clone());
    }
    left
}

// UNARY -x, NOT x
fn parse_unary_expression(lexer: &mut ParseSession) -> AstNode {
    // collect all consecutive operators
    let start_location = lexer.location();
    let mut operators = vec![];
    while let Some(operator) = match lexer.token {
        OperatorNot => Some(Operator::Not),
        OperatorPlus => Some(Operator::Plus),
        OperatorMinus => Some(Operator::Minus),
        _ => None,
    } {
        operators.push(operator);
        lexer.advance();
    }
    // created nested statements if necessary (e.g. &&)
    let init = parse_leaf_expression(lexer);
    operators.iter().rev().fold(init, |expression, operator| {
        let expression_location = expression.get_location();
        let location = start_location.span(&expression_location);

        match (&operator, &expression.get_stmt()) {
            (Operator::Minus, AstStatement::Literal(AstLiteral::Integer(value))) => {
                AstNode::new_literal(AstLiteral::new_integer(-value), lexer.next_id(), location)
            }

            (Operator::Plus, AstStatement::Literal(AstLiteral::Integer(value))) => {
                AstNode::new_literal(AstLiteral::new_integer(*value), lexer.next_id(), location)
            }

            // Return the reference itself instead of wrapping it inside a `AstStatement::UnaryExpression`
            (Operator::Plus, AstStatement::Identifier(name)) => {
                AstFactory::create_identifier(name, &location, lexer.next_id())
            }

            _ => AstFactory::create_unary_expression(*operator, expression, location, lexer.next_id()),
        }
    })
}

fn to_operator(token: &Token) -> Option<Operator> {
    match token {
        OperatorPlus => Some(Operator::Plus),
        OperatorMinus => Some(Operator::Minus),
        OperatorMultiplication => Some(Operator::Multiplication),
        OperatorExponent => Some(Operator::Exponentiation),
        OperatorDivision => Some(Operator::Division),
        OperatorEqual => Some(Operator::Equal),
        OperatorNotEqual => Some(Operator::NotEqual),
        OperatorLess => Some(Operator::Less),
        OperatorGreater => Some(Operator::Greater),
        OperatorLessOrEqual => Some(Operator::LessOrEqual),
        OperatorGreaterOrEqual => Some(Operator::GreaterOrEqual),
        OperatorModulo => Some(Operator::Modulo),
        OperatorAnd | OperatorAmp => Some(Operator::And),
        OperatorOr => Some(Operator::Or),
        OperatorXor => Some(Operator::Xor),
        OperatorNot => Some(Operator::Not),
        _ => None,
    }
}

// Literals, Identifiers, etc.
fn parse_leaf_expression(lexer: &mut ParseSession) -> AstNode {
    let literal_parse_result = match lexer.token {
        OperatorMultiplication => parse_vla_range(lexer),
        _ => parse_call_statement(lexer),
    };

    match literal_parse_result {
        Ok(statement) => {
            if lexer.token == KeywordAssignment {
                lexer.advance();
                AstFactory::create_assignment(statement, parse_range_statement(lexer), lexer.next_id())
            } else if lexer.token == KeywordOutputAssignment {
                lexer.advance();
                AstFactory::create_output_assignment(statement, parse_range_statement(lexer), lexer.next_id())
            } else {
                statement
            }
        }
        Err(diagnostic) => {
            let statement = AstFactory::create_empty_statement(diagnostic.get_location(), lexer.next_id());
            lexer.accept_diagnostic(diagnostic);
            statement
        }
    }
}

/// parse an expression at the bottom of the parse-tree.
/// leaf-expressions are literals, identifier, direct-access and parenthesized expressions
/// (since the parentheses change the parse-priority)
fn parse_atomic_leaf_expression(lexer: &mut ParseSession<'_>) -> Result<AstNode, Diagnostic> {
    // Check if we're dealing with a number that has an explicit '+' or '-' sign...

    match lexer.token {
        OperatorPlus | OperatorMinus => {
            let is_negative = lexer.token == OperatorMinus;
            lexer.advance();

            match lexer.token {
                LiteralInteger => parse_literal_number(lexer, is_negative),
                LiteralIntegerBin => parse_literal_number_with_modifier(lexer, 2, is_negative),
                LiteralIntegerOct => parse_literal_number_with_modifier(lexer, 8, is_negative),
                LiteralIntegerHex => parse_literal_number_with_modifier(lexer, 16, is_negative),
                _ => Err(Diagnostic::unexpected_token_found(
                    "Numeric Literal",
                    lexer.slice(),
                    lexer.location(),
                )),
            }
        }
        KeywordParensOpen => {
            parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
                lexer.advance(); // eat KeywordParensOpen

                let start = lexer.last_location();
                let expr = parse_expression(lexer);

                Ok(AstFactory::create_paren_expression(expr, start.span(&lexer.location()), lexer.next_id()))
            })
        }
        Identifier => Ok(parse_identifier(lexer)),
        HardwareAccess((hw_type, access_type)) => parse_hardware_access(lexer, hw_type, access_type),
        LiteralInteger => parse_literal_number(lexer, false),
        LiteralIntegerBin => parse_literal_number_with_modifier(lexer, 2, false),
        LiteralIntegerOct => parse_literal_number_with_modifier(lexer, 8, false),
        LiteralIntegerHex => parse_literal_number_with_modifier(lexer, 16, false),
        LiteralDate => parse_literal_date(lexer),
        LiteralTimeOfDay => parse_literal_time_of_day(lexer),
        LiteralTime => parse_literal_time(lexer),
        LiteralDateAndTime => parse_literal_date_and_time(lexer),
        LiteralString => parse_literal_string(lexer, false),
        LiteralWideString => parse_literal_string(lexer, true),
        LiteralTrue => parse_bool_literal(lexer, true),
        LiteralFalse => parse_bool_literal(lexer, false),
        LiteralNull => parse_null_literal(lexer),
        KeywordSquareParensOpen => parse_array_literal(lexer),
        DirectAccess(access) => parse_direct_access(lexer, access),
        _ => {
            if lexer.closing_keywords.contains(&vec![KeywordParensClose])
                && matches!(lexer.last_token, KeywordOutputAssignment | KeywordAssignment)
            {
                // due to closing keyword ')' and last_token '=>' / ':='
                // we are probably in a call statement missing a parameter assignment 'foo(param := );
                // optional parameter assignments are allowed, validation should handle any unwanted cases
                Ok(AstFactory::create_empty_statement(lexer.location(), lexer.next_id()))
            } else {
                Err(Diagnostic::unexpected_token_found("Literal", lexer.slice(), lexer.location()))
            }
        }
    }
}

fn parse_identifier(lexer: &mut ParseSession<'_>) -> AstNode {
    AstFactory::create_identifier(&lexer.slice_and_advance(), &lexer.last_location(), lexer.next_id())
}

fn parse_vla_range(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    lexer.advance();
    Ok(AstFactory::create_vla_range_statement(lexer.last_location(), lexer.next_id()))
}

fn parse_array_literal(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let start = lexer.range().start;
    lexer.expect(KeywordSquareParensOpen)?;
    lexer.advance();
    let elements = Some(Box::new(parse_expression(lexer)));
    let end = lexer.range().end;
    lexer.expect(KeywordSquareParensClose)?;
    lexer.advance();

    Ok(AstNode::new_literal(
        AstLiteral::new_array(elements),
        lexer.next_id(),
        lexer.source_range_factory.create_range(start..end),
    ))
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_bool_literal(lexer: &mut ParseSession, value: bool) -> Result<AstNode, Diagnostic> {
    let location = lexer.location();
    lexer.advance();
    Ok(AstNode::new_literal(AstLiteral::new_bool(value), lexer.next_id(), location))
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_null_literal(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let location = lexer.location();
    lexer.advance();

    Ok(AstNode::new_literal(AstLiteral::new_null(), lexer.next_id(), location))
}

pub fn parse_call_statement(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let reference = parse_qualified_reference(lexer)?;

    // is this a callstatement?
    if lexer.try_consume(&KeywordParensOpen) {
        let start = reference.get_location();
        // Call Statement
        let call_statement = if lexer.try_consume(&KeywordParensClose) {
            AstFactory::create_call_statement(reference, None, lexer.next_id(), start.span(&lexer.location()))
        } else {
            parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
                AstFactory::create_call_statement(
                    reference,
                    Some(parse_expression_list(lexer)),
                    lexer.next_id(),
                    start.span(&lexer.location()),
                )
            })
        };
        Ok(call_statement)
    } else {
        Ok(reference)
    }
}

pub fn parse_qualified_reference(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let mut current = None;
    let mut pos = lexer.parse_progress - 1; // force an initial loop

    // as long as we parse something we keep eating stuff eagerly
    while lexer.parse_progress > pos {
        pos = lexer.parse_progress;
        match (
            current,
            // only test for the tokens without eating it (Amp must not be consumed if it is in the middle of the chain)
            [KeywordDot, KeywordSquareParensOpen, OperatorDeref, OperatorAmp, TypeCastPrefix]
                .into_iter()
                .find(|it| lexer.token == *it),
        ) {
            // No base, No token -> Beginning of a qualified reference
            (None, None) => {
                let exp = parse_atomic_leaf_expression(lexer)?;
                // pack if this is something to be resolved
                current = if exp.is_identifier() {
                    Some(AstFactory::create_member_reference(exp, None, lexer.next_id()))
                } else {
                    Some(exp)
                };
            }
            // base._ -> a segment of a qualified reference, we stand right after the dot
            (Some(base), Some(KeywordDot)) => {
                lexer.advance();
                let member = if lexer.token == LiteralInteger {
                    let index = parse_strict_literal_integer(lexer)?;
                    let location = index.get_location();
                    AstFactory::create_direct_access(DirectAccessType::Bit, index, lexer.next_id(), location)
                } else {
                    parse_atomic_leaf_expression(lexer)?
                };
                current = Some(AstFactory::create_member_reference(member, Some(base), lexer.next_id()));
            }
            // CAST-Statement: INT#a.b.c
            // this means INT#(a.b.c) rather than (INT#a).b.c
            (_, Some(TypeCastPrefix)) => {
                let location_start = lexer.range().start;
                let location = lexer.location();
                let mut type_name = lexer.slice_and_advance();
                type_name.pop(); // get rid of the "#" at the end
                let stmt = parse_atomic_leaf_expression(lexer)?;
                let end = stmt.get_location();
                let type_range = lexer
                    .source_range_factory
                    .create_range(location_start..(location_start + type_name.len()));
                current = Some(AstFactory::create_cast_statement(
                    AstFactory::create_member_reference(
                        AstFactory::create_identifier(type_name.as_str(), &type_range, lexer.next_id()),
                        None,
                        lexer.next_id(),
                    ),
                    stmt,
                    &location.span(&end),
                    lexer.next_id(),
                ));
            }
            (Some(base), Some(KeywordSquareParensOpen)) => {
                lexer.advance();
                let index_reference =
                    parse_any_in_region(lexer, vec![KeywordSquareParensClose], parse_expression);
                let new_location = base.get_location().span(&lexer.last_location());
                current = Some({
                    AstFactory::create_index_reference(
                        index_reference,
                        Some(base),
                        lexer.next_id(),
                        new_location,
                    )
                })
            }
            (Some(base), Some(OperatorDeref)) => {
                lexer.advance();
                let new_location = base.get_location().span(&lexer.last_location());
                current = Some(AstFactory::create_deref_reference(base, lexer.next_id(), new_location))
            }
            (None, Some(OperatorAmp)) => {
                lexer.advance();
                let op_location = lexer.last_location();
                // the address-of-operator has different order compared ot other segments, we first see the operator, then
                // we expect the expression. so writing &a.b.c is more of &(a.b.c) instead of (&a).b.c.
                // So we expect NO base, and operator and we parse the base now
                let base = parse_call_statement(lexer)?;
                let new_location = op_location.span(&base.get_location());
                current = Some(AstFactory::create_address_of_reference(base, lexer.next_id(), new_location))
            }
            (last_current, _) => {
                current = last_current; // exit the loop
            }
        }
    }
    if let Some(current) = current {
        Ok(current)
    } else {
        parse_atomic_leaf_expression(lexer)
    }
}

fn parse_direct_access(lexer: &mut ParseSession, access: DirectAccessType) -> Result<AstNode, Diagnostic> {
    //Consume the direct access
    let location = lexer.location();
    lexer.advance();
    //The next token can either be an integer or an identifier
    let index = match lexer.token {
        LiteralInteger => parse_strict_literal_integer(lexer),
        Identifier => {
            let location = lexer.location();
            Ok(AstFactory::create_member_reference(
                AstFactory::create_identifier(lexer.slice_and_advance().as_str(), &location, lexer.next_id()),
                None,
                lexer.next_id(),
            ))
        }
        _ => Err(Diagnostic::unexpected_token_found("Integer or Reference", lexer.slice(), lexer.location())),
    }?;
    let location = location.span(&lexer.last_location());
    Ok(AstFactory::create_direct_access(access, index, lexer.next_id(), location))
}

fn parse_literal_number_with_modifier(
    lexer: &mut ParseSession,
    radix: u32,
    is_negative: bool,
) -> Result<AstNode, Diagnostic> {
    // we can safely unwrap the number string, since the token has
    // been matched using regular expressions
    let location = lexer.location();
    let token = lexer.slice_and_advance();
    let number_str = token.split('#').last().expect("token with '#'");
    let number_str = number_str.replace('_', "");

    // again, the parsed number can be safely unwrapped.
    let value = i128::from_str_radix(number_str.as_str(), radix).expect("valid i128");
    let value = if is_negative { -value } else { value };
    Ok(AstNode::new_literal(AstLiteral::new_integer(value), lexer.next_id(), location))
}

fn parse_literal_number(lexer: &mut ParseSession, is_negative: bool) -> Result<AstNode, Diagnostic> {
    let location = if is_negative {
        //correct the location if we just parsed a minus before
        lexer.last_range.start..lexer.range().end
    } else {
        lexer.range()
    };
    let result = lexer.slice_and_advance();
    if result.to_lowercase().contains('e') {
        let value = result.replace('_', "");
        //Treat exponents as reals
        return Ok(AstNode::new_literal(
            AstLiteral::new_real(value),
            lexer.next_id(),
            lexer.source_range_factory.create_range(location),
        ));
    } else if lexer.try_consume(&KeywordDot) {
        return parse_literal_real(lexer, result, location, is_negative);
    } else if lexer.try_consume(&KeywordParensOpen) {
        let start = location.start;
        let multiplier = result.parse::<u32>().map_err(|e| {
            Diagnostic::new(format!("Failed parsing number {result}"))
                .with_error_code("E011")
                .with_location(lexer.source_range_factory.create_range(location))
                .with_internal_error(e.into())
        })?;
        let element = parse_expression(lexer);
        lexer.expect(KeywordParensClose)?;
        let end = lexer.range().end;
        lexer.advance();
        return Ok(AstFactory::create_multiplied_statement(
            multiplier,
            element,
            lexer.source_range_factory.create_range(start..end),
            lexer.next_id(),
        ));
    }

    // parsed number value can be safely unwrapped
    let result = result.replace('_', "");

    let value = result.parse::<i128>().expect("valid i128");
    let value = if is_negative { -value } else { value };

    Ok(AstNode::new_literal(
        AstLiteral::new_integer(value),
        lexer.next_id(),
        lexer.source_range_factory.create_range(location),
    ))
}

/// Parses a literal integer without considering Signs or the Possibility of a Floating Point/ Exponent
pub fn parse_strict_literal_integer(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    //correct the location if we just parsed a minus before
    let location = lexer.location();
    let result = lexer.slice_and_advance();
    // parsed number value can be safely unwrapped
    let result = result.replace('_', "");
    if result.to_lowercase().contains('e') {
        Err(Diagnostic::unexpected_token_found("Integer", &format!("Exponent value: {result}"), location))
    } else {
        let value = result.parse::<i128>().expect("valid i128");
        Ok(AstNode::new_literal(AstLiteral::new_integer(value), lexer.next_id(), location))
    }
}

fn parse_number<F: FromStr>(text: &str, location: &SourceLocation) -> Result<F, Diagnostic> {
    text.parse::<F>().map_err(|_| {
        Diagnostic::new(format!("Failed parsing number {text}"))
            .with_error_code("E011")
            .with_location(location.clone())
    })
}

fn parse_date_from_string(text: &str, location: SourceLocation, id: AstId) -> Result<AstNode, Diagnostic> {
    let mut segments = text.split('-');

    //we can safely expect 3 numbers
    let year = segments
        .next()
        .map(|s| parse_number::<i32>(s, &location))
        .expect("year-segment - tokenizer broken?")?;
    let month = segments
        .next()
        .map(|s| parse_number::<u32>(s, &location))
        .expect("month-segment - tokenizer broken?")?;
    let day = segments
        .next()
        .map(|s| parse_number::<u32>(s, &location))
        .expect("day-segment - tokenizer broken?")?;

    Ok(AstNode::new_literal(AstLiteral::new_date(year, month, day), id, location))
}

fn parse_literal_date_and_time(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let last_minus_location = slice.rfind('-').expect("unexpected date-and-time syntax");

    let (_, date_and_time) = slice.split_at(hash_location + 1); //get rid of the prefix
    let (date, time) = date_and_time.split_at(last_minus_location - hash_location);

    //we can safely expect 3 numbers
    let mut segments = date.split('-');
    let year = parse_number::<i32>(segments.next().expect("unexpected date-and-time syntax"), &location)?;
    let month = parse_number::<u32>(segments.next().expect("unexpected date-and-time syntax"), &location)?;
    let day = parse_number::<u32>(segments.next().expect("unexpected date-and-time syntax"), &location)?;

    //we can safely expect 3 numbers
    let mut segments = time.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(&mut segments, &location)?;

    Ok(AstNode::new_literal(
        AstLiteral::new_date_and_time(year, month, day, hour, min, sec, nano),
        lexer.next_id(),
        location,
    ))
}

fn parse_literal_date(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    parse_date_from_string(slice, location, lexer.next_id())
}

fn parse_literal_time_of_day(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    let location = lexer.location();
    //get rid of TOD# or TIME_OF_DAY#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let mut segments = slice.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(&mut segments, &location)?;

    Ok(AstNode::new_literal(AstLiteral::new_time_of_day(hour, min, sec, nano), lexer.next_id(), location))
}

fn parse_time_of_day(
    time: &mut Split<char>,
    location: &SourceLocation,
) -> Result<(u32, u32, u32, u32), Diagnostic> {
    let hour = parse_number::<u32>(time.next().expect("expected hours"), location)?;
    let min = parse_number::<u32>(time.next().expect("expected minutes"), location)?;

    // doesn't necessarily have to have seconds, e.g [12:00] is also valid
    let sec = match time.next() {
        Some(v) => parse_number::<f64>(v, location)?,
        None => 0.0,
    };

    let nano = (sec.fract() * 1e+9_f64).round() as u32;

    Ok((hour, min, sec.floor() as u32, nano))
}

fn parse_literal_time(lexer: &mut ParseSession) -> Result<AstNode, Diagnostic> {
    const POS_D: usize = 0;
    const POS_H: usize = 1;
    const POS_M: usize = 2;
    const POS_S: usize = 3;
    const POS_MS: usize = 4;
    const POS_US: usize = 5;
    const POS_NS: usize = 6;
    let location = lexer.location();
    //get rid of T# or TIME#
    let slice = lexer.slice_and_advance();
    let (_, slice) = slice.split_at(slice.find('#').unwrap_or_default() + 1); //get rid of the prefix

    let mut chars = slice.char_indices();
    let mut char = chars.next();

    let is_negative = char.map(|(_, c)| c == '-').unwrap_or(false);
    if is_negative {
        char = chars.next();
    }

    let mut values: [Option<f64>; 7] = [None, None, None, None, None, None, None];

    let mut prev_pos = POS_D;
    while char.is_some() {
        //expect a number
        let number = {
            let start = char.expect("char").0;
            //just eat all the digits
            char = chars.find(|(_, ch)| !ch.is_ascii_digit() && !ch.eq(&'.'));
            char.ok_or_else(|| {
                Diagnostic::new("Invalid TIME Literal: Cannot parse segment.")
                    .with_error_code("E010")
                    .with_location(location.clone())
            })
            .and_then(|(index, _)| parse_number::<f64>(&slice[start..index], &location))?
        };

        //expect a unit
        let unit = {
            let start = char.map(|(index, _)| index).ok_or_else(|| {
                Diagnostic::new("Invalid TIME Literal: Missing unit (d|h|m|s|ms|us|ns)")
                    .with_error_code("E010")
                    .with_location(location.clone())
            })?;

            //just eat all the characters
            char = chars.find(|(_, ch)| !ch.is_ascii_alphabetic());
            &slice[start..char.unwrap_or((slice.len(), ' ')).0]
        }
        .to_lowercase();

        //now assign the number to the according segment of the value's array
        let position = match unit.as_str() {
            "d" => Some(POS_D),
            "h" => Some(POS_H),
            "m" => Some(POS_M),
            "s" => Some(POS_S),
            "ms" => Some(POS_MS),
            "us" => Some(POS_US),
            "ns" => Some(POS_NS),
            _ => None,
        };
        if let Some(position) = position {
            //check if we assign out of order - every assignment before must have been a smaller position
            if prev_pos > position {
                return Err(Diagnostic::new("Invalid TIME Literal: segments out of order, use d-h-m-s-ms")
                    .with_error_code("E010")
                    .with_location(location));
            }
            prev_pos = position; //remember that we wrote this position

            if values[position].is_some() {
                return Err(Diagnostic::new("Invalid TIME Literal: segments must be unique")
                    .with_error_code("E010")
                    .with_location(location));
            }
            values[position] = Some(number); //store the number
        } else {
            return Err(Diagnostic::new(format!("Invalid TIME Literal: illegal unit '{unit}'"))
                .with_error_code("E010")
                .with_location(location));
        }
    }

    Ok(AstNode::new_literal(
        AstLiteral::Time(Time {
            day: values[POS_D].unwrap_or_default(),
            hour: values[POS_H].unwrap_or_default(),
            min: values[POS_M].unwrap_or_default(),
            sec: values[POS_S].unwrap_or_default(),
            milli: values[POS_MS].unwrap_or_default(),
            micro: values[POS_US].unwrap_or_default(),
            nano: values[POS_NS].map(|it| it as u32).unwrap_or(0u32),
            negative: is_negative,
        }),
        lexer.next_id(),
        location,
    ))
}

fn trim_quotes(quoted_string: &str) -> String {
    quoted_string[1..quoted_string.len() - 1].to_string()
}

fn handle_special_chars(string: &str, is_wide: bool) -> String {
    let (re, re_hex) = if is_wide {
        (
            Regex::new(r#"(\$([lLnNpPrRtT$"]))"#).expect("valid regex"), //Cannot fail
            Regex::new(r"(\$([[:xdigit:]]{2}){2})+").expect("valid regex"), //Cannot fail
        )
    } else {
        (
            Regex::new(r"(\$([lLnNpPrRtT$']))").expect("valid regex"), //Cannot fail
            Regex::new(r"(\$([[:xdigit:]]{2}))+").expect("valid regex"), //Cannot fail
        )
    };

    // separated re and re_hex to minimize copying
    let res = re.replace_all(string, |caps: &Captures| {
        let cap_str = &caps[1];
        match cap_str {
            "$l" | "$L" => "\n",
            "$n" | "$N" => "\n",
            "$p" | "$P" => "\x0C",
            "$r" | "$R" => "\r",
            "$t" | "$T" => "\t",
            "$$" => "$",
            "$'" => "\'",
            "$\"" => "\"",
            _ => unreachable!(),
        }
    });

    re_hex
        .replace_all(&res, |caps: &Captures| {
            let hex = &caps[0];
            let hex_vals: Vec<&str> = hex.split('$').filter(|it| !it.is_empty()).collect();
            let res = if is_wide {
                let hex_vals: Vec<u16> =
                    hex_vals.iter().map(|it| u16::from_str_radix(it, 16).unwrap_or_default()).collect();
                String::from_utf16_lossy(&hex_vals)
            } else {
                let hex_vals: Vec<u8> =
                    hex_vals.iter().map(|it| u8::from_str_radix(it, 16).unwrap_or_default()).collect();
                String::from_utf8_lossy(&hex_vals).to_string()
            };
            res
        })
        .into()
}

fn parse_literal_string(lexer: &mut ParseSession, is_wide: bool) -> Result<AstNode, Diagnostic> {
    let result = lexer.slice();
    let location = lexer.location();

    let string_literal = Ok(AstNode::new_literal(
        AstLiteral::new_string(handle_special_chars(&trim_quotes(result), is_wide), is_wide),
        lexer.next_id(),
        location,
    ));
    lexer.advance();
    string_literal
}

fn parse_literal_real(
    lexer: &mut ParseSession,
    integer: String,
    integer_range: Range<usize>,
    is_negative: bool,
) -> Result<AstNode, Diagnostic> {
    if lexer.token == LiteralInteger {
        let start = integer_range.start;
        let end = lexer.range().end;
        let fractional = lexer.slice_and_advance();
        let value = format!("{}{}.{}", if is_negative { "-" } else { "" }, integer, fractional);
        let new_location = lexer.source_range_factory.create_range(start..end);

        Ok(AstNode::new_literal(AstLiteral::new_real(value), lexer.next_id(), new_location))
    } else {
        Err(Diagnostic::unexpected_token_found(
            "LiteralInteger or LiteralExponent",
            lexer.slice(),
            lexer.location(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::expressions_parser::handle_special_chars;

    #[test]
    fn replace_all_test() {
        // following special chars should be replaced
        let string = "a $l$L b $n$N test $p$P c $r$R d $t$T$$ $'quote$' $57 💖 $F0$9F$92$96";
        let expected = "a \n\n b \n\n test \x0C\x0C c \r\r d \t\t$ 'quote' W 💖 💖";

        let w_string = r#"a $l$L b $n$N test $p$P c $r$R d $t$T$$ $"double$" $0077 💖 $D83D$DC96"#;
        let w_expected = "a \n\n b \n\n test \x0C\x0C c \r\r d \t\t$ \"double\" w 💖 💖";

        assert_eq!(handle_special_chars(w_string, true), w_expected);
        assert_eq!(handle_special_chars(string, false), expected);
    }

    #[test]
    fn should_not_replace_test() {
        // following special chars should not be replaced
        let string = r#"$0043 $"no replace$""#;
        let expected = "\u{0}43 $\"no replace$\"";

        let w_string = r#"$57 $'no replace$'"#;
        let w_expected = "$57 $'no replace$'";

        assert_eq!(handle_special_chars(w_string, true), w_expected);
        assert_eq!(handle_special_chars(string, false), expected);
    }
}
