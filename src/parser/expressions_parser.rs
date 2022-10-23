// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{
    ast::*,
    lexer::Token::*,
    lexer::{ParseSession, Token},
    parser::parse_any_in_region,
    Diagnostic,
};
use core::str::Split;
use regex::{Captures, Regex};
use std::str::FromStr;

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
                left = AstStatement::BinaryExpression {
                    operator,
                    left: Box::new(left),
                    right: Box::new(right),
                    id: $lexer.next_id(),
                };
            }
            left
        }
    };
}

/// parse_expression(): returns expression as Statement. if a parse error
/// is encountered, the erroneous part of the AST will consist of an
/// EmptyStatement and a diagnostic will be logged. That case is different from
/// only an EmptyStatement returned, which does not denote an error condition.
pub fn parse_expression(lexer: &mut ParseSession) -> AstStatement {
    if lexer.token == KeywordSemicolon {
        AstStatement::EmptyStatement {
            location: lexer.location(),
            id: lexer.next_id(),
        }
    } else {
        parse_expression_list(lexer)
    }
}

pub fn parse_expression_list(lexer: &mut ParseSession) -> AstStatement {
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
            return AstStatement::ExpressionList {
                expressions,
                id: lexer.next_id(),
            };
        }
    }
    left
}

pub(crate) fn parse_range_statement(lexer: &mut ParseSession) -> AstStatement {
    let start = parse_or_expression(lexer);

    if lexer.token == KeywordDotDot {
        lexer.advance();
        let end = parse_or_expression(lexer);
        return AstStatement::RangeStatement {
            start: Box::new(start),
            end: Box::new(end),
            id: lexer.next_id(),
        };
    }
    start
}

// OR
fn parse_or_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(lexer, parse_xor_expression, OperatorOr,)
}

// XOR
fn parse_xor_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(lexer, parse_and_expression, OperatorXor,)
}

// AND
fn parse_and_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(lexer, parse_equality_expression, OperatorAmp | OperatorAnd,)
}

//EQUALITY  =, <>
fn parse_equality_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(
        lexer,
        parse_compare_expression,
        OperatorEqual | OperatorNotEqual,
    )
}

//COMPARE <, >, <=, >=
fn parse_compare_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(
        lexer,
        parse_additive_expression,
        OperatorLess | OperatorGreater | OperatorLessOrEqual | OperatorGreaterOrEqual,
    )
}

// Addition +, -
fn parse_additive_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(
        lexer,
        parse_multiplication_expression,
        OperatorPlus | OperatorMinus,
    )
}

// Multiplication *, /, MOD
fn parse_multiplication_expression(lexer: &mut ParseSession) -> AstStatement {
    parse_left_associative_expression!(
        lexer,
        parse_exponent_expression,
        OperatorMultiplication | OperatorDivision | OperatorModulo,
    )
}

// Expoent **
fn parse_exponent_expression(lexer: &mut ParseSession) -> AstStatement {
    //This is always parsed as a function call to the EXPT function
    //Parse left
    let mut left = parse_unary_expression(lexer);
    while matches!(lexer.token, OperatorExponent) {
        let start_location = lexer.last_location();
        let op_location = lexer.location();
        lexer.advance();
        let right = parse_unary_expression(lexer);
        left = AstStatement::CallStatement {
            operator: Box::new(AstStatement::Reference {
                name: "EXPT".to_string(),
                location: op_location,
                id: lexer.next_id(),
            }),
            parameters: Box::new(Some(AstStatement::ExpressionList {
                expressions: vec![left, right],
                id: lexer.next_id(),
            })),
            location: (start_location.get_start()..lexer.last_location().get_end()).into(),
            id: lexer.next_id(),
        }
    }
    left
}

// UNARY -x, NOT x
fn parse_unary_expression(lexer: &mut ParseSession) -> AstStatement {
    let operator = match lexer.token {
        OperatorNot => Some(Operator::Not),
        OperatorMinus => Some(Operator::Minus),
        OperatorAmp => Some(Operator::Address),
        _ => None,
    };

    let start = lexer.range().start;
    if let Some(operator) = operator {
        lexer.advance();
        let expression = parse_parenthesized_expression(lexer);
        let expression_location = expression.get_location();
        let location = lexer
            .source_range_factory
            .create_range(start..expression_location.get_end());

        if let (AstStatement::LiteralInteger { value, .. }, Operator::Minus) =
            (&expression, &operator)
        {
            //if this turns out to be a negative number, we want to have a negative literal integer
            //instead of a Unary-Not-Expression
            AstStatement::LiteralInteger {
                value: -value,
                location,
                id: lexer.next_id(),
            }
        } else {
            AstStatement::UnaryExpression {
                operator,
                value: Box::new(expression),
                location,
                id: lexer.next_id(),
            }
        }
    } else {
        parse_parenthesized_expression(lexer)
    }
}

