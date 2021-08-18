// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{
    ast::*, lexer::ParseSession, lexer::Token::*, parser::parse_any_in_region, Diagnostic,
};
use std::str::FromStr;

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
        let mut expressions = vec![left];
        // this starts an expression list
        while lexer.token == KeywordComma {
            lexer.advance();
            expressions.push(parse_range_statement(lexer));
        }
        return AstStatement::ExpressionList {
            expressions,
            id: lexer.next_id(),
        };
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
    let left = parse_xor_expression(lexer);

    let operator = match lexer.token {
        OperatorOr => Operator::Or,
        _ => return left,
    };

    lexer.advance();

    let right = parse_or_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
}

// XOR
fn parse_xor_expression(lexer: &mut ParseSession) -> AstStatement {
    let left = parse_and_expression(lexer);

    let operator = match lexer.token {
        OperatorXor => Operator::Xor,
        _ => return left,
    };

    lexer.advance();

    let right = parse_xor_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
}

// AND
fn parse_and_expression(lexer: &mut ParseSession) -> AstStatement {
    let left = parse_equality_expression(lexer);

    let operator = match lexer.token {
        OperatorAnd => Operator::And,
        _ => return left,
    };

    lexer.advance();

    let right = parse_and_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
}

//EQUALITY  =, <>
fn parse_equality_expression(lexer: &mut ParseSession) -> AstStatement {
    let left = parse_compare_expression(lexer);
    let operator = match lexer.token {
        OperatorEqual => Operator::Equal,
        OperatorNotEqual => Operator::NotEqual,
        _ => return left,
    };
    lexer.advance();
    let right = parse_equality_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
}

//COMPARE <, >, <=, >=
fn parse_compare_expression(lexer: &mut ParseSession) -> AstStatement {
    let left = parse_additive_expression(lexer);
    let operator = match lexer.token {
        OperatorLess => Operator::Less,
        OperatorGreater => Operator::Greater,
        OperatorLessOrEqual => Operator::LessOrEqual,
        OperatorGreaterOrEqual => Operator::GreaterOrEqual,
        _ => return left,
    };
    lexer.advance();
    let right = parse_compare_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
}

// Addition +, -
fn parse_additive_expression(lexer: &mut ParseSession) -> AstStatement {
    let left = parse_multiplication_expression(lexer);
    let operator = match lexer.token {
        OperatorPlus => Operator::Plus,
        OperatorMinus => Operator::Minus,
        _ => return left,
    };
    lexer.advance();
    let right = parse_additive_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
}

// Multiplication *, /, MOD
fn parse_multiplication_expression(lexer: &mut ParseSession) -> AstStatement {
    let left = parse_unary_expression(lexer);
    let operator = match lexer.token {
        OperatorMultiplication => Operator::Multiplication,
        OperatorDivision => Operator::Division,
        OperatorModulo => Operator::Modulo,
        _ => return left,
    };
    lexer.advance();
    let right = parse_multiplication_expression(lexer);
    AstStatement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        id: lexer.next_id(),
    }
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
        let location = SourceRange::new(start..expression_location.get_end());

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

// Literals, Identifiers, etc.
fn parse_leaf_expression(lexer: &mut ParseSession) -> AstStatement {
    let literal_parse_result = match lexer.token {
        Identifier => parse_qualified_reference(lexer),
        LiteralInteger => parse_literal_number(lexer),
        LiteralIntegerBin => parse_literal_number_with_modifier(lexer, 2),
        LiteralIntegerOct => parse_literal_number_with_modifier(lexer, 8),
        LiteralIntegerHex => parse_literal_number_with_modifier(lexer, 16),
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
        _ => Err(Diagnostic::unexpected_token_found(
            "Literal",
            lexer.slice(),
            lexer.location(),
        )),
    };

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
        location: SourceRange::new(start..end),
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
        reference_elements.push(parse_reference_access(lexer)?);
    }

    let reference = if reference_elements.len() == 1 {
        reference_elements.pop().unwrap()
    } else {
        AstStatement::QualifiedReference {
            elements: reference_elements,
            id: lexer.next_id(),
        }
    };

    if lexer.allow(&KeywordParensOpen) {
        // Call Statement
        let call_statement = if lexer.allow(&KeywordParensClose) {
            AstStatement::CallStatement {
                operator: Box::new(reference),
                parameters: Box::new(None),
                location: SourceRange::new(start..lexer.range().end),
                id: lexer.next_id(),
            }
        } else {
            parse_any_in_region(lexer, vec![KeywordParensClose], |lexer| {
                AstStatement::CallStatement {
                    operator: Box::new(reference),
                    parameters: Box::new(Some(parse_expression_list(lexer))),
                    location: SourceRange::new(start..lexer.range().end),
                    id: lexer.next_id(),
                }
            })
        };
        Ok(call_statement)
    } else {
        Ok(reference)
    }
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
            lexer.expect(KeywordSquareParensClose)?;
            lexer.advance();
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
) -> Result<AstStatement, Diagnostic> {
    // we can safely unwrap the number string, since the token has
    // been matched using regular expressions
    let location = lexer.location();
    let token = lexer.slice_and_advance();
    let number_str = token.split('#').last().unwrap();
    let number_str = number_str.replace("_", "");

    // again, the parsed number can be safely unwrapped.
    let value = i64::from_str_radix(number_str.as_str(), radix).unwrap();

    Ok(AstStatement::LiteralInteger {
        value,
        location,

        id: lexer.next_id(),
    })
}

