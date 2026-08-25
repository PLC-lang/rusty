//! `STRING_TO_*` conversions for BOOL, the bit/integer widths, and the short duration/date/time
//! types.
//!
//! All functions here share the same contract:
//! - Never fault: every input, including malformed or empty ones, returns a value.
//! - Surrounding whitespace (space, tab, CR, LF, FF, VT) is trimmed before any other rule applies.
//! - A rejected input returns the type's zero value.

use crate::string_functions::ptr_to_slice;
use num::PrimInt;

const MILLIS_PER_SECOND: u32 = 1_000;
const MILLIS_PER_MINUTE: u32 = 60 * MILLIS_PER_SECOND;
const MILLIS_PER_HOUR: u32 = 60 * MILLIS_PER_MINUTE;
const NANOS_PER_MICROSECOND: f64 = 1_000.0;
const NANOS_PER_MILLISECOND: f64 = 1_000_000.0;
const NANOS_PER_SECOND: f64 = 1_000_000_000.0;
const NANOS_PER_MINUTE: f64 = 60.0 * NANOS_PER_SECOND;
const NANOS_PER_HOUR: f64 = 60.0 * NANOS_PER_MINUTE;
const NANOS_PER_DAY: f64 = 24.0 * NANOS_PER_HOUR;

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

/// Strips one of `prefixes` from `s`, matching case-insensitively. Returns `None` if none match.
fn strip_prefix_ci<'a>(s: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    let upper: String = s.chars().map(|c| c.to_ascii_uppercase()).collect();
    prefixes.iter().find(|p| upper.starts_with(*p)).map(|p| &s[p.len()..])
}

fn all_ascii_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

fn parse_plain_u32(s: &str) -> Option<u32> {
    if !all_ascii_digits(s) {
        return None;
    }
    s.parse().ok()
}

// --------- BOOL

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_BOOL(src: *const u8) -> bool {
    match trimmed_str(src) {
        "1" => true,
        "0" => false,
        s if s.eq_ignore_ascii_case("TRUE") => true,
        _ => false,
    }
}

// --------- integer / bit-string widths

/// One radix-prefix + full-consumption parse rule shared by every integer width, so widening a
/// variable never changes the parsed value.
fn parse_int_strict<T: PrimInt>(s: &str) -> T {
    let (radix, rest) = match s.as_bytes() {
        [b'1', b'6', b'#', ..] => (16, &s[3..]),
        [b'0', b'x', ..] | [b'0', b'X', ..] => (16, &s[2..]),
        [b'8', b'#', ..] => (8, &s[2..]),
        [b'2', b'#', ..] | [b'0', b'b', ..] | [b'0', b'B', ..] => (2, &s[2..]),
        _ => (10, s),
    };

    if radix != 10 {
        return T::from_str_radix(rest, radix).unwrap_or_else(|_| T::zero());
    }

    // decimal: a fractional part is allowed and truncated toward zero, everything else must
    // consume the whole (trimmed) string.
    let (int_part, frac_part) = match rest.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (rest, None),
    };
    if let Some(frac) = frac_part {
        if !all_ascii_digits(frac) {
            return T::zero();
        }
    }

    T::from_str_radix(int_part, 10).unwrap_or_else(|_| T::zero())
}

macro_rules! string_to_int_fn {
    ($name:ident, $ty:ty) => {
        /// # Safety
        /// Uses raw pointers, inherently unsafe.
        #[allow(non_snake_case)]
        #[no_mangle]
        pub unsafe extern "C" fn $name(src: *const u8) -> $ty {
            parse_int_strict(trimmed_str(src))
        }
    };
}

string_to_int_fn!(STRING_TO_BYTE, u8);
string_to_int_fn!(STRING_TO_WORD, u16);
string_to_int_fn!(STRING_TO_DWORD, u32);
string_to_int_fn!(STRING_TO_LWORD, u64);
string_to_int_fn!(STRING_TO_SINT, i8);
string_to_int_fn!(STRING_TO_USINT, u8);
string_to_int_fn!(STRING_TO_INT, i16);
string_to_int_fn!(STRING_TO_UINT, u16);
string_to_int_fn!(STRING_TO_UDINT, u32);
string_to_int_fn!(STRING_TO_ULINT, u64);

// --------- durations (TIME / LTIME)

