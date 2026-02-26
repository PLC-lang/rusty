// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{
    expect_token,
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

            // Handle negative and positive real literals directly instead of creating UnaryExpressions
            // This ensures that the minus sign is part of the literal string, which is crucial for
            // proper constant evaluation (especially for very small numbers like 1e-100)
            (Operator::Minus, AstStatement::Literal(AstLiteral::Real(value))) => {
                let negative_value = format!("-{}", value);
                AstNode::new_literal(AstLiteral::new_real(negative_value), lexer.next_id(), location)
            }

            (Operator::Plus, AstStatement::Literal(AstLiteral::Real(value))) => {
                AstNode::new_literal(AstLiteral::new_real(value.clone()), lexer.next_id(), location)
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
        Some(statement) => match lexer.token {
            KeywordAssignment => {
                lexer.advance();
                AstFactory::create_assignment(statement, parse_range_statement(lexer), lexer.next_id())
            }
            KeywordOutputAssignment => {
                lexer.advance();
                AstFactory::create_output_assignment(statement, parse_range_statement(lexer), lexer.next_id())
            }
            KeywordReferenceAssignment => {
                lexer.advance();
                AstFactory::create_ref_assignment(statement, parse_range_statement(lexer), lexer.next_id())
            }
            _ => statement,
        },
        None => {
            let statement = AstFactory::create_empty_statement(
                lexer.diagnostics.last().map_or(SourceLocation::undefined(), |d| d.get_location()),
                lexer.next_id(),
            );
            statement
        }
    }
}

/// parse an expression at the bottom of the parse-tree.
/// leaf-expressions are literals, identifier, direct-access and parenthesized expressions
/// (since the parentheses change the parse-priority)
fn parse_atomic_leaf_expression(lexer: &mut ParseSession<'_>) -> Option<AstNode> {
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
                _ => {
                    lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                        "Numeric Literal",
                        lexer.slice(),
                        lexer.location(),
                    ));
                    None
                }
            }
        }
        KeywordParensOpen => {
            parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
                lexer.advance(); // eat KeywordParensOpen

                let start = lexer.last_location();
                let expr = parse_expression(lexer);

                Some(AstFactory::create_paren_expression(
                    expr,
                    start.span(&lexer.location()),
                    lexer.next_id(),
                ))
            })
        }
        Identifier => Some(parse_identifier(lexer)),
        KeywordSuper => {
            lexer.advance();
            Some(AstFactory::create_super_reference(
                lexer.last_location(),
                lexer.try_consume(OperatorDeref).then_some(()),
                lexer.next_id(),
            ))
        }
        KeywordThis => {
            lexer.advance();
            Some(AstFactory::create_this_reference(lexer.last_location(), lexer.next_id()))
        }
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
                Some(AstFactory::create_empty_statement(lexer.location(), lexer.next_id()))
            } else {
                lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                    "Literal",
                    lexer.slice(),
                    lexer.location(),
                ));
                None
            }
        }
    }
}

fn parse_identifier(lexer: &mut ParseSession<'_>) -> AstNode {
    AstFactory::create_identifier(lexer.slice_and_advance(), lexer.last_location(), lexer.next_id())
}

fn parse_vla_range(lexer: &mut ParseSession) -> Option<AstNode> {
    lexer.advance();
    Some(AstFactory::create_vla_range_statement(lexer.last_location(), lexer.next_id()))
}

