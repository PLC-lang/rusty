//! `STRING_TO_*` conversions for BOOL, the bit/integer widths, and the duration/date/time types.
//!
//! All functions here share the same contract:
//! - Never fault: every input, including malformed or empty ones, returns a value.
//! - Surrounding whitespace (space, tab, CR, LF, FF, VT) is trimmed before any other rule applies.
//! - A rejected input returns the type's zero value.

use crate::string_functions::ptr_to_slice;
use num::NumCast;
use plc_ast::{
    ast::{AstNode, AstStatement},
    literals::AstLiteral,
    provider::IdProvider,
};
use plc_lexer::{lex_with_ids, ParseSession, Token};
use plc_parser::{
    parse_bool, parse_integer, parse_literal_date, parse_literal_date_and_time, parse_literal_time,
    parse_literal_time_of_day,
};
use plc_source::source_location::SourceLocationFactory;
use std::borrow::Cow;

// --------- shared helpers

/// Reads the null-terminated source string and trims the whitespace set shared by every
/// `STRING_TO_*` function: space, tab, CR, LF, FF, VT.
///
/// # Safety
/// `src` must point to a null-terminated buffer, or be null.
unsafe fn trimmed_str<'a>(src: *const u8) -> &'a str {
    let slice = ptr_to_slice(src);
    match std::str::from_utf8(slice) {
        Ok(s) => s.trim_matches(|c: char| matches!(c, ' ' | '\t' | '\r' | '\n' | '\x0C' | '\x0B')),
        Err(_) => "",
    }
}

fn parse_single_statement<'a>(input: &'a str) -> ParseSession<'a> {
    lex_with_ids(input, IdProvider::default(), SourceLocationFactory::internal(input))
}

fn parse_literal(
    input: &str,
    expected: Token,
    parser: fn(&mut ParseSession) -> Option<AstNode>,
) -> Option<AstLiteral> {
    let mut session = parse_single_statement(input);
    if session.token != expected {
        return None;
    }
    let node = parser(&mut session)?;
    if !session.is_end_of_stream() || !session.diagnostics.is_empty() {
        return None;
    }
    match node.get_stmt() {
        AstStatement::Literal(literal) => Some(literal.clone()),
        _ => None,
    }
}

fn parse_prefixed_literal(
    input: &str,
    prefix: &str,
    expected: Token,
    parser: fn(&mut ParseSession) -> Option<AstNode>,
) -> Option<AstLiteral> {
    let source = if input.contains('#') {
        input.to_owned()
    } else {
        format!("{prefix}{}", input.replacen('T', "-", 1))
    };
    parse_literal(&source, expected, parser)
}

fn parse_time_literal(input: &str, prefixes: &[&str]) -> Option<AstLiteral> {
    prefixes
        .iter()
        .find(|prefix| input.get(..prefix.len()).is_some_and(|start| start.eq_ignore_ascii_case(prefix)))?;
    let normalized: String =
        input.replace(',', ".").chars().filter(|character| !character.is_ascii_whitespace()).collect();
    parse_literal(&normalized, Token::LiteralTime, parse_literal_time)
}

fn date_nanos(literal: AstLiteral) -> Option<i64> {
    match literal {
        AstLiteral::Date(date) => date.value().ok(),
        _ => None,
    }
}

fn date_time_nanos(literal: AstLiteral) -> Option<i64> {
    match literal {
        AstLiteral::DateAndTime(date_time) => date_time.value().ok(),
        _ => None,
    }
}

fn time_of_day_nanos(literal: AstLiteral) -> Option<i64> {
    match literal {
        AstLiteral::TimeOfDay(time) => time.value().ok(),
        _ => None,
    }
}

fn duration_nanos(literal: AstLiteral) -> Option<i64> {
    match literal {
        AstLiteral::Time(time) if !time.is_negative() => Some(time.value()),
        _ => None,
    }
}

// --------- BOOL

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_BOOL(src: *const u8) -> bool {
    parse_bool(trimmed_str(src)).unwrap_or_default()
}

// --------- integer / bit-string widths

fn normalize_integer_literal(input: &str) -> Cow<'_, str> {
    if let Some(digits) = input.strip_prefix("0b").or_else(|| input.strip_prefix("0B")) {
        Cow::Owned(format!("2#{digits}"))
    } else if let Some(digits) = input.strip_prefix("0x").or_else(|| input.strip_prefix("0X")) {
        Cow::Owned(format!("16#{digits}"))
    } else {
        Cow::Borrowed(input)
    }
}

/// One radix-prefix + full-consumption parse rule shared by every integer width, so widening a
/// variable never changes the parsed value.
macro_rules! string_to_int_fn {
    ($name:ident, $ty:ty) => {
        /// # Safety
        /// Uses raw pointers, inherently unsafe.
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(src: *const u8) -> $ty {
            let input = normalize_integer_literal(trimmed_str(src));
            parse_integer(&input).and_then(NumCast::from).unwrap_or_default()
        }
    };
}

