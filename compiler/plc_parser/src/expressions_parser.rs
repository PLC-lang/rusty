//! Parser helpers extracted from the crate root.
//!
//! This module contains functionality that was moved out of `lib.rs` to keep the
//! expression parsing logic isolated in its own file.

use core::str::Split;
use plc_ast::{
    ast::AstNode,
    literals::{AstLiteral, Time},
};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_lexer::ParseSession;
use plc_source::source_location::SourceLocation;
use std::str::FromStr;

/// Parses an integer conversion input using the same radix rules as integer literals.
pub fn parse_integer(input: &str) -> Option<i128> {
    let (radix, rest) = match input.as_bytes() {
        [b'1', b'6', b'#', ..] => (16, &input[3..]),
        [b'0', b'x', ..] | [b'0', b'X', ..] => (16, &input[2..]),
        [b'8', b'#', ..] => (8, &input[2..]),
        [b'2', b'#', ..] | [b'0', b'b', ..] | [b'0', b'B', ..] => (2, &input[2..]),
        _ => (10, input),
    };
    let rest = rest.replace('_', "");

    if radix != 10 {
        return i128::from_str_radix(&rest, radix).ok();
    }

    let (integer, fraction) = match rest.split_once('.') {
        Some((integer, fraction)) if !fraction.is_empty() => (integer, fraction),
        Some(_) => return None,
        None => (rest.as_str(), ""),
    };
    if !fraction.is_empty() && !fraction.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    integer.parse().ok()
}

/// Parses a boolean conversion input using Structured Text boolean literals and numeric forms.
pub fn parse_bool(input: &str) -> Option<bool> {
    match input {
        "1" => Some(true),
        "0" => Some(false),
        input if input.eq_ignore_ascii_case("TRUE") => Some(true),
        input if input.eq_ignore_ascii_case("FALSE") => Some(false),
        _ => None,
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
    id: usize,
    is_long: bool,
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

    Some(AstNode::new_literal(AstLiteral::new_date_with_long_flag(year, month, day, is_long), id, location))
}

pub fn parse_literal_date_and_time(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance().to_string();
    let hash_location = slice.find('#').unwrap_or_default();
    let is_long = slice[..hash_location].starts_with('L') || slice[..hash_location].starts_with('l');
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

    let literal = if is_long {
        AstLiteral::new_long_date_and_time(year, month, day, hour, min, sec, nano)
    } else {
        AstLiteral::new_date_and_time(year, month, day, hour, min, sec, nano)
    };

    Some(AstNode::new_literal(literal, lexer.next_id(), location))
}

pub fn parse_literal_date(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    //get rid of D# or DATE#
    let slice = lexer.slice_and_advance().to_string();
    let hash_location = slice.find('#').unwrap_or_default();
    let is_long = slice[..hash_location].starts_with('L') || slice[..hash_location].starts_with('l');
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let id = lexer.next_id();
    parse_date_from_string(lexer, slice, location, id, is_long)
}

pub fn parse_literal_time_of_day(lexer: &mut ParseSession) -> Option<AstNode> {
    let location = lexer.location();
    //get rid of TOD# or TIME_OF_DAY#
    let slice = lexer.slice_and_advance().to_string();
    let hash_location = slice.find('#').unwrap_or_default();
    let is_long = slice[..hash_location].starts_with('L') || slice[..hash_location].starts_with('l');
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

    let mut segments = slice.split(':');
    let (hour, min, sec, nano) = parse_time_of_day(lexer, &mut segments, &location)?;

    Some(AstNode::new_literal(
        AstLiteral::new_time_of_day_with_long_flag(hour, min, sec, nano, is_long),
        lexer.next_id(),
        location,
    ))
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

pub fn parse_literal_time(lexer: &mut ParseSession) -> Option<AstNode> {
    const POS_D: usize = 0;
    const POS_H: usize = 1;
    const POS_M: usize = 2;
    const POS_S: usize = 3;
    const POS_MS: usize = 4;
    const POS_US: usize = 5;
    const POS_NS: usize = 6;
    let location = lexer.location();
    //get rid of T# or TIME#
    let slice = lexer.slice_and_advance().to_string();
    let hash_location = slice.find('#').unwrap_or_default();
    let is_long = slice[..hash_location].starts_with('L') || slice[..hash_location].starts_with('l');
    let (_, slice) = slice.split_at(hash_location + 1); //get rid of the prefix

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
            prev_pos = position; //remember that we wrote position

            if values[position].is_some() {
                lexer.accept_diagnostic(
                    Diagnostic::new("Invalid TIME Literal: segments must be unique")
                        .with_error_code("E010")
                        .with_location(location),
                );
                return None;
            }
            values[position] = Some(number);
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
            is_long,
        }),
        lexer.next_id(),
        location,
    ))
}