fn parse_array_literal(lexer: &mut ParseSession) -> Option<AstNode> {
    let start = lexer.range().start;
    expect_token!(lexer, KeywordSquareParensOpen, None);
    lexer.advance();
    let elements = Some(Box::new(parse_expression(lexer)));
    let end = lexer.range().end;
    expect_token!(lexer, KeywordSquareParensClose, None);
    lexer.advance();

    Some(AstNode::new_literal(
        AstLiteral::new_array(elements),
        lexer.next_id(),
        lexer.source_range_factory.create_range(start..end),
    ))
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_bool_literal(lexer: &mut ParseSession, value: bool) -> Option<AstNode> {
    let location = lexer.location();
    lexer.advance();
    Some(AstNode::new_literal(AstLiteral::new_bool(value), lexer.next_id(), location))
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_null_literal(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    lexer.advance();

    Some(AstNode::new_literal(AstLiteral::new_null(), lexer.next_id(), location))
}

pub fn parse_call_statement(lexer: &mut ParseSession) -> Option<AstNode> {
    let reference = parse_qualified_reference(lexer)?;
    let reference_loc = reference.get_location();

    // We're not dealing with a call statement here
    if !lexer.try_consume(KeywordParensOpen) {
        return Some(reference);
    }

    let call = if lexer.try_consume(KeywordParensClose) {
        AstFactory::create_call_statement(
            reference,
            None,
            lexer.next_id(),
            reference_loc.span(&lexer.location()),
        )
    } else {
        parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
            AstFactory::create_call_statement(
                reference,
                Some(parse_expression_list(lexer)),
                lexer.next_id(),
                reference_loc.span(&lexer.location()),
            )
        })
    };

    // Are we dealing with an array-index access directly after the call, e.g. `foo()[...]`?
    if lexer.try_consume(KeywordSquareParensOpen) {
        let index = parse_any_in_region(lexer, vec![KeywordSquareParensClose], parse_expression);
        let statement = AstFactory::create_index_reference(
            index,
            Some(call),
            lexer.next_id(),
            SourceLocation::undefined(),
        );

        return Some(statement);
    }

    Some(call)
}

pub fn parse_qualified_reference(lexer: &mut ParseSession) -> Option<AstNode> {
    let mut current = None;
    let mut pos = lexer.parse_progress - 1; // force an initial loop

    // as long as we parse something we keep eating stuff eagerly
    while lexer.parse_progress > pos {
        pos = lexer.parse_progress;
        match (
            current,
            // only test for the tokens without eating it
            [KeywordDot, KeywordSquareParensOpen, OperatorDeref, TypeCastPrefix]
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
            // Global Namespace Operator, e.g. `.foo`
            (None, Some(KeywordDot)) => {
                let location_dot = lexer.location();
                lexer.advance();

                let expr = parse_atomic_leaf_expression(lexer)?;
                let location = location_dot.span(&expr.location);

                current = Some(AstFactory::create_global_reference(lexer.next_id(), expr, location));
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
            (last_current, _) => {
                current = last_current; // exit the loop
            }
        }
    }
    match current {
        Some(current) => Some(current),
        None => parse_atomic_leaf_expression(lexer),
    }
}

fn parse_direct_access(lexer: &mut ParseSession, access: DirectAccessType) -> Option<AstNode> {
    //Consume the direct access
    let location = lexer.location();
    lexer.advance();
    //The next token can either be an integer or an identifier
    let index = match lexer.token {
        LiteralInteger => parse_strict_literal_integer(lexer),
        Identifier => {
            let location = lexer.location();
            Some(AstFactory::create_member_reference(
                AstFactory::create_identifier(lexer.slice_and_advance().as_str(), location, lexer.next_id()),
                None,
                lexer.next_id(),
            ))
        }
        _ => {
            lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                "Integer or Reference",
                lexer.slice(),
                lexer.location(),
            ));
            None
        }
    }?;
    let location = location.span(&lexer.last_location());
    Some(AstFactory::create_direct_access(access, index, lexer.next_id(), location))
}

fn parse_literal_number_with_modifier(
    lexer: &mut ParseSession,
    radix: u32,
    is_negative: bool,
) -> Option<AstNode> {
    // we can safely unwrap the number string, since the token has
    // been matched using regular expressions
    let location = lexer.location();
    let token = lexer.slice_and_advance();
    let number_str = token.split('#').next_back().expect("token with '#'");
    let number_str = number_str.replace('_', "");

    // again, the parsed number can be safely unwrapped.
    let value = i128::from_str_radix(number_str.as_str(), radix).expect("valid i128");
    let value = if is_negative { -value } else { value };
    Some(AstNode::new_literal(AstLiteral::new_integer(value), lexer.next_id(), location))
}

