// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::ast::*;
use crate::expect;
use crate::lexer::Token::*;
use std::str::FromStr;

use super::allow;
use super::RustyLexer;
use super::{slice_and_advance, unexpected_token};

pub fn parse_primary_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    parse_expression_list(lexer)
}

pub fn parse_expression_list(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_range_statement(lexer);
    if lexer.token == KeywordComma {
        let mut expressions = vec![left?];
        // this starts an expression list
        while lexer.token == KeywordComma {
            lexer.advance();
            expressions.push(parse_range_statement(lexer)?);
        }
        return Ok(Statement::ExpressionList { expressions });
    }
    left
}

pub(crate) fn parse_range_statement(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = parse_or_expression(lexer)?;

    if lexer.token == KeywordDotDot {
        lexer.advance();
        let end = parse_or_expression(lexer)?;
        return Ok(Statement::RangeStatement {
            start: Box::new(start),
            end: Box::new(end),
        });
    }
    Ok(start)
}

// OR
fn parse_or_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_xor_expression(lexer)?;

    let operator = match lexer.token {
        OperatorOr => Operator::Or,
        _ => return Ok(left),
    };

    lexer.advance();

    let right = parse_or_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

// XOR
fn parse_xor_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_and_expression(lexer)?;

    let operator = match lexer.token {
        OperatorXor => Operator::Xor,
        _ => return Ok(left),
    };

    lexer.advance();

    let right = parse_xor_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

// AND
fn parse_and_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_equality_expression(lexer)?;

    let operator = match lexer.token {
        OperatorAnd => Operator::And,
        _ => return Ok(left),
    };

    lexer.advance();

    let right = parse_and_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

//EQUALITY  =, <>
fn parse_equality_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_compare_expression(lexer)?;
    let operator = match lexer.token {
        OperatorEqual => Operator::Equal,
        OperatorNotEqual => Operator::NotEqual,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_equality_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

//COMPARE <, >, <=, >=
fn parse_compare_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_additive_expression(lexer)?;
    let operator = match lexer.token {
        OperatorLess => Operator::Less,
        OperatorGreater => Operator::Greater,
        OperatorLessOrEqual => Operator::LessOrEqual,
        OperatorGreaterOrEqual => Operator::GreaterOrEqual,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_compare_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

// Addition +, -
fn parse_additive_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_multiplication_expression(lexer)?;
    let operator = match lexer.token {
        OperatorPlus => Operator::Plus,
        OperatorMinus => Operator::Minus,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_additive_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}

// Multiplication *, /, MOD
fn parse_multiplication_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let left = parse_unary_expression(lexer)?;
    let operator = match lexer.token {
        OperatorMultiplication => Operator::Multiplication,
        OperatorDivision => Operator::Division,
        OperatorModulo => Operator::Modulo,
        _ => return Ok(left),
    };
    lexer.advance();
    let right = parse_multiplication_expression(lexer)?;
    Ok(Statement::BinaryExpression {
        operator,
        left: Box::new(left),
        right: Box::new(right),
    })
}
// UNARY -x, NOT x
fn parse_unary_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let operator = match lexer.token {
        OperatorNot => Some(Operator::Not),
        OperatorMinus => Some(Operator::Minus),
        _ => None,
    };

    let start = lexer.range().start;
    if let Some(operator) = operator {
        lexer.advance();
        let expression = parse_parenthesized_expression(lexer)?;
        let expression_location = expression.get_location();
        let location = SourceRange::new(
            expression_location.get_file_path(),
            start..expression_location.get_end(),
        );
        Ok(Statement::UnaryExpression {
            operator,
            value: Box::new(expression),
            location,
        })
    } else {
        parse_parenthesized_expression(lexer)
    }
}

// PARENTHESIZED (...)
fn parse_parenthesized_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    match lexer.token {
        KeywordParensOpen => {
            lexer.advance();
            let result = parse_primary_expression(lexer);
            expect!(KeywordParensClose, lexer);
            lexer.advance();
            result
        }
        _ => parse_leaf_expression(lexer),
    }
}

// Literals, Identifiers, etc.
fn parse_leaf_expression(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let current = match lexer.token {
        Identifier => parse_qualified_reference(lexer),
        LiteralInteger => parse_literal_number(lexer),
        LiteralDate => parse_literal_date(lexer),
        LiteralTimeOfDay => parse_literal_time_of_day(lexer),
        LiteralTime => parse_literal_time(lexer),
        LiteralDateAndTime => parse_literal_date_and_time(lexer),
        LiteralString => parse_literal_string(lexer),
        LiteralTrue => parse_bool_literal(lexer, true),
        LiteralFalse => parse_bool_literal(lexer, false),
        KeywordSquareParensOpen => parse_array_literal(lexer),
        _ => Err(unexpected_token(lexer)),
    };

    if current.is_ok() && lexer.token == KeywordAssignment {
        lexer.advance();
        return Ok(Statement::Assignment {
            left: Box::new(current?),
            right: Box::new(parse_range_statement(lexer)?),
        });
    } else if current.is_ok() && lexer.token == KeywordOutputAssignment {
        lexer.advance();
        return Ok(Statement::OutputAssignment {
            left: Box::new(current?),
            right: Box::new(parse_range_statement(lexer)?),
        });
    };
    current
}