/// Parses the body after the `T#`/`TIME#`/`LT#`/`LTIME#` prefix has been stripped, returning the
/// duration in (fractional) nanoseconds. Rejects negative durations, out-of-order or duplicate
/// unit segments, and unknown units.
fn parse_duration_body_nanos(body: &str) -> Option<f64> {
    if body.is_empty() || body.contains('-') {
        return None;
    }

    let normalized = body.replace(',', ".");
    let bytes = normalized.as_bytes();
    let mut i = 0;
    let mut total_nanos = 0.0_f64;
    let mut last_rank: i32 = -1;
    let mut seen_any = false;

    while i < bytes.len() {
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        let num_start = i;
        let mut seen_dot = false;
        while i < bytes.len() && (bytes[i].is_ascii_digit() || (bytes[i] == b'.' && !seen_dot)) {
            seen_dot |= bytes[i] == b'.';
            i += 1;
        }
        if i == num_start {
            return None;
        }
        let number: f64 = normalized[num_start..i].parse().ok()?;

        let unit_start = i;
        while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
            i += 1;
        }
        if i == unit_start {
            return None;
        }
        let unit = normalized[unit_start..i].to_ascii_lowercase();

        let (rank, nanos_per_unit) = match unit.as_str() {
            "d" => (0, NANOS_PER_DAY),
            "h" => (1, NANOS_PER_HOUR),
            "m" => (2, NANOS_PER_MINUTE),
            "s" => (3, NANOS_PER_SECOND),
            "ms" => (4, NANOS_PER_MILLISECOND),
            "us" => (5, NANOS_PER_MICROSECOND),
            "ns" => (6, 1.0),
            _ => return None,
        };
        // segments must appear in d-h-m-s-ms-us-ns order and each unit at most once
        if rank <= last_rank {
            return None;
        }
        last_rank = rank;

        total_nanos += number * nanos_per_unit;
        seen_any = true;
    }

    seen_any.then_some(total_nanos)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_TIME(src: *const u8) -> u32 {
    let s = trimmed_str(src);
    let Some(body) = strip_prefix_ci(s, &["TIME#", "T#"]) else {
        return 0;
    };
    match parse_duration_body_nanos(body) {
        Some(nanos) => (nanos / NANOS_PER_MILLISECOND).trunc().clamp(0.0, u32::MAX as f64) as u32,
        None => 0,
    }
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_LTIME(src: *const u8) -> i64 {
    let s = trimmed_str(src);
    let Some(body) = strip_prefix_ci(s, &["LTIME#", "LT#"]) else {
        return 0;
    };
    match parse_duration_body_nanos(body) {
        Some(nanos) => nanos.trunc().clamp(0.0, i64::MAX as f64) as i64,
        None => 0,
    }
}

// --------- dates and times of day (DATE / DT / TOD)

/// Splits `yyyy-mm-dd[-T]hh:mm:ss[.fff]` at the date/time boundary: either an `'T'`/`'t'`
/// separator, or the third `'-'` (the first two belong to the date itself).
fn split_date_time(body: &str) -> Option<(&str, &str)> {
    let mut dash_count = 0;
    for (idx, ch) in body.char_indices() {
        if ch == 'T' || ch == 't' {
            return Some((&body[..idx], &body[idx + ch.len_utf8()..]));
        }
        if ch == '-' {
            dash_count += 1;
            if dash_count == 3 {
                return Some((&body[..idx], &body[idx + 1..]));
            }
        }
    }
    None
}

fn parse_ymd(body: &str) -> Option<chrono::NaiveDate> {
    let mut it = body.split('-');
    let year_str = it.next()?;
    let month_str = it.next()?;
    let day_str = it.next()?;
    if it.next().is_some() || !all_ascii_digits(year_str) {
        return None;
    }
    let year: i32 = year_str.parse().ok()?;
    let month = parse_plain_u32(month_str)?;
    let day = parse_plain_u32(day_str)?;
    chrono::NaiveDate::from_ymd_opt(year, month, day)
}

/// Parses `hh:mm:ss[.fff]`, returning `(hour, minute, second, nanosecond)`. Rejects out-of-range
/// components (hour > 23, minute/second > 59).
fn parse_hms(body: &str) -> Option<(u32, u32, u32, u32)> {
    let mut it = body.split(':');
    let hour = parse_plain_u32(it.next()?)?;
    let min = parse_plain_u32(it.next()?)?;
    let sec_str = it.next()?;
    if it.next().is_some() {
        return None;
    }

    let (sec_str, frac_str) = match sec_str.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (sec_str, None),
    };
    let sec = parse_plain_u32(sec_str)?;
    let nanos = match frac_str {
        Some(f) if all_ascii_digits(f) => {
            let mut digits = f.to_string();
            digits.truncate(9);
            while digits.len() < 9 {
                digits.push('0');
            }
            digits.parse::<u32>().ok()?
        }
        Some(_) => return None,
        None => 0,
    };

    if hour > 23 || min > 59 || sec > 59 {
        return None;
    }
    Some((hour, min, sec, nanos))
}