fn parse_literal_number(lexer: &mut ParseSession, is_negative: bool) -> Option<AstNode> {
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
        return Some(AstNode::new_literal(
            AstLiteral::new_real(value),
            lexer.next_id(),
            lexer.source_range_factory.create_range(location),
        ));
    } else if lexer.try_consume(KeywordDot) {
        return parse_literal_real(lexer, result, location, is_negative);
    } else if lexer.try_consume(KeywordParensOpen) {
        let start = location.start;
        let multiplier = match result.parse::<u32>() {
            Ok(v) => Some(v),
            Err(e) => {
                lexer.accept_diagnostic(
                    Diagnostic::new(format!("Failed to parse number {result}"))
                        .with_error_code("E011")
                        .with_location(lexer.source_range_factory.create_range(location))
                        .with_internal_error(e.into()),
                );
                None
            }
        }?;
        let element = parse_expression(lexer);
        expect_token!(lexer, KeywordParensClose, None);
        let end = lexer.range().end;
        lexer.advance();
        return Some(AstFactory::create_multiplied_statement(
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

    Some(AstNode::new_literal(
        AstLiteral::new_integer(value),
        lexer.next_id(),
        lexer.source_range_factory.create_range(location),
    ))
}

/// Parses a literal integer without considering Signs or the Possibility of a Floating Point/ Exponent
pub fn parse_strict_literal_integer(lexer: &mut ParseSession) -> Option<AstNode> {
    //correct the location if we just parsed a minus before
    let location = lexer.location();
    let result = lexer.slice_and_advance();
    // parsed number value can be safely unwrapped
    let result = result.replace('_', "");
    if result.to_lowercase().contains('e') {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "Integer",
            &format!("Exponent value: {result}"),
            location,
        ));
        None
    } else {
        let value = result.parse::<i128>().expect("valid i128");
        Some(AstNode::new_literal(AstLiteral::new_integer(value), lexer.next_id(), location))
    }
}

fn parse_number<F: FromStr>(lexer: &mut ParseSession, text: &str, location: &SourceLocation) -> Option<F> {
    match text.parse::<F>() {
        Ok(v) => Some(v),
        Err(_) => {
            lexer.accept_diagnostic(
                Diagnostic::new(format!("Failed to parse number {text}"))
                    .with_error_code("E011")
                    .with_location(location),
            );
            None
        }
    }
}

fn parse_date_from_string(
    lexer: &mut ParseSession,
    text: &str,
    location: SourceLocation,
    id: AstId,
) -> Option<AstNode> {
    let mut segments = text.split('-');

    //we can safely expect 3 numbers
    let year = segments
        .next()
        .map(|s| parse_number::<i32>(lexer, s, &location))
        .expect("year-segment - tokenizer broken?")?;
    let month = segments
        .next()
        .map(|s| parse_number::<u32>(lexer, s, &location))
        .expect("month-segment - tokenizer broken?")?;
    let day = segments
        .next()
        .map(|s| parse_number::<u32>(lexer, s, &location))
        .expect("day-segment - tokenizer broken?")?;

    Some(AstNode::new_literal(AstLiteral::new_date(year, month, day), id, location))
}

fn parse_literal_date_and_time(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let last_minus_location = slice.rfind('-').expect("unexpected date-and-time syntax");

    let (_, date_and_time) = slice.split_at(hash_location + 1); //get rid of the prefix
    let (date, time) = date_and_time.split_at(last_minus_location - hash_location);

    //we can safely expect 3 numbers
    let mut segments = date.split('-');
    let msg = "unexpected date-and-time syntax";
    let year = parse_number::<i32>(lexer, segments.next().expect(msg), &location)?;
    let month = parse_number::<u32>(lexer, segments.next().expect(msg), &location)?;
    let day = parse_number::<u32>(lexer, segments.next().expect(msg), &location)?;

    //we can safely expect 3 numbers
    let mut segments = time.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(lexer, &mut segments, &location)?;

    Some(AstNode::new_literal(
        AstLiteral::new_date_and_time(year, month, day, hour, min, sec, nano),
        lexer.next_id(),
        location,
    ))
}

fn parse_literal_date(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let next_id = lexer.next_id();
    parse_date_from_string(lexer, slice, location, next_id)
}

fn parse_literal_time_of_day(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    //get rid of TOD# or TIME_OF_DAY#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let mut segments = slice.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(lexer, &mut segments, &location)?;

    Some(AstNode::new_literal(AstLiteral::new_time_of_day(hour, min, sec, nano), lexer.next_id(), location))
}