fn parse_literal_number(lexer: &mut ParseSession) -> Result<AstStatement, Diagnostic> {
    let location = lexer.location();
    let result = lexer.slice_and_advance();
    if lexer.allow(&KeywordDot) {
        return parse_literal_real(lexer, result, location);
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
            location: SourceRange::new(location.get_start()..end),
            id: lexer.next_id(),
        });
    }

    // parsed number value can be safely unwrapped
    let result = result.replace("_", "");
    Ok(AstStatement::LiteralInteger {
        value: result.parse::<i64>().unwrap(),
        location,
        id: lexer.next_id(),
    })
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
        .unwrap()?;
    let month = segments
        .next()
        .map(|s| parse_number::<u32>(s, &location))
        .unwrap()?;
    let day = segments
        .next()
        .map(|s| parse_number::<u32>(s, &location))
        .unwrap()?;

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
    let last_minus_location = slice.rfind('-').unwrap();

    let (_, date_and_time) = slice.split_at(hash_location + 1); //get rid of the prefix
    let (date, time) = date_and_time.split_at(last_minus_location - hash_location);

    //we can safely expect 3 numbers
    let mut segments = date.split('-');
    let year = parse_number::<i32>(segments.next().unwrap(), &location)?;
    let month = parse_number::<u32>(segments.next().unwrap(), &location)?;
    let day = parse_number::<u32>(segments.next().unwrap(), &location)?;

    //we can safely expect 3 numbers
    let mut segments = time.split(':');
    let hour = parse_number::<u32>(segments.next().unwrap(), &location)?;
    let min = parse_number::<u32>(segments.next().unwrap(), &location)?;
    let sec_fraction = parse_number::<f64>(segments.next().unwrap(), &location)?;

    let sec = sec_fraction as u32;
    let milli = ((sec_fraction - sec as f64) * 1000_f64) as u32;

    Ok(AstStatement::LiteralDateAndTime {
        location,
        year,
        month,
        day,
        hour,
        min,
        sec,
        milli,
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
    let hour = parse_number::<u32>(segments.next().unwrap(), &location)?;
    let min = parse_number::<u32>(segments.next().unwrap(), &location)?;

    let sec = parse_number::<f64>(segments.next().unwrap(), &location)?;
    let milli = (sec.fract() * 1000_f64) as u32;
    Ok(AstStatement::LiteralTimeOfDay {
        hour,
        min,
        sec: sec.floor() as u32,
        milli,
        location,
        id: lexer.next_id(),
    })
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
            let start = char.unwrap().0;
            //just eat all the digits
            char = chars.find(|(_, ch)| !ch.is_digit(10) && !ch.eq(&'.'));
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

fn parse_literal_string(
    lexer: &mut ParseSession,
    is_wide: bool,
) -> Result<AstStatement, Diagnostic> {
    let result = lexer.slice();
    let location = lexer.location();
    let string_literal = Ok(AstStatement::LiteralString {
        value: trim_quotes(result),
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
) -> Result<AstStatement, Diagnostic> {
    lexer.expect(LiteralInteger)?;
    let start = integer_range.get_start();
    let fraction_end = lexer.range().end;
    let fractional = lexer.slice_and_advance();

    let (exponent, end) = if lexer.token == LiteralExponent {
        //this spans everything, [integer].[integer]
        (lexer.slice_and_advance(), lexer.range().end)
    } else {
        ("".to_string(), fraction_end)
    };

    let result = format!("{}.{}{}", integer, fractional, exponent);
    let new_location = SourceRange::new(start..end);
    Ok(AstStatement::LiteralReal {
        value: result,
        location: new_location,
        id: lexer.next_id(),
    })
}
