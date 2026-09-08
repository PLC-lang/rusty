//! `STRING_TO_*` conversions for BOOL, the bit/integer widths, and the duration/date/time types.
//!
//! All functions here share the same contract:
//! - Never fault: every input, including malformed or empty ones, returns a value.
//! - Surrounding whitespace (space, tab, CR, LF, FF, VT) is trimmed before any other rule applies.
//! - The input must be a literal of the target type, prefix included, and nothing else may follow
//!   it. Durations may additionally separate segments with whitespace or `_` and repeat or reorder
//!   them.
//! - A rejected input, or a value the target type cannot hold, returns the type's zero value.
//! - Nothing on this path allocates.

use crate::string_functions::ptr_to_slice;
use num::NumCast;
use plc_literals::{parse_duration, strip_prefix_ignore_ascii_case, trim, Leniency};

// --------- shared helpers

/// Reads the null-terminated source string and trims the surrounding whitespace.
///
/// # Safety
/// `src` must point to a null-terminated buffer, or be null.
unsafe fn trimmed_str<'a>(src: *const u8) -> &'a str {
    match std::str::from_utf8(ptr_to_slice(src)) {
        Ok(s) => trim(s),
        Err(_) => "",
    }
}

/// Strips one of the accepted type prefixes and parses the remaining duration body.
fn duration_after_prefix(input: &str, prefixes: &[&str]) -> Option<plc_literals::Duration> {
    let body = strip_prefix_ignore_ascii_case(input, prefixes)?;
    parse_duration(body, Leniency::RUNTIME).ok()
}

// --------- BOOL

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_BOOL(src: *const u8) -> bool {
    plc_literals::parse_bool(trimmed_str(src)).unwrap_or_default()
}

// --------- integer / bit-string widths

/// One parse rule shared by every integer width, so widening a variable never changes the parsed
/// value: an integer literal in any radix, or a real literal truncated toward zero.
fn parse_integer_input(input: &str) -> Option<i128> {
    plc_literals::parse_integer(input).or_else(|_| plc_literals::parse_real_as_integer(input)).ok()
}