fn parse_time_of_day(
    lexer: &mut ParseSession,
    time: &mut Split<char>,
    location: &SourceLocation,
) -> Option<(u32, u32, u32, u32)> {
    let hour = parse_number::<u32>(lexer, time.next().expect("expected hours"), location)?;
    let min = parse_number::<u32>(lexer, time.next().expect("expected minutes"), location)?;

    // doesn't necessarily have to have seconds, e.g [12:00] is also valid
    let sec = match time.next() {
        Some(v) => parse_number::<f64>(lexer, v, location)?,
        None => 0.0,
    };

    let nano = (sec.fract() * 1e+9_f64).round() as u32;

    Some((hour, min, sec.floor() as u32, nano))
}

fn parse_literal_time(lexer: &mut ParseSession) -> Option<AstNode> {
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
            match char {
                None => {
                    lexer.accept_diagnostic(
                        Diagnostic::new("Invalid TIME Literal: Cannot parse segment.")
                            .with_error_code("E010")
                            .with_location(location),
                    );
                    return None;
                }
                Some((index, _)) => parse_number::<f64>(lexer, &slice[start..index], &location)?,
            }
        };

        //expect a unit
        let unit = {
            let start = match char {
                Some((index, _)) => index,
                None => {
                    lexer.accept_diagnostic(
                        Diagnostic::new("Invalid TIME Literal: Missing unit (d|h|m|s|ms|us|ns)")
                            .with_error_code("E010")
                            .with_location(location),
                    );
                    return None;
                }
            };

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
                lexer.accept_diagnostic(
                    Diagnostic::new("Invalid TIME Literal: segments out of order, use d-h-m-s-ms")
                        .with_error_code("E010")
                        .with_location(location),
                );
                return None;
            }
            prev_pos = position; //remember that we wrote this position

            if values[position].is_some() {
                lexer.accept_diagnostic(
                    Diagnostic::new("Invalid TIME Literal: segments must be unique")
                        .with_error_code("E010")
                        .with_location(location),
                );
                return None;
            }
            values[position] = Some(number); //store the number
        } else {
            lexer.accept_diagnostic(
                Diagnostic::new(format!("Invalid TIME Literal: illegal unit '{unit}'"))
                    .with_error_code("E010")
                    .with_location(location),
            );
            return None;
        }
    }

    Some(AstNode::new_literal(
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

/// Errors produced by [`handle_special_chars`] for invalid `$`-escape sequences.
#[derive(Debug, PartialEq, Clone)]
enum EscapeError {
    /// `$` at the very end of the string — nothing follows it.
    ///
    /// **Note**: this variant is currently unreachable from [`parse_literal_string`].
    /// The lexer regex treats `$.` (dollar + any character) as an atomic unit, so a
    /// `$` immediately before the closing delimiter is consumed as the `$'`/`$"` quote-escape,
    /// leaving the string unterminated — a lexer-level E007 rather than E124.
    /// Detecting this case as E124 requires the lexer to distinguish mid-string `$'`
    /// (quote-escape) from trailing `$` + closing delimiter, which needs a separate
    /// token variant and is tracked as a future enhancement.
    TrailingDollar,
    /// `$X` where `X` is not a recognised named-escape character and not a hex digit.
    UnrecognizedEscape(char),
    /// `$` followed by hex digit(s) but not enough to form a complete escape
    /// (STRING needs 2, WSTRING needs 4).
    IncompleteHexEscape,
}

/// Process all `$`-escape sequences in a string literal (content between quotes) in a
/// single pass.  Returns the decoded string and a list of any invalid escapes found.
/// Invalid escapes are kept as-is in the decoded string for error-recovery purposes.
fn handle_special_chars(string: &str, is_wide: bool) -> (String, Vec<EscapeError>) {
    let hex_digits_per_unit = if is_wide { 4 } else { 2 };

    let mut result = String::with_capacity(string.len());
    let mut errors: Vec<EscapeError> = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] != '$' {
            result.push(chars[i]);
            i += 1;
            continue;
        }

        // We have a '$' — check what follows.
        if i + 1 >= len {
            errors.push(EscapeError::TrailingDollar);
            result.push('$');
            i += 1;
            continue;
        }

        let next = chars[i + 1];

        // Named escapes
        match next {
            'l' | 'L' | 'n' | 'N' => {
                result.push('\n');
                i += 2;
                continue;
            }
            'p' | 'P' => {
                result.push('\x0C');
                i += 2;
                continue;
            }
            'r' | 'R' => {
                result.push('\r');
                i += 2;
                continue;
            }
            't' | 'T' => {
                result.push('\t');
                i += 2;
                continue;
            }
            '$' => {
                result.push('$');
                i += 2;
                continue;
            }
            '\'' | '"' => {
                result.push(next);
                i += 2;
                continue;
            }
            _ => {}
        }

        // Try to consume consecutive hex escape sequences ($XX or $XXXX).
        if next.is_ascii_hexdigit() {
            if is_wide {
                let mut hex_units: Vec<u16> = Vec::new();
                while i < len && chars[i] == '$' {
                    if i + 1 + hex_digits_per_unit > len {
                        break;
                    }
                    let hex_str: String = chars[i + 1..i + 1 + hex_digits_per_unit].iter().collect();
                    if hex_str.len() == hex_digits_per_unit && hex_str.chars().all(|c| c.is_ascii_hexdigit())
                    {
                        hex_units.push(u16::from_str_radix(&hex_str, 16).unwrap_or_default());
                        i += 1 + hex_digits_per_unit;
                    } else {
                        break;
                    }
                }
                if !hex_units.is_empty() {
                    result.push_str(&String::from_utf16_lossy(&hex_units));
                    continue;
                }
            } else {
                let mut hex_bytes: Vec<u8> = Vec::new();
                while i < len && chars[i] == '$' {
                    if i + 1 + hex_digits_per_unit > len {
                        break;
                    }
                    let hex_str: String = chars[i + 1..i + 1 + hex_digits_per_unit].iter().collect();
                    if hex_str.len() == hex_digits_per_unit && hex_str.chars().all(|c| c.is_ascii_hexdigit())
                    {
                        hex_bytes.push(u8::from_str_radix(&hex_str, 16).unwrap_or_default());
                        i += 1 + hex_digits_per_unit;
                    } else {
                        break;
                    }
                }
                if !hex_bytes.is_empty() {
                    result.push_str(&String::from_utf8_lossy(&hex_bytes));
                    continue;
                }
            }
            // next was a hex digit but we couldn't form a complete escape.
            errors.push(EscapeError::IncompleteHexEscape);
            result.push('$');
            i += 1;
            continue;
        }

        // Unrecognized escape — not a named escape and not a hex digit.
        errors.push(EscapeError::UnrecognizedEscape(next));
        result.push('$');
        i += 1;
    }

    (result, errors)
}