// PARENTHESIZED (...)
fn parse_parenthesized_expression(lexer: &mut ParseSession) -> AstStatement {
    match lexer.token {
        KeywordParensOpen => {
            lexer.advance();
            super::parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
                parse_expression(lexer)
            })
        }
        _ => parse_leaf_expression(lexer),
    }
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
fn parse_leaf_expression(lexer: &mut ParseSession) -> AstStatement {
    //see if there's a cast
    let literal_cast = if lexer.token == TypeCastPrefix {
        let location = lexer.location();
        let mut a = lexer.slice_and_advance();
        a.pop(); //drop last char '#' - the lexer made sure it ends with a '#'
        Some((a, location))
    } else {
        None
    };

    let literal_parse_result = if lexer.allow(&OperatorMinus) {
        //so we've seen a Minus '-', this has to be a number
        match lexer.token {
            LiteralInteger => parse_literal_number(lexer, true),
            LiteralIntegerBin => parse_literal_number_with_modifier(lexer, 2, true),
            LiteralIntegerOct => parse_literal_number_with_modifier(lexer, 8, true),
            LiteralIntegerHex => parse_literal_number_with_modifier(lexer, 16, true),
            _ => Err(Diagnostic::unexpected_token_found(
                "Numeric Literal",
                lexer.slice(),
                lexer.location(),
            )),
        }
    } else {
        // no minus ... so this may be anything
        match lexer.token {
            Identifier => parse_qualified_reference(lexer),
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
            _ => {
                if lexer.closing_keywords.contains(&vec![KeywordParensClose])
                    && matches!(
                        lexer.last_token,
                        KeywordOutputAssignment | KeywordAssignment
                    )
                {
                    // due to closing keyword ')' and last_token '=>' / ':='
                    // we are probably in a call statement missing a parameter assignment 'foo(param := );
                    // optional parameter assignments are allowed, validation should handle any unwanted cases
                    Ok(AstStatement::EmptyStatement {
                        location: lexer.location(),
                        id: lexer.next_id(),
                    })
                } else {
                    Err(Diagnostic::unexpected_token_found(
                        "Literal",
                        lexer.slice(),
                        lexer.location(),
                    ))
                }
            }
        }
    };
    let literal_parse_result = literal_parse_result.and_then(|statement| {
        if let Some((cast, location)) = literal_cast {
            //check if there is something between the literal-type and the literal itself
            if location.get_end() != statement.get_location().get_start() {
                return Err(Diagnostic::syntax_error("Incomplete statement", location));
            }

            Ok(AstStatement::CastStatement {
                id: lexer.next_id(),
                location: (location.get_start()..statement.get_location().get_end()).into(),
                target: Box::new(statement),
                type_name: cast,
            })
        } else {
            Ok(statement)
        }
    });

    match literal_parse_result {
        Ok(statement) => {
            if lexer.token == KeywordAssignment {
                lexer.advance();
                AstStatement::Assignment {
                    left: Box::new(statement),
                    right: Box::new(parse_range_statement(lexer)),
                    id: lexer.next_id(),
                }
            } else if lexer.token == KeywordOutputAssignment {
                lexer.advance();
                AstStatement::OutputAssignment {
                    left: Box::new(statement),
                    right: Box::new(parse_range_statement(lexer)),
                    id: lexer.next_id(),
                }
            } else {
                statement
            }
        }
        Err(diagnostic) => {
            let statement = AstStatement::EmptyStatement {
                location: diagnostic.get_location(),
                id: lexer.next_id(),
            };
            lexer.accept_diagnostic(diagnostic);
            statement
        }
    }
}