/// Converts a `NaiveDate`'s midnight into the DATE/DT epoch-seconds representation, rejecting
/// anything outside the representable window (1970-01-01 to 2106-02-07).
fn date_time_to_epoch_seconds(date: chrono::NaiveDate, hour: u32, min: u32, sec: u32) -> Option<u32> {
    let seconds = date.and_hms_opt(hour, min, sec)?.and_utc().timestamp();
    u32::try_from(seconds).ok()
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DATE(src: *const u8) -> u32 {
    let s = trimmed_str(src);
    let body = strip_prefix_ci(s, &["DATE#", "D#"]).unwrap_or(s);
    let result = parse_ymd(body).and_then(|date| date_time_to_epoch_seconds(date, 0, 0, 0));
    result.unwrap_or(0)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DT(src: *const u8) -> u32 {
    let s = trimmed_str(src);
    let body = strip_prefix_ci(s, &["DATE_AND_TIME#", "DT#"]).unwrap_or(s);
    let result = split_date_time(body).and_then(|(date_part, time_part)| {
        let date = parse_ymd(date_part)?;
        let (hour, min, sec, _nanos) = parse_hms(time_part)?;
        date_time_to_epoch_seconds(date, hour, min, sec)
    });
    result.unwrap_or(0)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_TOD(src: *const u8) -> u32 {
    let s = trimmed_str(src);
    let body = strip_prefix_ci(s, &["TIME_OF_DAY#", "TOD#"]).unwrap_or(s);
    match parse_hms(body) {
        Some((hour, min, sec, nanos)) => {
            hour * MILLIS_PER_HOUR
                + min * MILLIS_PER_MINUTE
                + sec * MILLIS_PER_SECOND
                + nanos / NANOS_PER_MILLISECOND as u32
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn call_bool(s: &str) -> bool {
        STRING_TO_BOOL(format!("{s}\0").as_ptr())
    }
    unsafe fn call_u8(f: unsafe extern "C" fn(*const u8) -> u8, s: &str) -> u8 {
        f(format!("{s}\0").as_ptr())
    }
    unsafe fn call_u32(f: unsafe extern "C" fn(*const u8) -> u32, s: &str) -> u32 {
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
            assert_eq!(call_u32(STRING_TO_DATE, "not a date"), 0);
            assert_eq!(call_u32(STRING_TO_DT, "\u{0}"), 0);
            assert_eq!(call_u32(STRING_TO_TOD, "🦀"), 0);
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
            assert_eq!(call_u64(STRING_TO_ULINT, "12abc"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "12 34"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "12 34"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "1e3"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "1e3"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "8#19"), 0);
            assert_eq!(call_u64(STRING_TO_ULINT, "8#19"), 0);
            assert_eq!(call_u32(STRING_TO_UDINT, "  12  "), 12);
            assert_eq!(call_u64(STRING_TO_ULINT, "  12  "), 12);
            assert_eq!(call_u32(STRING_TO_UDINT, "16#FF"), 255);
            assert_eq!(call_u64(STRING_TO_ULINT, "16#FF"), 255);
            assert_eq!(call_u32(STRING_TO_UDINT, "1.9"), 1);
            assert_eq!(call_u64(STRING_TO_ULINT, "1.9"), 1);
            assert_eq!(call_i16(STRING_TO_INT, "-1"), -1);
            assert_eq!(call_u8(STRING_TO_BYTE, "-1"), 0);
        }
    }

    #[test]
    fn fractional_durations_parse_correctly() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_TIME, "T#1.5s"), 1_500);
            assert_eq!(call_u32(STRING_TO_TIME, "T#0.5s"), 500);
            assert_eq!(call_u32(STRING_TO_TIME, "T#2.75s"), 2_750);
            assert_eq!(call_u32(STRING_TO_TIME, "T#1.5h"), 90 * 60 * 1_000);
            assert_eq!(call_u32(STRING_TO_TIME, "T#1,5s"), 1_500);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#1.5s"), 1_500_000_000);
            assert_eq!(call_u32(STRING_TO_TIME, "T#1.0004ms"), 1);
        }
    }

    #[test]
    fn negative_durations_are_rejected() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_TIME, "T#-1s"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "T#-1000ms"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "T#- 1s"), 0);
            assert_eq!(call_i64(STRING_TO_LTIME, "LTIME#-1s"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "T#1s"), 1_000);
        }
    }

    #[test]
    fn the_67ms_constant_is_gone() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_TIME, ""), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "abc"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "1s"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "LTIME#1s"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "T#67ms"), 67);
        }
    }

    #[test]
    fn impossible_dates_are_rejected() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_DATE, "D#2024-02-30"), 0);
            assert_eq!(call_u32(STRING_TO_DATE, "D#2024-13-01"), 0);
            assert_eq!(call_u32(STRING_TO_DATE, "D#2024-01-00"), 0);
            assert_eq!(call_u32(STRING_TO_DATE, "D#2023-02-29"), 0);
            assert_ne!(call_u32(STRING_TO_DATE, "D#2024-02-29"), 0);
            assert_eq!(call_u32(STRING_TO_DT, "DT#2024-01-01-25:00:00"), 0);
            assert_eq!(call_u32(STRING_TO_TOD, "TOD#12:60:00"), 0);
            assert_eq!(call_u32(STRING_TO_TOD, "TOD#25:00:00"), 0);
        }
    }

    #[test]
    fn out_of_range_dates_are_rejected() {
        unsafe {
            assert_eq!(call_u32(STRING_TO_DATE, "D#1969-12-31"), 0);
            assert_eq!(call_u32(STRING_TO_DATE, "D#2106-02-08"), 0);
            assert_eq!(call_u32(STRING_TO_DATE, "D#0001-01-01"), 0);
            assert_eq!(call_u32(STRING_TO_DATE, "D#1970-01-01"), 0);
            assert_ne!(call_u32(STRING_TO_DATE, "D#2106-02-07"), 0);
            assert_eq!(call_u32(STRING_TO_DT, "DT#1969-12-31-23:59:59"), 0);
        }
    }

    #[test]
    fn unprefixed_iso_dates_and_times_are_accepted() {
        unsafe {
            let d1 = call_u32(STRING_TO_DATE, "2024-01-01");
            let d2 = call_u32(STRING_TO_DATE, "D#2024-01-01");
            assert_eq!(d1, d2);
            assert_ne!(d1, 0);

            assert_eq!(call_u32(STRING_TO_TOD, "12:00:00"), 12 * 3_600_000);
            assert_eq!(call_u32(STRING_TO_TOD, "12:00:00.500"), 12 * 3_600_000 + 500);

            let dt1 = call_u32(STRING_TO_DT, "2024-01-01-12:00:00");
            let dt2 = call_u32(STRING_TO_DT, "2024-01-01T12:00:00");
            assert_eq!(dt1, dt2);
            assert_ne!(dt1, 0);
        }
    }

    #[test]
    fn whitespace_is_handled_the_same_everywhere() {
        unsafe {
            assert_ne!(call_u32(STRING_TO_DATE, "D#2024-01-01 "), 0);
            assert_ne!(call_u32(STRING_TO_DATE, "D#2024-01-01\r\n"), 0);
            assert_eq!(call_i16(STRING_TO_INT, "\n12"), 12);
            assert_eq!(call_u64(STRING_TO_ULINT, "12 "), 12);
            assert_ne!(call_u32(STRING_TO_TOD, "TOD#12:00:00\t"), 0);
            assert_eq!(call_u32(STRING_TO_TIME, "T#1h 30m"), 90 * 60 * 1_000);
            assert_eq!(call_u32(STRING_TO_DATE, "D# 2024-01-01"), 0);
        }
    }
}