fn parse_array_literal(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    expect!(KeywordSquareParensOpen, lexer);
    lexer.advance();
    let elements = Some(Box::new(parse_primary_expression(lexer)?));
    let end = lexer.range().end;
    expect!(KeywordSquareParensClose, lexer);
    lexer.advance();
    Ok(Statement::LiteralArray {
        elements,
        location: SourceRange::new(lexer.get_file_path(), start..end),
    })
}

#[allow(clippy::unnecessary_wraps)]
//Allowing the unnecessary wrap here because this method is used along other methods that need to return Results
fn parse_bool_literal(lexer: &mut RustyLexer, value: bool) -> Result<Statement, String> {
    let location = lexer.location();
    lexer.advance();
    Ok(Statement::LiteralBool { value, location })
}

pub fn parse_qualified_reference(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let start = lexer.range().start;
    let mut reference_elements = vec![parse_reference_access(lexer)?];
    while allow(KeywordDot, lexer) {
        reference_elements.push(parse_reference_access(lexer)?);
    }

    let reference = if reference_elements.len() == 1 {
        reference_elements.pop().unwrap()
    } else {
        Statement::QualifiedReference {
            elements: reference_elements,
        }
    };

    if allow(KeywordParensOpen, lexer) {
        let (statement_list, end) = if allow(KeywordParensClose, lexer) {
            (None, lexer.range().end)
        } else {
            let list = parse_expression_list(lexer)?;
            expect!(KeywordParensClose, lexer);
            let end = lexer.range().end;
            lexer.advance();
            (Some(list), end)
        };
        Ok(Statement::CallStatement {
            operator: Box::new(reference),
            parameters: Box::new(statement_list),
            location: SourceRange::new(lexer.get_file_path(), start..end),
        })
    } else {
        Ok(reference)
    }
}

pub fn parse_reference_access(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let location = lexer.location();
    let mut reference = Statement::Reference {
        name: slice_and_advance(lexer),
        location,
    };
    //If (while) we hit a dereference, parse and append the dereference to the result
    while allow(KeywordSquareParensOpen, lexer) {
        let access = parse_primary_expression(lexer)?;
        expect!(KeywordSquareParensClose, lexer);
        lexer.advance();
        reference = Statement::ArrayAccess {
            reference: Box::new(reference),
            access: Box::new(access),
        };
    }
    Ok(reference)
}

fn parse_literal_number(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let location = lexer.location();
    let result = slice_and_advance(lexer);
    if allow(KeywordDot, lexer) {
        return parse_literal_real(lexer, result, location);
    } else if allow(KeywordParensOpen, lexer) {
        let multiplier = result.parse::<u32>().map_err(|e| format!("{}", e))?;
        let element = parse_primary_expression(lexer)?;
        expect!(KeywordParensClose, lexer);
        let end = lexer.range().end;
        lexer.advance();
        return Ok(Statement::MultipliedStatement {
            multiplier,
            element: Box::new(element),
            location: SourceRange::new(location.get_file_path(), location.get_start()..end),
        });
    }

    Ok(Statement::LiteralInteger {
        value: result,
        location,
    })
}

fn parse_number<F: FromStr>(text: &str) -> Result<F, String> {
    text.parse::<F>()
        .map_err(|_| format!("Failed parsing number {}", text))
}

fn parse_date_from_string(text: &str, location: SourceRange) -> Result<Statement, String> {
    let mut segments = text.split('-');

    //we can safely expect 3 numbers
    let year = parse_number::<i32>(segments.next().unwrap())?;
    let month = parse_number::<u32>(segments.next().unwrap())?;
    let day = parse_number::<u32>(segments.next().unwrap())?;

    Ok(Statement::LiteralDate {
        year,
        month,
        day,
        location,
    })
}