fn parse_array_literal(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let start = lexer.range().start;
    lexer.expect(KeywordSquareParensOpen)?;
    lexer.advance();
    let elements = Some(Box::new(parse_expression(lexer)));
    let end = lexer.range().end;
    lexer.expect(KeywordSquareParensClose)?;
    lexer.advance();
    Ok(AstStatement::LiteralArray {
        elements,
        location: lexer.source_range_factory.create_range(start..end),
        id: lexer.next_id(),
    })
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_bool_literal(lexer: &mut ParseSession, value: bool) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    lexer.advance();
    Ok(AstStatement::LiteralBool {
        value,
        location,

        id: lexer.next_id(),
    })
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_null_literal(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    lexer.advance();
    Ok(AstStatement::LiteralNull {
        location,
        id: lexer.next_id(),
    })
}

pub fn parse_qualified_reference(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let start = lexer.range().start;
    let mut reference_elements = vec![parse_reference_access(lexer)?];
    while lexer.allow(&KeywordDot) {
        let segment = match lexer.token {
            //Is this an integer?
            LiteralInteger => {
                let number = parse_strict_literal_integer(lexer)?;
                let location = number.get_location().clone();
                Ok(AstStatement::DirectAccess {
                    access: crate::ast::DirectAccessType::Bit,
                    index: Box::new(number),
                    location,
                    id: lexer.next_id(),
                })
            }
            //Is this a direct access?
            DirectAccess(access) => parse_direct_access(lexer, access),
            _ => parse_reference_access(lexer),
        }?;

        //Is this a direct access?
        reference_elements.push(segment);
    }

    let reference = match &reference_elements[..] {
        [single_element] => single_element.clone(),
        [_elements @ ..] => AstStatement::QualifiedReference {
            elements: reference_elements,
            id: lexer.next_id(),
        },
    };

    if lexer.allow(&KeywordParensOpen) {
        // Call Statement
        let call_statement = if lexer.allow(&KeywordParensClose) {
            AstStatement::CallStatement {
                operator: Box::new(reference),
                parameters: Box::new(None),
                location: lexer
                    .source_range_factory
                    .create_range(start..lexer.range().end),
                id: lexer.next_id(),
            }
        } else {
            parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
                AstStatement::CallStatement {
                    operator: Box::new(reference),
                    parameters: Box::new(Some(parse_expression_list(lexer))),
                    location: lexer
                        .source_range_factory
                        .create_range(start..lexer.range().end),
                    id: lexer.next_id(),
                }
            })
        };
        Ok(call_statement)
    } else {
        Ok(reference)
    }
}

fn parse_direct_access(
    lexer: &mut ParseSession,
    access: DirectAccessType,
) -> Result<AstStatement, Diagnostic> {
    //Consume the direct access
    let location = lexer.location();
    lexer.advance();
    //The next token can either be an integer or an identifier
    let index = match lexer.token {
        LiteralInteger => parse_strict_literal_integer(lexer),
        Identifier => parse_reference_access(lexer),
        _ => Err(Diagnostic::unexpected_token_found(
            "Integer or Reference",
            lexer.slice(),
            lexer.location(),
        )),
    }?;

    let location = (location.get_start()..lexer.last_location().get_end()).into();
    Ok(AstStatement::DirectAccess {
        access,
        index: Box::new(index),
        location,
        id: lexer.next_id(),
    })
}

pub fn parse_reference_access(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    let reference = AstStatement::Reference {
        name: lexer.slice_and_advance(),
        location,
        id: lexer.next_id(),
    };
    parse_access_modifiers(lexer, reference)
}

fn parse_access_modifiers(
    lexer: &mut ParseSession,
    original_reference: AstStatement,
) -> Result<AstStatement, Diagnostic> {
    let mut reference = original_reference;
    //If (while) we hit a dereference, parse and append the dereference to the result
    while lexer.token == KeywordSquareParensOpen || lexer.token == OperatorDeref {
        if lexer.allow(&KeywordSquareParensOpen) {
            let access = parse_expression(lexer);
            lexer.consume_or_report(KeywordSquareParensClose);
            reference = AstStatement::ArrayAccess {
                reference: Box::new(reference),
                access: Box::new(access),
                id: lexer.next_id(),
            };
        } else if lexer.allow(&OperatorDeref) {
            reference = AstStatement::PointerAccess {
                reference: Box::new(reference),
                id: lexer.next_id(),
            }
        }
    }
    Ok(reference)
}