fn parse_literal_string(lexer: &mut ParseSession, is_wide: bool) -> Option<AstNode> {
    let result = lexer.slice();
    let location = lexer.location();

    let (processed, errors) = handle_special_chars(&trim_quotes(result), is_wide);

    for error in &errors {
        let message = match error {
            EscapeError::TrailingDollar => {
                "Invalid escape sequence in string literal: trailing '$' has nothing to escape".to_string()
            }
            EscapeError::UnrecognizedEscape(c) => {
                format!("Invalid escape sequence in string literal: '${c}' is not a valid escape sequence")
            }
            EscapeError::IncompleteHexEscape => {
                let n = if is_wide { 4 } else { 2 };
                format!(
                    "Invalid escape sequence in string literal: incomplete hex escape, expected {n} hex digits after '$'"
                )
            }
        };
        lexer.accept_diagnostic(
            Diagnostic::new(message).with_error_code("E124").with_location(location.clone()),
        );
    }

    let string_literal =
        Some(AstNode::new_literal(AstLiteral::new_string(processed, is_wide), lexer.next_id(), location));
    lexer.advance();
    string_literal
}

fn parse_literal_real(
    lexer: &mut ParseSession,
    integer: String,
    integer_range: Range<usize>,
    is_negative: bool,
) -> Option<AstNode> {
    if lexer.token == LiteralInteger {
        let start = integer_range.start;
        let end = lexer.range().end;
        let fractional = lexer.slice_and_advance();
        let value = format!("{}{}.{}", if is_negative { "-" } else { "" }, integer, fractional);
        let new_location = lexer.source_range_factory.create_range(start..end);

        Some(AstNode::new_literal(AstLiteral::new_real(value), lexer.next_id(), new_location))
    } else {
        lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
            "LiteralInteger or LiteralExponent",
            lexer.slice(),
            lexer.location(),
        ));
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::expressions_parser::{handle_special_chars, EscapeError};

    // ── helpers ────────────────────────────────────────────────────────────────

    fn ok(s: &str, wide: bool) -> String {
        let (result, errors) = handle_special_chars(s, wide);
        assert!(errors.is_empty(), "unexpected errors for {s:?}: {errors:?}");
        result
    }

    fn errs(s: &str, wide: bool) -> Vec<EscapeError> {
        handle_special_chars(s, wide).1
    }

    // ── existing regression tests (updated to use the new return type) ─────────

    #[test]
    fn replace_all_test() {
        let string = "a $l$L b $n$N test $p$P c $r$R d $t$T$$ $'quote$' $57 💖 $F0$9F$92$96";
        let expected = "a \n\n b \n\n test \x0C\x0C c \r\r d \t\t$ 'quote' W 💖 💖";

        let w_string = r#"a $l$L b $n$N test $p$P c $r$R d $t$T$$ $"double$" $0077 💖 $D83D$DC96"#;
        let w_expected = "a \n\n b \n\n test \x0C\x0C c \r\r d \t\t$ \"double\" w 💖 💖";

        assert_eq!(ok(w_string, true), w_expected);
        assert_eq!(ok(string, false), expected);
    }

    #[test]
    fn should_not_replace_test() {
        // $0043 in STRING: $00 → NUL, then '4' and '3' as literals; $" produces '"'
        let string = r#"$0043 $"no replace$""#;
        let expected = "\u{0}43 \"no replace\"";
        // $57 in WSTRING: '5' is a hex digit but only 4-digit escapes are valid in WSTRING,
        // and "57 $" is not 4 hex digits → incomplete hex error; $' produces '\''
        let w_string = r#"$57 $'no replace$'"#;
        let w_expected = "$57 'no replace'";

        assert_eq!(handle_special_chars(w_string, true).0, w_expected);
        assert_eq!(handle_special_chars(string, false).0, expected);
    }

    #[test]
    fn dollar_dollar_followed_by_hex_should_not_be_double_processed() {
        // $$ → literal '$'; the digits that follow must NOT be re-interpreted as a hex escape
        assert_eq!(ok("Price: $$100", false), "Price: $100");
        assert_eq!(ok("$$41", false), "$41");
        assert_eq!(ok("$$1002", true), "$1002");
    }

    // ── happy-path unit tests ──────────────────────────────────────────────────

    #[test]
    fn named_escapes_string() {
        assert_eq!(ok("$N", false), "\n");
        assert_eq!(ok("$n", false), "\n");
        assert_eq!(ok("$L", false), "\n"); // line-feed alias
        assert_eq!(ok("$l", false), "\n");
        assert_eq!(ok("$R", false), "\r");
        assert_eq!(ok("$r", false), "\r");
        assert_eq!(ok("$T", false), "\t");
        assert_eq!(ok("$t", false), "\t");
        assert_eq!(ok("$P", false), "\x0C");
        assert_eq!(ok("$p", false), "\x0C");
        assert_eq!(ok("$$", false), "$");
        assert_eq!(ok("$'", false), "'");
        assert_eq!(ok("$\"", false), "\""); // double-quote also valid in STRING
    }

    #[test]
    fn named_escapes_wstring() {
        assert_eq!(ok("$N", true), "\n");
        assert_eq!(ok("$n", true), "\n");
        assert_eq!(ok("$L", true), "\n");
        assert_eq!(ok("$l", true), "\n");
        assert_eq!(ok("$R", true), "\r");
        assert_eq!(ok("$r", true), "\r");
        assert_eq!(ok("$T", true), "\t");
        assert_eq!(ok("$t", true), "\t");
        assert_eq!(ok("$P", true), "\x0C");
        assert_eq!(ok("$p", true), "\x0C");
        assert_eq!(ok("$$", true), "$");
        assert_eq!(ok("$\"", true), "\"");
        assert_eq!(ok("$'", true), "'"); // single-quote also valid in WSTRING
    }

    #[test]
    fn hex_escape_string_valid_ascii() {
        assert_eq!(ok("$48$49", false), "HI"); // $48='H', $49='I'
        assert_eq!(ok("$41", false), "A");
        assert_eq!(ok("$00", false), "\u{0}"); // NUL byte
    }

    #[test]
    fn hex_escape_string_valid_multibyte_utf8() {
        // 💖 = U+1F496 = bytes F0 9F 92 96
        assert_eq!(ok("$F0$9F$92$96", false), "💖");
    }

    #[test]
    fn hex_escape_string_invalid_utf8_replaced_with_replacement_char() {
        // Invalid UTF-8 byte sequences are decoded lossily → U+FFFD
        assert_eq!(ok("$80", false), "\u{FFFD}"); // lone continuation byte
        assert_eq!(ok("caf$E9", false), "caf\u{FFFD}"); // 0xE9 without continuation
        assert_eq!(ok("$FF", false), "\u{FFFD}"); // never-valid UTF-8 byte
    }

    #[test]
    fn hex_escape_wstring_valid_bmp() {
        assert_eq!(ok("$0048$0049", true), "HI"); // U+0048='H', U+0049='I'
        assert_eq!(ok("$0077", true), "w"); // U+0077='w'
    }

    #[test]
    fn hex_escape_wstring_valid_surrogate_pair() {
        assert_eq!(ok("$D83D$DE00", true), "😀"); // U+1F600
        assert_eq!(ok("$D83D$DC96", true), "💖"); // U+1F496
    }

    #[test]
    fn hex_escape_wstring_unpaired_surrogates_replaced_with_replacement_char() {
        assert_eq!(ok("$D800", true), "\u{FFFD}"); // high surrogate alone
        assert_eq!(ok("$DE00", true), "\u{FFFD}"); // orphan low surrogate
        assert_eq!(ok("$D83D$0041", true), "\u{FFFD}A"); // high + non-low BMP char
        assert_eq!(ok("$DE00$D83D", true), "\u{FFFD}\u{FFFD}"); // reversed pair
    }

    // ── unhappy-path unit tests ────────────────────────────────────────────────

    #[test]
    fn trailing_dollar_produces_error() {
        assert_eq!(errs("hello$", false), vec![EscapeError::TrailingDollar]);
        assert_eq!(errs("$", false), vec![EscapeError::TrailingDollar]);
        assert_eq!(errs("hello$", true), vec![EscapeError::TrailingDollar]);
        // Trailing '$' after a valid escape is also caught
        assert_eq!(errs("$N$", false), vec![EscapeError::TrailingDollar]);
        assert_eq!(errs("$48$", false), vec![EscapeError::TrailingDollar]);
    }

    #[test]
    fn unrecognized_named_escape_produces_error() {
        // $Q: 'Q' is not a named-escape char and not a hex digit
        assert_eq!(errs("$Q", false), vec![EscapeError::UnrecognizedEscape('Q')]);
        assert_eq!(errs("$Q", true), vec![EscapeError::UnrecognizedEscape('Q')]);
        // $Z similarly
        assert_eq!(errs("$Z", false), vec![EscapeError::UnrecognizedEscape('Z')]);
    }

    #[test]
    fn incomplete_hex_escape_produces_error() {
        // STRING needs 2 hex digits: '$A' has only 1
        assert_eq!(errs("$A", false), vec![EscapeError::IncompleteHexEscape]);
        // WSTRING needs 4 hex digits: '$004' has only 3
        assert_eq!(errs("$004", true), vec![EscapeError::IncompleteHexEscape]);
        // WSTRING: first char hex but 4-char window contains non-hex
        assert_eq!(errs("$A5z1", true), vec![EscapeError::IncompleteHexEscape]);
    }

    #[test]
    fn multiple_errors_all_reported() {
        // Two invalid escapes in one string — both must appear in the error list
        let errors = errs("$Q hello $", false);
        assert_eq!(errors, vec![EscapeError::UnrecognizedEscape('Q'), EscapeError::TrailingDollar]);
    }

    #[test]
    fn error_recovery_keeps_dollar_in_output() {
        // The decoded string keeps the '$' for error-recovery; the error is reported separately
        let (s, e) = handle_special_chars("$Q", false);
        assert_eq!(s, "$Q");
        assert_eq!(e, vec![EscapeError::UnrecognizedEscape('Q')]);

        let (s, e) = handle_special_chars("hello$", false);
        assert_eq!(s, "hello$");
        assert_eq!(e, vec![EscapeError::TrailingDollar]);

        let (s, e) = handle_special_chars("$A", false);
        assert_eq!(s, "$A");
        assert_eq!(e, vec![EscapeError::IncompleteHexEscape]);
    }
}