fn parse_literal_date_and_time(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = slice_and_advance(lexer);
    let hash_location = slice.find('#').unwrap_or_default();
    let last_minus_location = slice.rfind('-').unwrap();

    let (_, date_and_time) = slice.split_at(hash_location + 1); //get rid of the prefix
    let (date, time) = date_and_time.split_at(last_minus_location - hash_location);

    //we can safely expect 3 numbers
    let mut segments = date.split('-');
    let year = parse_number::<i32>(segments.next().unwrap())?;
    let month = parse_number::<u32>(segments.next().unwrap())?;
    let day = parse_number::<u32>(segments.next().unwrap())?;

    //we can safely expect 3 numbers
    let mut segments = time.split(':');
    let hour = parse_number::<u32>(segments.next().unwrap())?;
    let min = parse_number::<u32>(segments.next().unwrap())?;
    let sec_fraction = parse_number::<f64>(segments.next().unwrap())?;

    let sec = sec_fraction as u32;
    let milli = ((sec_fraction - sec as f64) * 1000_f64) as u32;

    Ok(Statement::LiteralDateAndTime {
        location,
        year,
        month,
        day,
        hour,
        min,
        sec,
        milli,
    })
}

fn parse_literal_date(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = slice_and_advance(lexer);
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    parse_date_from_string(slice, location)
}

fn parse_literal_time_of_day(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let location = lexer.location();
    //get rid of TOD# or TIME_OF_DAY#
    let slice = slice_and_advance(lexer);
    let hash_location = slice.find('#').unwrap_or_default();
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let mut segments = slice.split(':');
    let hour = parse_number::<u32>(segments.next().unwrap())?;
    let min = parse_number::<u32>(segments.next().unwrap())?;

    let sec = parse_number::<f64>(segments.next().unwrap())?;
    let milli = (sec.fract() * 1000_f64) as u32;
    Ok(Statement::LiteralTimeOfDay {
        hour,
        min,
        sec: sec.floor() as u32,
        milli,
        location,
    })
}

fn parse_literal_time(lexer: &mut RustyLexer) -> Result<Statement, String> {
    const POS_D: usize = 0;
    const POS_H: usize = 1;
    const POS_M: usize = 2;
    const POS_S: usize = 3;
    const POS_MS: usize = 4;
    const POS_US: usize = 5;
    const POS_NS: usize = 6;
    let location = lexer.location();
    //get rid of T# or TIME#
    let slice = slice_and_advance(lexer);
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
            char.ok_or_else(|| "Invalid TIME Literal: Cannot parse segment.".to_string())
                .and_then(|(index, _)| parse_number::<f64>(&slice[start..index]))?
        };

        //expect a unit
        let unit = {
            let start = char.map(|(index, _)| index).ok_or_else(|| {
                "Invalid TIME Literal: Missing unit (d|h|m|s|ms|us|ns)".to_string()
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
                return Err(
                    "Invalid TIME Literal: segments out of order, use d-h-m-s-ms".to_string(),
                );
            }
            prev_pos = position; //remember that we wrote this position

            if values[position].is_some() {
                return Err("Invalid TIME Literal: segments must be unique".to_string());
            }
            values[position] = Some(number); //store the number
        } else {
            return Err(format!("Invalid TIME Literal: illegal unit '{}'", unit));
        }
    }

    Ok(Statement::LiteralTime {
        day: values[POS_D].unwrap_or_default(),
        hour: values[POS_H].unwrap_or_default(),
        min: values[POS_M].unwrap_or_default(),
        sec: values[POS_S].unwrap_or_default(),
        milli: values[POS_MS].unwrap_or_default(),
        micro: values[POS_US].unwrap_or_default(),
        nano: values[POS_NS].map(|it| it as u32).unwrap_or(0u32),
        negative: is_negative,
        location,
    })
}

fn trim_quotes(quoted_string: &str) -> String {
    quoted_string[1..quoted_string.len() - 1].to_string()
}

fn parse_literal_string(lexer: &mut RustyLexer) -> Result<Statement, String> {
    let result = lexer.slice();
    let location = lexer.location();
    let string_literal = Ok(Statement::LiteralString {
        value: trim_quotes(result),
        location,
    });
    lexer.advance();
    string_literal
}

fn parse_literal_real(
    lexer: &mut RustyLexer,
    integer: String,
    integer_range: SourceRange,
) -> Result<Statement, String> {
    expect!(LiteralInteger, lexer);
    let start = integer_range.get_start();
    let fraction_end = lexer.range().end;
    let fractional = slice_and_advance(lexer);

    let (exponent, end) = if lexer.token == LiteralExponent {
        //this spans everything, [integer].[integer]
        (slice_and_advance(lexer), lexer.range().end)
    } else {
        ("".to_string(), fraction_end)
    };

    let result = format!("{}.{}{}", integer, fractional, exponent);
    let new_location = SourceRange::new(lexer.get_file_path(), start..end);
    Ok(Statement::LiteralReal {
        value: result,
        location: new_location,
    })
}