fn parse_literal_number_with_modifier(
    lexer: &mut ParseSession,
    radix: u32,
    is_negative: bool,
) -> Result<AstStatement, Diagnostic> {
    // we can safely unwrap the number string, since the token has
    // been matched using regular expressions
    let location = lexer.location();
    let token = lexer.slice_and_advance();
    let number_str = token.split('#').last().expect("token with '#'");
    let number_str = number_str.replace('_', "");

    // again, the parsed number can be safely unwrapped.
    let value = i128::from_str_radix(number_str.as_str(), radix).expect("valid i128");
    let value = if is_negative { -value } else { value };
    Ok(AstStatement::LiteralInteger {
        value,
        location,

        id: lexer.next_id(),
    })
}

fn parse_literal_number(
    lexer: &mut ParseSession,
    is_negative: bool,
) -> Result<AstStatement, Diagnostic> {
    //correct the location if we just parsed a minus before
    let location = if is_negative {
        (lexer.last_range.start..lexer.location().get_end()).into()
    } else {
        lexer.location()
    };
    let result = lexer.slice_and_advance();
    if result.to_lowercase().contains('e') {
        let result = result.replace('_', "");
        //Treat exponents as reals
        return Ok(AstStatement::LiteralReal {
            value: result,
            location,
            id: lexer.next_id(),
        });
    } else if lexer.allow(&KeywordDot) {
        return parse_literal_real(lexer, result, location, is_negative);
    } else if lexer.allow(&KeywordParensOpen) {
        let multiplier = result
            .parse::<u32>()
            .map_err(|e| Diagnostic::syntax_error(format!("{}", e).as_str(), location.clone()))?;
        let element = parse_expression(lexer);
        lexer.expect(KeywordParensClose)?;
        let end = lexer.range().end;
        lexer.advance();
        return Ok(AstStatement::MultipliedStatement {
            multiplier,
            element: Box::new(element),
            location: lexer
                .source_range_factory
                .create_range(location.get_start()..end),
            id: lexer.next_id(),
        });
    }

    // parsed number value can be safely unwrapped
    let result = result.replace('_', "");

    let value = result.parse::<i128>().expect("valid i128");
    let value = if is_negative { -value } else { value };

    Ok(AstStatement::LiteralInteger {
        value,
        location,
        id: lexer.next_id(),
    })
}

/// Parses a literal integer without considering Signs or the Possibility of a Floating Point/ Exponent
pub fn parse_strict_literal_integer(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    //correct the location if we just parsed a minus before
    let location = lexer.location();
    let result = lexer.slice_and_advance();
    // parsed number value can be safely unwrapped
    let result = result.replace('_', "");
    if result.to_lowercase().contains('e') {
        Err(Diagnostic::unexpected_token_found(
            "Integer",
            &format!("Exponent value: {}", result),
            location,
        ))
    } else {
        let value = result.parse::<i128>().expect("valid i128");
        Ok(AstStatement::LiteralInteger {
            value,
            location,
            id: lexer.next_id(),
        })
    }
}

fn parse_number<F: FromStr>(text: &str, location: &SourceRange) -> Result<F, Diagnostic> {
    text.parse::<F>().map_err(|_| {
        Diagnostic::syntax_error(
            format!("Failed parsing number {}", text).as_str(),
            location.clone(),
        )
    })
}

fn parse_date_from_string(
    text: &str,
    location: SourceRange,
    id: AstId,
) -> Result<AstStatement, Diagnostic> {
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

    Ok(AstStatement::LiteralDate {
        year,
        month,
        day,
        location,
        id,
    })
}

fn parse_literal_date_and_time(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let last_minus_location = slice.rfind('-').expect("unexpected date-and-time syntax");

    let (_, date_and_time) = slice.split_at(hash_location + 1); //get rid of the prefix
    let (date, time) = date_and_time.split_at(last_minus_location - hash_location);

    //we can safely expect 3 numbers
    let mut segments = date.split('-');
    let year = parse_number::<i32>(
        segments.next().expect("unexpected date-and-time syntax"),
        &location,
    )?;
    let month = parse_number::<u32>(
        segments.next().expect("unexpected date-and-time syntax"),
        &location,
    )?;
    let day = parse_number::<u32>(
        segments.next().expect("unexpected date-and-time syntax"),
        &location,
    )?;

    //we can safely expect 3 numbers
    let mut segments = time.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(&mut segments, &location)?;

    Ok(AstStatement::LiteralDateAndTime {
        location,
        year,
        month,
        day,
        hour,
        min,
        sec,
        nano,
        id: lexer.next_id(),
    })
}