string_to_int_fn!(STRING_TO_BYTE, u8);
string_to_int_fn!(STRING_TO_WORD, u16);
string_to_int_fn!(STRING_TO_DWORD, u32);
string_to_int_fn!(STRING_TO_DINT, i32);
string_to_int_fn!(STRING_TO_LWORD, u64);
string_to_int_fn!(STRING_TO_LINT, i64);
string_to_int_fn!(STRING_TO_SINT, i8);
string_to_int_fn!(STRING_TO_USINT, u8);
string_to_int_fn!(STRING_TO_INT, i16);
string_to_int_fn!(STRING_TO_UINT, u16);
string_to_int_fn!(STRING_TO_UDINT, u32);
string_to_int_fn!(STRING_TO_ULINT, u64);

// --------- durations (TIME / LTIME)

/// Parses the body after the `T#`/`TIME#` prefix, returning the duration in nanoseconds.
/// Rejects negative durations, out-of-order or duplicate unit segments, and unknown units.
/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_TIME(src: *const u8) -> i64 {
    let s = trimmed_str(src);
    parse_time_literal(s, &["TIME#", "T#"]).and_then(duration_nanos).unwrap_or_default()
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_LTIME(src: *const u8) -> i64 {
    let s = trimmed_str(src);
    parse_time_literal(s, &["LTIME#", "LT#"]).and_then(duration_nanos).unwrap_or_default()
}

// --------- dates and times of day (DATE / DT / TOD)

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DATE(src: *const u8) -> i64 {
    let s = trimmed_str(src);
    parse_prefixed_literal(s, "D#", Token::LiteralDate, parse_literal_date)
        .and_then(date_nanos)
        .unwrap_or_default()
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DT(src: *const u8) -> i64 {
    let s = trimmed_str(src);
    let source = if !s.contains('#') && !s.contains('T') && !s.contains('t') && s.matches('-').count() == 2 {
        format!("DT#{s}-00:00:00")
    } else {
        s.to_owned()
    };
    parse_prefixed_literal(&source, "DT#", Token::LiteralDateAndTime, parse_literal_date_and_time)
        .and_then(date_time_nanos)
        .unwrap_or_default()
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_TOD(src: *const u8) -> i64 {
    let s = trimmed_str(src);
    parse_prefixed_literal(s, "TOD#", Token::LiteralTimeOfDay, parse_literal_time_of_day)
        .and_then(time_of_day_nanos)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NANOS_PER_MILLISECOND: i64 = 1_000_000;
    const NANOS_PER_SECOND: i64 = 1_000 * NANOS_PER_MILLISECOND;

    unsafe fn call_bool(s: &str) -> bool {
        STRING_TO_BOOL(format!("{s}\0").as_ptr())
    }
    unsafe fn call_u8(f: unsafe extern "C" fn(*const u8) -> u8, s: &str) -> u8 {
        f(format!("{s}\0").as_ptr())
    }
    unsafe fn call_u32(f: unsafe extern "C" fn(*const u8) -> u32, s: &str) -> u32 {
        f(format!("{s}\0").as_ptr())
    }
    unsafe fn call_i32(f: unsafe extern "C" fn(*const u8) -> i32, s: &str) -> i32 {
        f(format!("{s}\0").as_ptr())
    }
    unsafe fn call_u64(f: unsafe extern "C" fn(*const u8) -> u64, s: &str) -> u64 {
        f(format!("{s}\0").as_ptr())
    }
    unsafe fn call_i16(f: unsafe extern "C" fn(*const u8) -> i16, s: &str) -> i16 {
        f(format!("{s}\0").as_ptr())
    }
    unsafe fn call_i64(f: unsafe extern "C" fn(*const u8) -> i64, s: &str) -> i64 {
        f(format!("{s}\0").as_ptr())
    }

    #[test]
    fn never_faults_on_garbage() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_UDINT, ""), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "\u{1F980}"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "not a date"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "\u{0}"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "🦀"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "!!!"), 0);
            assert!(!call_bool("garbage"));
        }
    }

    #[test]
    fn bool_conversions() {
        unsafe {
            assert!(call_bool("1"));
            assert!(call_bool("TRUE"));
            assert!(call_bool("true"));
            assert!(call_bool("True"));
            assert!(call_bool("  1  "));
            assert!(call_bool("  true  "));
            assert!(!call_bool("0"));
            assert!(!call_bool("FALSE"));
            assert!(!call_bool("false"));
            assert!(!call_bool("TRUEX"));
            assert!(!call_bool("TRUE1"));
            assert!(!call_bool("2"));
            assert!(!call_bool("T"));
            assert!(!call_bool(""));
        }
    }

    #[test]
    fn one_parse_rule_at_every_width() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_UDINT, "12abc"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "12abc"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "12abc"), 0);
            assert_eq!(call_i64(STRING_TO_LINT, "12abc"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "12 34"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "12 34"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "1e3"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "1e3"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "8#19"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "8#19"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "  12  "), 12);
            assert_eq!(call_i32(STRING_TO_DINT, "  12  "), 12);
            assert_eq!(call_u64(STRING_TO_ULINT, "  12  "), 12);
            assert_eq!(call_i64(STRING_TO_LINT, "  12  "), 12);
            assert_eq!(call_u32(STRING_TO_UDINT, "16#FF"), 255);
            assert_eq!(call_u64(STRING_TO_ULINT, "16#FF"), 255);
            assert_eq!(call_i32(STRING_TO_DINT, "2#1111"), 15);
            assert_eq!(call_i64(STRING_TO_LINT, "2#1111"), 15);
            assert_eq!(call_i32(STRING_TO_DINT, "8#77"), 63);
            assert_eq!(call_i64(STRING_TO_LINT, "8#77"), 63);
            assert_eq!(call_i32(STRING_TO_DINT, "16#FF"), 255);
            assert_eq!(call_i64(STRING_TO_LINT, "16#FF"), 255);
            assert_eq!(call_u32(STRING_TO_UDINT, "0b1010"), 10);
            assert_eq!(call_i32(STRING_TO_DINT, "0B1010"), 10);
            assert_eq!(call_u64(STRING_TO_ULINT, "0xFF"), 255);
            assert_eq!(call_i64(STRING_TO_LINT, "0XFF"), 255);
            assert_eq!(call_u32(STRING_TO_UDINT, "1.9"), 1);
            assert_eq!(call_u64(STRING_TO_ULINT, "1.9"), 1);
            assert_eq!(call_i16(STRING_TO_INT, "-1"), -1);
            assert_eq!(call_u8(STRING_TO_BYTE, "-1"), 0);
        }
    }

    #[test]
    fn fractional_durations_parse_correctly() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.5s"), 1_500 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#0.5s"), 500 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#2.75s"), 2_750 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.5h"), 90 * 60 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1,5s"), 1_500 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1.5s"), 1_500 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.0004ms"), 1_000_400);
        }
    }

    #[test]
    fn negative_durations_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#-1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#-1000ms"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#- 1s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#-1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s"), NANOS_PER_SECOND);
        }
    }

    #[test]
    fn durations_without_a_matching_prefix_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, ""), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "abc"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "LTIME#1s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "T#1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#67ms"), 67 * NANOS_PER_MILLISECOND);
        }
    }

    #[test]
    fn impossible_dates_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-02-30"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-13-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-00"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2023-02-29"), 0);
            assert_ne!(call_i64(STRING_TO_DATE, "D#2024-02-29"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01-25:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:60:00"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#25:00:00"), 0);
        }
    }

    #[test]
    fn dates_outside_the_nanosecond_range_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_DATE, "D#0001-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#9999-12-31"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#9999-12-31-23:59:59"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#1970-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#1969-12-31"), -24 * 3_600 * NANOS_PER_SECOND);
            assert_ne!(call_i64(STRING_TO_DATE, "D#2106-02-08"), 0);
            assert_ne!(call_i64(STRING_TO_DT, "DT#2106-02-08-00:00:00"), 0);
        }
    }

    #[test]
    fn unprefixed_iso_dates_and_times_are_accepted() {
        unsafe {
            let d1 = call_i64(STRING_TO_DATE, "2024-01-01");
            let d2 = call_i64(STRING_TO_DATE, "D#2024-01-01");
            assert_eq!(d1, d2);
            assert_ne!(d1, 0);

            assert_eq!(call_i64(STRING_TO_TOD, "12:00:00"), 12 * 3_600 * NANOS_PER_SECOND);
            assert_eq!(
                call_i64(STRING_TO_TOD, "12:00:00.500"),
                12 * 3_600 * NANOS_PER_SECOND + 500 * NANOS_PER_MILLISECOND
            );

            let dt_date_only = call_i64(STRING_TO_DT, "2024-01-01");
            assert_eq!(dt_date_only, d1);

            let dt1 = call_i64(STRING_TO_DT, "2024-01-01-12:00:00");
            let dt2 = call_i64(STRING_TO_DT, "2024-01-01T12:00:00");
            assert_eq!(dt1, dt2);
            assert_ne!(dt1, 0);
        }
    }

    #[test]
    fn whitespace_is_handled_the_same_everywhere() {
        unsafe {
            assert_ne!(call_i64(STRING_TO_DATE, "D#2024-01-01 "), 0);
            assert_ne!(call_i64(STRING_TO_DATE, "D#2024-01-01\r\n"), 0);
            assert_eq!(call_i16(STRING_TO_INT, "\n12"), 12);
            assert_eq!(call_u64(STRING_TO_ULINT, "12 "), 12);
            assert_ne!(call_i64(STRING_TO_TOD, "TOD#12:00:00\t"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h 30m"), 90 * 60 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D# 2024-01-01"), 0);
        }
    }
}