macro_rules! string_to_int_fn {
    ($name:ident, $ty:ty) => {
        /// # Safety
        /// Uses raw pointers, inherently unsafe.
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(src: *const u8) -> $ty {
            parse_integer_input(trimmed_str(src)).and_then(NumCast::from).unwrap_or_default()
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

/// Accepts `T#` and `TIME#` literals; the value is returned in nanoseconds. Negative durations
/// and durations above the `TIME` range yield 0.
/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_TIME(src: *const u8) -> i64 {
    duration_after_prefix(trimmed_str(src), &["TIME#", "T#"])
        .filter(|duration| !duration.negative)
        .and_then(|duration| duration.signed_nanos())
        .unwrap_or_default()
}

/// Accepts `LT#` and `LTIME#` literals; the value is returned in nanoseconds. Negative durations
/// and durations above the `LTIME` range yield 0.
/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_LTIME(src: *const u8) -> i64 {
    duration_after_prefix(trimmed_str(src), &["LTIME#", "LT#"])
        .filter(|duration| !duration.negative)
        .and_then(|duration| duration.signed_nanos())
        .unwrap_or_default()
}

// --------- dates and times of day (DATE / DT / TOD)

/// Accepts `D#` and `DATE#` literals; the value is returned in nanoseconds since the epoch. Dates
/// that do not exist or lie outside the `DATE` range yield 0.
/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DATE(src: *const u8) -> i64 {
    strip_prefix_ignore_ascii_case(trimmed_str(src), &["DATE#", "D#"])
        .and_then(|body| plc_literals::parse_date(body).ok())
        .and_then(|date| date.nanos_since_epoch())
        .unwrap_or_default()
}

/// Accepts `DT#` and `DATE_AND_TIME#` literals; the value is returned in nanoseconds since the
/// epoch. Values that do not exist or lie outside the `DT` range yield 0.
/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DT(src: *const u8) -> i64 {
    strip_prefix_ignore_ascii_case(trimmed_str(src), &["DATE_AND_TIME#", "DT#"])
        .and_then(|body| plc_literals::parse_date_and_time(body).ok())
        .and_then(|date_time| date_time.nanos_since_epoch())
        .unwrap_or_default()
}

/// Accepts `TOD#` and `TIME_OF_DAY#` literals; the value is returned in nanoseconds since
/// midnight. Times that do not exist yield 0.
/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_TOD(src: *const u8) -> i64 {
    strip_prefix_ignore_ascii_case(trimmed_str(src), &["TIME_OF_DAY#", "TOD#"])
        .and_then(|body| plc_literals::parse_time_of_day(body).ok())
        .and_then(|time| time.nanos())
        .and_then(|nanos| i64::try_from(nanos).ok())
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
            assert_eq!(call_u32(STRING_TO_UDINT, "1e3"), 1_000);
            assert_eq!(call_u64(STRING_TO_ULINT, "1e3"), 1_000);
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
            assert_eq!(call_u32(STRING_TO_UDINT, "0b1010"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "0B1010"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "0xFF"), 0);
            assert_eq!(call_i64(STRING_TO_LINT, "0XFF"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "1.9"), 1);
            assert_eq!(call_u64(STRING_TO_ULINT, "1.9"), 1);
            assert_eq!(call_i16(STRING_TO_INT, "-1"), -1);
            assert_eq!(call_u8(STRING_TO_BYTE, "-1"), 0);
        }
    }

    #[test]
    fn fractional_durations_parse_correctly() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.5s"), (1_500) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#0.5s"), (500) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#2.75s"), (2_750) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.5h"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.5h30m"), (120 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1.5s"), 1_500_000_000);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1.0004ms"), 1_000_400);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1500us"), 1_500_000);
            assert_eq!(call_i64(STRING_TO_TIME, "T#999us"), 999_000);
        }
    }

    #[test]
    fn comma_is_not_a_decimal_separator() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#1,5s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1,5s"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:00:00,500"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01-12:00:00,5"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1,5"), 0);
        }
    }

    #[test]
    fn negative_durations_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#-1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#-1000ms"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#- 1s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#-1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s"), (1_000) * NANOS_PER_MILLISECOND);
        }
    }

    #[test]
    fn the_67ms_constant_is_gone() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, ""), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "abc"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "LTIME#1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#67ms"), (67) * NANOS_PER_MILLISECOND);
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
            assert_eq!(call_i64(STRING_TO_DATE, "D#1969-12-31"), -86_400 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2106-02-08"), 4_295_030_400 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#0001-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#9999-12-31"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#1970-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2262-04-11"), 9_223_286_400 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2262-04-12"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#1969-12-31-23:59:59"), -NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2262-04-11-23:47:16.854775807"), i64::MAX);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2262-04-11-23:47:16.854775808"), 0);
        }
    }

    #[test]
    fn unprefixed_forms_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_DATE, "2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "12:00:00.500"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "2024-01-01-12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "2024-01-01T12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "2024-01-01 12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "1000"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "1s"), 0);
        }
    }

    #[test]
    fn prefixes_are_case_insensitive_and_accept_the_long_spelling() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "TIME#1s"), (1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "t#1s"), (1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "time#1S"), (1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1s"), 1_000_000_000);
            assert_eq!(call_i64(STRING_TO_LTIME, "LT#1s"), 1_000_000_000);
            assert_eq!(call_i64(STRING_TO_LTIME, "ltime#1s"), 1_000_000_000);

            let date = call_i64(STRING_TO_DATE, "D#2024-01-01");
            assert_eq!(date, 1_704_067_200 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "DATE#2024-01-01"), date);
            assert_eq!(call_i64(STRING_TO_DATE, "d#2024-01-01"), date);

            let date_time = call_i64(STRING_TO_DT, "DT#2024-01-01-12:00:00");
            assert_eq!(date_time, 1_704_110_400 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DT, "DATE_AND_TIME#2024-01-01-12:00:00"), date_time);
            assert_eq!(call_i64(STRING_TO_DT, "dt#2024-01-01-12:00:00"), date_time);

            let time_of_day = call_i64(STRING_TO_TOD, "TOD#12:00:00");
            assert_eq!(time_of_day, 12 * 3_600_000 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TOD, "TIME_OF_DAY#12:00:00"), time_of_day);
            assert_eq!(call_i64(STRING_TO_TOD, "tod#12:00:00"), time_of_day);
        }
    }

    #[test]
    fn long_prefixes_are_rejected_for_short_types() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "LTIME#1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "LT#1s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "TIME#1s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "T#1s"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "LDATE#2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "LD#2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "LDT#2024-01-01-12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "LDATE_AND_TIME#2024-01-01-12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "LTOD#12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "LTIME_OF_DAY#12:00:00"), 0);
        }
    }

    #[test]
    fn the_literal_grammar_is_not_relaxed_for_calendar_types() {
        unsafe {
            // date-only and hours-only forms are not literals
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01-12"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12"), 0);
            // neither are ISO / space separators
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01T12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01 12:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024T01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-01-00:00:00"), 0);
            // but the compiler's grammar is: single-digit fields and optional seconds
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-1-1"), (1_704_067_200) * NANOS_PER_SECOND);
            assert_eq!(
                call_i64(STRING_TO_DT, "DT#2024-1-1-1:2:3"),
                (1_704_067_200 + 3_723) * NANOS_PER_SECOND
            );
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01-12:00"), (1_704_110_400) * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:00"), (12 * 3_600_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#1:2:3"), (3_723_000) * NANOS_PER_MILLISECOND);
        }
    }

    #[test]
    fn trailing_input_is_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-01§"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-01//junk"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-01(*junk*)"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-01{junk}"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-01-01x"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h§30m"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s//x"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s(*x*)"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s{x}"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1sx"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s§"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1s{x}"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:00:00//x"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:00:00§"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:00:00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01-12:00:00{x}"), 0);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2024-01-01-12:00:00§"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "12//x"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "12(*x*)"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "12{x}"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "12§"), 0);
            assert!(!call_bool("TRUE//x"));
            assert!(!call_bool("TRUE§"));
        }
    }

    #[test]
    fn whitespace_and_underscores_separate_segments_but_never_split_numbers() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h 30m"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h_30m"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h\t30m"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h\x0B30m"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h  30m"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1_000ms"), (1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1h 30m"), 90 * 60 * 1_000_000_000);

            assert_eq!(call_i64(STRING_TO_TIME, "T#1 0s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h3 0m"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1 s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1_ 0s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T# 1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T #1s"), 0);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12 :00:00"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D# 2024-01-01"), 0);
        }
    }

    #[test]
    fn segments_may_repeat_or_be_unordered() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#30m1h"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h1h"), (2 * 60 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1ms1s"), (1_001) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s1ms"), (1_001) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1ns1d"), 86_400_000_000_001);
        }
    }

    #[test]
    fn plus_sign_is_accepted_on_durations() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#+1s"), (1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#+1s"), 1_000_000_000);
            assert_eq!(call_i64(STRING_TO_TIME, "T#+-1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#++1s"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#1s+1s"), 0);
        }
    }

    #[test]
    fn nanosecond_segments_are_not_truncated_to_u32() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#5000000000ns"), (5_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#5000000000ns"), 5_000_000_000);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#4294967296ns"), 4_294_967_296);
        }
    }

    #[test]
    fn overflowing_durations_are_rejected() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TIME, "T#49d17h2m47s296ms"), 4_294_967_296 * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#50d"), 50 * 86_400 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_TIME, "T#106751d23h47m16s854ms775us807ns"), i64::MAX);
            assert_eq!(call_i64(STRING_TO_TIME, "T#106751d23h47m16s854ms775us808ns"), 0);
            assert_eq!(call_i64(STRING_TO_TIME, "T#106752d"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#106751d23h47m16s854ms775us807ns"), i64::MAX);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#106751d23h47m16s854ms775us808ns"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#106752d"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#99999999999999999999999999999999999999999d"), 0);
        }
    }

    #[test]
    fn fractions_of_a_second_truncate_instead_of_rounding() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#23:59:59.999"), (86_399_999) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#23:59:59.9999999999"), 86_400 * NANOS_PER_SECOND - 1);
            assert_eq!(
                call_i64(STRING_TO_TOD, "TOD#12:00:59.9999999999"),
                (12 * 3_600 + 60) * NANOS_PER_SECOND - 1
            );
            assert_eq!(
                call_i64(STRING_TO_TOD, "TOD#12:00:00.9999999999"),
                (12 * 3_600 + 1) * NANOS_PER_SECOND - 1
            );
            assert_eq!(
                call_i64(STRING_TO_TOD, "TOD#12:00:00.5"),
                (12 * 3_600_000 + 500) * NANOS_PER_MILLISECOND
            );
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#12:00:00.0005"), 12 * 3_600 * NANOS_PER_SECOND + 500_000);
            assert_eq!(
                call_i64(STRING_TO_DT, "DT#2024-01-01-23:59:59.9999999999"),
                (1_704_067_200 + 86_400) * NANOS_PER_SECOND - 1
            );
            assert_eq!(
                call_i64(STRING_TO_DT, "DT#2024-01-01-12:00:00.5"),
                1_704_110_400 * NANOS_PER_SECOND + 500 * NANOS_PER_MILLISECOND
            );
            assert_eq!(call_i64(STRING_TO_TOD, "TOD#24:00:00"), 0);
        }
    }

    #[test]
    fn calendar_arithmetic_matches_the_proleptic_gregorian_calendar() {
        unsafe {
            assert_eq!(call_i64(STRING_TO_DATE, "D#1970-01-02"), (86_400) * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2000-02-29"), (951_782_400) * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2096-02-29"), (3_981_312_000) * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2100-02-29"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2106-02-07"), (4_294_944_000) * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2106-02-07-06:28:15"), 4_294_967_295 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DT, "DT#2106-02-07-06:28:16"), 4_294_967_296 * NANOS_PER_SECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-04-31"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024-00-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#0-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#-2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#+2024-01-01"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#20240101"), 0);
            assert_eq!(call_i64(STRING_TO_DATE, "D#2024/01/01"), 0);
        }
    }

    #[test]
    fn based_literals_take_no_sign_and_only_single_inner_underscores() {
        unsafe {
            assert_eq!(call_i32(STRING_TO_DINT, "16#-FF"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "-16#FF"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "+16#FF"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#+FF"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "2#-1"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#_FF"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#FF_"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#F__F"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#F_F"), 255);
            assert_eq!(call_i32(STRING_TO_DINT, "16#"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#G"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "#FF"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "10#12"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "16#ff"), 255);
            assert_eq!(call_i32(STRING_TO_DINT, "2#12"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "_1"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1_"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1__0"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1_000"), 1_000);
            assert_eq!(call_i32(STRING_TO_DINT, "-1_000"), -1_000);
            assert_eq!(call_i32(STRING_TO_DINT, "+1"), 1);
            assert_eq!(call_i32(STRING_TO_DINT, "- 1"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "--1"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "007"), 7);
            assert_eq!(call_i32(STRING_TO_DINT, "-0"), 0);
        }
    }

    #[test]
    fn real_and_exponent_forms_follow_the_compiler() {
        unsafe {
            assert_eq!(call_i32(STRING_TO_DINT, "1e3"), 1_000);
            assert_eq!(call_i32(STRING_TO_DINT, "1E3"), 1_000);
            assert_eq!(call_i32(STRING_TO_DINT, "1e+3"), 1_000);
            assert_eq!(call_i32(STRING_TO_DINT, "1.5e2"), 150);
            assert_eq!(call_i32(STRING_TO_DINT, "1e-3"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1e"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "e3"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1.9"), 1);
            assert_eq!(call_i32(STRING_TO_DINT, "-1.9"), -1);
            assert_eq!(call_i32(STRING_TO_DINT, "1.0"), 1);
            assert_eq!(call_i32(STRING_TO_DINT, "123."), 0);
            assert_eq!(call_i32(STRING_TO_DINT, ".5"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1.9.9"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "1.5e2.5"), 0);
        }
    }

    #[test]
    fn integers_that_do_not_fit_the_target_yield_zero() {
        unsafe {
            assert_eq!(call_i32(STRING_TO_DINT, "2147483647"), i32::MAX);
            assert_eq!(call_i32(STRING_TO_DINT, "2147483648"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "-2147483648"), i32::MIN);
            assert_eq!(call_i32(STRING_TO_DINT, "-2147483649"), 0);
            assert_eq!(call_i32(STRING_TO_DINT, "4294967297"), 0);
            assert_eq!(call_i16(STRING_TO_INT, "32767"), i16::MAX);
            assert_eq!(call_i16(STRING_TO_INT, "32768"), 0);
            assert_eq!(call_i16(STRING_TO_INT, "16#7FFF"), i16::MAX);
            assert_eq!(call_i16(STRING_TO_INT, "16#FFFF"), 0);
            assert_eq!(call_u8(STRING_TO_BYTE, "255"), u8::MAX);
            assert_eq!(call_u8(STRING_TO_BYTE, "256"), 0);
            assert_eq!(call_u8(STRING_TO_BYTE, "16#100"), 0);
            assert_eq!(call_u8(STRING_TO_BYTE, "2#100000000"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "4294967295"), u32::MAX);
            assert_eq!(call_u32(STRING_TO_UDINT, "4294967296"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "-1"), 0);
            assert_eq!(call_i64(STRING_TO_LINT, "9223372036854775807"), i64::MAX);
            assert_eq!(call_i64(STRING_TO_LINT, "9223372036854775808"), 0);
            assert_eq!(call_i64(STRING_TO_LINT, "-9223372036854775808"), i64::MIN);
            assert_eq!(call_i64(STRING_TO_LINT, "-9223372036854775809"), 0);
            assert_eq!(call_i64(STRING_TO_LINT, "16#FFFFFFFFFFFFFFFF"), 0);
            assert_eq!(call_u64(STRING_TO_LWORD, "16#FFFFFFFFFFFFFFFF"), u64::MAX);
            assert_eq!(call_u64(STRING_TO_ULINT, "18446744073709551615"), u64::MAX);
            assert_eq!(call_u64(STRING_TO_ULINT, "18446744073709551616"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "340282366920938463463374607431768211456"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "99999999999999999999999999999999999999999999"), 0);
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
            assert_eq!(call_i64(STRING_TO_TIME, "T#1h 30m"), (90 * 60 * 1_000) * NANOS_PER_MILLISECOND);
            assert_eq!(call_i64(STRING_TO_DATE, "D# 2024-01-01"), 0);
        }
    }
}