fn parse_literal_date(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    parse_date_from_string(slice, location, lexer.next_id())
}

fn parse_literal_time_of_day(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    //get rid of TOD# or TIME_OF_DAY#
    let slice = lexer.slice_and_advance();
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let mut segments = slice.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(&mut segments, &location)?;

    Ok(AstStatement::LiteralTimeOfDay {
        hour,
        min,
        sec,
        nano,
        location,
        id: lexer.next_id(),
    })
}

fn parse_time_of_day(
    time: &mut Split<char>,
    location: &SourceRange,
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

fn parse_literal_time(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
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
                Diagnostic::syntax_error(
                    "Invalid TIME Literal: Cannot parse segment.",
                    location.clone(),
                )
            })
            .and_then(|(index, _)| parse_number::<f64>(&slice[start..index], &location))?
        };

        //expect a unit
        let unit = {
            let start = char.map(|(index, _)| index).ok_or_else(|| {
                Diagnostic::syntax_error(
                    "Invalid TIME Literal: Missing unit (d|h|m|s|ms|us|ns)",
                    location.clone(),
                )
            })?;

            //just eat all the characters
            char = chars.find(|(_, ch)| !ch.is_ascii_alphabetic());
            &slice[start..char.unwrap_or((slice.len(), ' ')).0]
        };

        //now assign the number to the according segment of the value's array
        let position = match unit {
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
                return Err(Diagnostic::syntax_error(
                    "Invalid TIME Literal: segments out of order, use d-h-m-s-ms",
                    location,
                ));
            }
            prev_pos = position; //remember that we wrote this position

            if values[position].is_some() {
                return Err(Diagnostic::syntax_error(
                    "Invalid TIME Literal: segments must be unique",
                    location,
                ));
            }
            values[position] = Some(number); //store the number
        } else {
            return Err(Diagnostic::syntax_error(
                format!("Invalid TIME Literal: illegal unit '{}'", unit).as_str(),
                location,
            ));
        }
    }

    Ok(AstStatement::LiteralTime {
        day: values[POS_D].unwrap_or_default(),
        hour: values[POS_H].unwrap_or_default(),
        min: values[POS_M].unwrap_or_default(),
        sec: values[POS_S].unwrap_or_default(),
        milli: values[POS_MS].unwrap_or_default(),
        micro: values[POS_US].unwrap_or_default(),
        nano: values[POS_NS].map(|it| it as u32).unwrap_or(0u32),
        negative: is_negative,
        location,
        id: lexer.next_id(),
    })
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
                let hex_vals: Vec<u16> = hex_vals
                    .iter()
                    .map(|it| u16::from_str_radix(*it, 16).unwrap_or_default())
                    .collect();
                String::from_utf16_lossy(&hex_vals)
            } else {
                let hex_vals: Vec<u8> = hex_vals
                    .iter()
                    .map(|it| u8::from_str_radix(*it, 16).unwrap_or_default())
                    .collect();
                String::from_utf8_lossy(&hex_vals).to_string()
            };
            res
        })
        .into()
}

fn parse_literal_string(
    lexer: &mut ParseSession,
    is_wide: bool,
) -> Result<AstStatement, Diagnostic> {
    let result = lexer.slice();
    let location = lexer.location();
    let string_literal = Ok(AstStatement::LiteralString {
        value: handle_special_chars(&trim_quotes(result), is_wide),
        is_wide,
        location,
        id: lexer.next_id(),
    });
    lexer.advance();
    string_literal
}

fn parse_literal_real(
    lexer: &mut ParseSession,
    integer: String,
    integer_range: SourceRange,
    is_negative: bool,
) -> Result<AstStatement, Diagnostic> {
    if lexer.token == LiteralInteger {
        let start = integer_range.get_start();
        let end = lexer.range().end;
        let fractional = lexer.slice_and_advance();
        let result = format!(
            "{}{}.{}",
            if is_negative { "-" } else { "" },
            integer,
            fractional
        );
        let new_location = lexer.source_range_factory.create_range(start..end);
        Ok(AstStatement::LiteralReal {
            value: result,
            location: new_location,
            id: lexer.next_id(),
        })
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
