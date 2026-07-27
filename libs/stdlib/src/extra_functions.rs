#[cfg(not(feature = "mock_time"))]
use chrono::offset::Local;

#[cfg(feature = "mock_time")]
use crate::extra_functions::test_time_helpers::Local;

#[cfg(feature = "mock_time")]
pub mod test_time_helpers;

use crate::string_functions::ptr_to_slice;
use chrono::{TimeZone, Timelike};
use num::{Float, PrimInt};
use std::{io::Write, str::FromStr};

// can't determine string buffer length of an empty string, therefore
// _TO_STRING functions use the default string length.
const DEFAULT_STRING_LEN: usize = 81;
const NANOS_PER_MILLISECOND: i64 = 1_000 * 1_000;
const NANOS_PER_SECOND: i64 = 1_000 * NANOS_PER_MILLISECOND;
// --------- x_TO_STRING

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn BYTE_TO_STRING_EXT(input: u8, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{input}").unwrap();

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LWORD_TO_STRING_EXT(input: u64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{input}").unwrap();

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LINT_TO_STRING_EXT(input: i64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    write!(&mut *buf, "{input}").unwrap();

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LREAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    // double: 52 bits are used for the mantissa (about 16 decimal digits)
    if input.floor() < 1e14 {
        write!(&mut *buf, "{input:.6}").unwrap()
    } else {
        write!(&mut *buf, "{input:.6e}").unwrap()
    }

    0
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn REAL_TO_STRING_EXT(input: f64, dest: *mut u8) -> i32 {
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
    // float: 23 bits are used for the mantissa (about 7 decimal digits)

    // TODO: discuss when scientific notation should be displayed
    if input.floor() < 1e6 {
        write!(&mut *buf, "{input:.6}").unwrap()
    } else {
        write!(&mut *buf, "{input:.6e}").unwrap()
    }

    0
}

unsafe fn string_to_int<T>(src: *const u8) -> T
where
    T: PrimInt,
{
    let slice = ptr_to_slice(src);
    let (string, radix) = match slice {
        [b'1', b'6', b'#', ..] => (std::str::from_utf8(&slice[3..]), 16),
        [b'0', b'x', ..] | [b'0', b'X', ..] => (std::str::from_utf8(&slice[2..]), 16),
        [b'8', b'#', ..] => (std::str::from_utf8(&slice[2..]), 8), // support c-style octal prefixes? e.g. 010 -> 10 octal
        [b'2', b'#', ..] | [b'0', b'b', ..] | [b'0', b'B', ..] => (std::str::from_utf8(&slice[2..]), 2),
        _ => (std::str::from_utf8(slice), 10),
    };

    // Parse the longest valid prefix instead of panicking on malformed input.
    // For example "12j3" yields 12 and "123.456" yields 123, while a string with no valid prefix yields 0.
    match string {
        Ok(s) => parse_longest_prefix(s, |candidate| T::from_str_radix(candidate, radix).ok()),
        Err(_) => T::zero(),
    }
}

/// Returns the value parsed from the longest prefix of `s` accepted by `parse`, or the type's
/// default (`0`) when no non-empty prefix is valid.
fn parse_longest_prefix<T: num::Zero>(s: &str, parse: impl Fn(&str) -> Option<T>) -> T {
    let mut end = s.len();
    while end > 0 {
        if s.is_char_boundary(end) {
            if let Some(number) = parse(&s[..end]) {
                return number;
            }
        }
        end -= 1;
    }
    T::zero()
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn STRING_TO_LINT(src: *const u8) -> i64 {
    string_to_int(src)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_DINT(src: *const u8) -> i32 {
    string_to_int(src)
}

unsafe fn string_to_float<T>(src: *const u8) -> T
where
    T: Float + FromStr,
{
    let slice = ptr_to_slice(src);

    // Parse the longest valid prefix instead of panicking on malformed input.
    // For example "1.2j3" yields 1.2, while "asdf" or an empty string yield 0.0.
    match std::str::from_utf8(slice) {
        Ok(s) => parse_longest_prefix(s, |candidate| candidate.parse::<T>().ok()),
        Err(_) => T::zero(),
    }
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn STRING_TO_LREAL(src: *const u8) -> f64 {
    string_to_float(src)
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn STRING_TO_REAL(src: *const u8) -> f32 {
    string_to_float(src)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TIME() -> u32 {
    let dt = Local::now();
    dt.num_seconds_from_midnight() * 1_000 + (dt.nanosecond() / NANOS_PER_MILLISECOND as u32)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LTIME() -> i64 {
    let dt = Local::now();
    dt.num_seconds_from_midnight() as i64 * NANOS_PER_SECOND + dt.nanosecond() as i64
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TIME_TO_STRING(dest: *mut u8, input: i32) {
    write_time_to_string((input as u32 as i64) * NANOS_PER_MILLISECOND, dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LTIME_TO_STRING(dest: *mut u8, input: i64) {
    write_time_to_string(input, dest);
}

unsafe fn write_time_to_string(input_nanos: i64, dest: *mut u8) {
    let mut dest = dest;
    let literals = parse_timestamp(input_nanos);
    literals.iter().filter(|&it| it.0 != 0).for_each(|it| {
        let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);
        write!(&mut *buf, "{}{}", it.0, it.1).unwrap();
        let idx = buf.iter().position(|&c| c == 0).unwrap();
        dest = dest.add(idx);
    });
}

fn parse_timestamp<'a>(timestamp_nanos: i64) -> [(u32, &'a str); 7] {
    let datetime = chrono::Utc.timestamp_nanos(timestamp_nanos);
    let (nanos, micros, millis, seconds, minutes, hours) = (
        datetime.timestamp_subsec_nanos() % 1000,
        datetime.timestamp_subsec_micros() % 1000,
        datetime.timestamp_subsec_millis(),
        datetime.second(),
        datetime.minute(),
        datetime.hour(),
    );
    let nanos_per_day = 1e9 as i64 * 3600 * 24;
    let days = (timestamp_nanos / nanos_per_day) as u32;

    [(days, "d"), (hours, "h"), (minutes, "m"), (seconds, "s"), (millis, "ms"), (micros, "us"), (nanos, "ns")]
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DT_TO_STRING(dest: *mut u8, input: i32) {
    write_dt_to_string((input as u32 as i64) * NANOS_PER_SECOND, dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LDT_TO_STRING(dest: *mut u8, input: i64) {
    write_dt_to_string(input, dest);
}

unsafe fn write_dt_to_string(input_nanos: i64, dest: *mut u8) {
    let datetime = chrono::Utc.timestamp_nanos(input_nanos);
    let date = datetime.date_naive().to_string();
    let time = datetime.time().to_string();
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{date}-{time}").unwrap();
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DATE_TO_STRING(dest: *mut u8, input: i32) {
    write_date_to_string((input as u32 as i64) * NANOS_PER_SECOND, dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LDATE_TO_STRING(dest: *mut u8, input: i64) {
    write_date_to_string(input, dest);
}

unsafe fn write_date_to_string(input_nanos: i64, dest: *mut u8) {
    let datetime = chrono::Utc.timestamp_nanos(input_nanos).date_naive();
    let date = datetime.to_string();
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{date}").unwrap();
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TOD_TO_STRING(dest: *mut u8, input: i32) {
    write_tod_to_string((input as u32 as i64) * NANOS_PER_MILLISECOND, dest);
}

/// # Safety
/// Uses raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LTOD_TO_STRING(dest: *mut u8, input: i64) {
    write_tod_to_string(input, dest);
}

unsafe fn write_tod_to_string(input_nanos: i64, dest: *mut u8) {
    let datetime = chrono::Utc.timestamp_nanos(input_nanos);
    let time = datetime.time().to_string();
    let buf = core::slice::from_raw_parts_mut(dest, DEFAULT_STRING_LEN);

    write!(&mut *buf, "{time}").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    // tests
    #[test]
    fn byte_to_string_conversion() {
        let byte = 0b1010_1010_u8;
        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();

        let _ = unsafe { BYTE_TO_STRING_EXT(byte, dest_ptr) };
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!(0b1010_1010_u8.to_string(), res.trim_end_matches('\0'));
    }

    #[test]
    fn lword_to_string_conversion() {
        let lword = 0xFF_00_FF_00_00_FF_00_FF_u64;
        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();

        let _ = unsafe { LWORD_TO_STRING_EXT(lword, dest_ptr) };
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!(0xFF_00_FF_00_00_FF_00_FF_u64.to_string(), res.trim_end_matches('\0'));
    }

    #[test]
    fn lint_to_string_conversion() {
        let lint = 100_200_300_400_500_i64;
        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();

        let _ = unsafe { LINT_TO_STRING_EXT(lint, dest_ptr) };
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!("100200300400500", res.trim_end_matches('\0'));
    }

    #[test]
    fn lreal_to_string_conversion() {
        let lreal = 10230.2321123121;
        let lreal_neg = -lreal;
        let pre_e_notation = 99_999_999_999_999.25;
        let e_notation = 123_456_789_123_456.13;
        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        let _ = unsafe { LREAL_TO_STRING_EXT(lreal, dest_ptr) };
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!(format!("{lreal:.6}"), res.trim_end_matches('\0'));

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        let _ = unsafe { LREAL_TO_STRING_EXT(lreal_neg, dest_ptr) };
        let res_neg = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!(format!("{lreal_neg:.6}"), res_neg.trim_end_matches('\0'));

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        let _ = unsafe { LREAL_TO_STRING_EXT(pre_e_notation, dest_ptr) };
        let res_large = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!(format!("{pre_e_notation:.6}"), res_large.trim_end_matches('\0'));

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        let _ = unsafe { LREAL_TO_STRING_EXT(e_notation, dest_ptr) };
        let res_scientific =
            std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();

        assert_eq!(format!("{e_notation:.6e}"), res_scientific.trim_end_matches('\0'));
    }

    #[test]
    fn string_to_lint_conversion() {
        let string = "12345\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(12345_i64, result);

        let string = "2#1111\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(15_i64, result);

        let string = "8#77\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(63_i64, result);

        let string = "16#FF\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(255_i64, result);

        let string = "0b1111\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(15_i64, result);

        let string = "0B1111\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(15_i64, result);

        let string = "0xFF\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(255_i64, result);

        let string = "0XFF\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(255_i64, result);
    }

    #[test]
    fn string_to_lint_parses_longest_valid_prefix() {
        // a string with no valid prefix yields 0 instead of panicking
        let string = "ab456\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(0_i64, result);

        // parsing stops at the first invalid character
        let string = "12j3\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(12_i64, result);

        // the integer part of a decimal number is parsed (the '.' ends the prefix)
        let string = "123.456\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(123_i64, result);

        // empty string yields 0
        let string = "\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(0_i64, result);

        // radix prefixes are still honoured, and trailing garbage is trimmed off the digits.
        // Note 'E'/'F' are valid hex digits here, not an exponent: "16#FFE0" parses fully.
        let string = "16#FFxy\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(255_i64, result);

        let string = "16#FFE0\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(65504_i64, result);

        let string = "8#1239\0"; // 9 is not a valid octal digit, so the prefix is "123"
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(0o123_i64, result);

        let string = "2#10102\0"; // 2 is not a valid binary digit, so the prefix is "1010"
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(0b1010_i64, result);

        let string = "0xdeadZZ\0";
        let result = unsafe { STRING_TO_LINT(string.as_ptr()) };
        assert_eq!(0xdead_i64, result);
    }

    #[test]
    fn string_to_lreal_conversion() {
        let string = "1.25\0";
        let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
        assert_eq!(1.25, result);
    }

    #[test]
    fn string_to_real_conversion() {
        let string = "1.25\0";
        let result = unsafe { STRING_TO_REAL(string.as_ptr()) };
        assert_eq!(1.25, result);
    }

    #[test]
    fn string_to_lreal_parses_longest_valid_prefix() {
        // parsing stops at the first invalid character instead of panicking
        let string = "1,25f\0";
        let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
        assert_eq!(1.0, result);

        let string = "1.2j3\0";
        let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
        assert_eq!(1.2, result);

        // ST escape sequences (here $R$N -> CR LF) trailing the number are ignored
        let string = "123.456\r\n\0";
        let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
        assert_eq!(123.456, result);

        // a string with no valid prefix yields 0.0
        let string = "asdf\0";
        let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
        assert_eq!(0.0, result);

        // empty string yields 0.0
        let string = "\0";
        let result = unsafe { STRING_TO_LREAL(string.as_ptr()) };
        assert_eq!(0.0, result);
    }

    #[test]
    fn date_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(0, 0, 0, 0))
            .expect("Cannot create date time from given parameters");
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        unsafe { LDATE_TO_STRING(dest_ptr, timestamp) };

        let expected = "1982-12-15";
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();
        let res = res.trim_end_matches('\0');
        assert_eq!(expected, res);
    }

    #[test]
    fn dt_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123456789))
            .expect("Cannot create date time from given parameters");
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        unsafe { LDT_TO_STRING(dest_ptr, timestamp) };

        let expected = "1982-12-15-10:10:02.123456789";
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();
        let res = res.trim_end_matches('\0');
        assert_eq!(expected, res);
    }

    #[test]
    fn tod_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123456789))
            .expect("Cannot create date time from given parameters");
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        unsafe { LTOD_TO_STRING(dest_ptr, timestamp) };

        let expected = "10:10:02.123456789";
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();
        let res = res.trim_end_matches('\0');
        assert_eq!(expected, res);
    }

    #[test]
    fn time_to_string_is_converted_in_correct_format() {
        let datetime = chrono::NaiveDate::from_ymd_opt(2023, 1, 23)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 0, 123456789))
            .expect("Cannot create date time from given parameters");
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();

        let mut dest = [0_u8; 81];
        let dest_ptr = dest.as_mut_ptr();
        unsafe { LTIME_TO_STRING(dest_ptr, timestamp) };

        let expected = "19380d10h10m123ms456us789ns";
        let res = std::str::from_utf8(unsafe { core::slice::from_raw_parts(dest_ptr, 81) }).unwrap();
        let res = res.trim_end_matches('\0');
        assert_eq!(expected, res);
    }
}
