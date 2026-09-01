#[cfg(not(feature = "mock_time"))]
use chrono::offset::Local;

#[cfg(feature = "mock_time")]
use crate::extra_functions::test_time_helpers::Local;

#[cfg(feature = "mock_time")]
pub mod test_time_helpers;

use crate::string_functions::ptr_to_slice;
#[cfg(not(feature = "mock_time"))]
use chrono::Timelike;
use num::Float;
#[cfg(test)]
use std::io::Write;
use std::str::FromStr;

const NANOS_PER_MILLISECOND: i64 = 1_000 * 1_000;
const NANOS_PER_SECOND: i64 = 1_000 * NANOS_PER_MILLISECOND;

#[cfg(test)]
unsafe fn write_terminated(dest: *mut u8, capacity: usize, args: std::fmt::Arguments) -> usize {
    let content = core::slice::from_raw_parts_mut(dest, capacity - 1);
    let mut cursor = std::io::Cursor::new(content);
    let _ = cursor.write_fmt(args);
    let written = cursor.position() as usize;
    *dest.add(written) = 0;
    written
}

/// Returns the value parsed from the longest prefix of `s` accepted by `parse`, or the type's
/// default (`0`) when no non-empty prefix is valid.
fn parse_longest_prefix<T: num::Zero>(s: &str, parse: impl Fn(&str) -> Option<T>) -> T {
    let mut end = s.len();
    while end > 0 {
        // `get` returns None between char boundaries
        if let Some(number) = s.get(..end).and_then(&parse) {
            return number;
        }
        end -= 1;
    }
    T::zero()
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_TIME(input: f64) -> u32 {
    input.round() as u32
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LTIME(input: f64) -> i64 {
    input.round() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::to_string_conversions::*;

    /// Reads the IEC string (up to the first terminator) from a buffer.
    fn terminated_str(buf: &[u8]) -> &str {
        let len = buf.iter().position(|&c| c == 0).expect("result must be null-terminated");
        std::str::from_utf8(&buf[..len]).unwrap()
    }

    #[test]
    fn conversions_terminate_results_in_dirty_buffers() {
        // Result buffers are not guaranteed to be zeroed; every writer must
        // terminate its own output instead of relying on zeroed memory.
        let mut dest = [0xAA_u8; 81];
        let dest_ptr = dest.as_mut_ptr();

        unsafe { BYTE_TO_STRING_EXT(42, dest_ptr) };
        assert_eq!("42", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LWORD_TO_STRING_EXT(7, dest_ptr) };
        assert_eq!("7", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LINT_TO_STRING_EXT(-12, dest_ptr) };
        assert_eq!("-12", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { REAL_TO_STRING_EXT(3.5, dest_ptr) };
        assert_eq!("3.500000", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LREAL_TO_STRING_EXT(2.5, dest_ptr) };
        assert_eq!("2.500000", terminated_str(&dest));

        let datetime = chrono::NaiveDate::from_ymd_opt(1982, 12, 15)
            .and_then(|date| date.and_hms_nano_opt(10, 10, 2, 123456789))
            .expect("Cannot create date time from given parameters");
        let timestamp = datetime.and_utc().timestamp_nanos_opt().unwrap();

        dest.fill(0xAA);
        unsafe { LDATE_TO_STRING(dest_ptr, timestamp) };
        assert_eq!("1982-12-15", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LDT_TO_STRING(dest_ptr, timestamp) };
        assert_eq!("1982-12-15-10:10:02.123456789", terminated_str(&dest));

        dest.fill(0xAA);
        unsafe { LTOD_TO_STRING(dest_ptr, timestamp) };
        assert_eq!("10:10:02.123456789", terminated_str(&dest));

        // skipped zero-components must not leave stale bytes between the parts
        dest.fill(0xAA);
        unsafe { LTIME_TO_STRING(dest_ptr, (2 * 3600 + 3) * 1_000_000_000) };
        assert_eq!("2h3s", terminated_str(&dest));

        // a zero duration writes no component but must still terminate
        dest.fill(0xAA);
        unsafe { LTIME_TO_STRING(dest_ptr, 0) };
        assert_eq!("", terminated_str(&dest));
    }

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
    fn lreal_to_string_uses_scientific_notation_for_huge_negative_values() {
        let mut dest = [0xAA_u8; 81];
        let dest_ptr = dest.as_mut_ptr();

        let _ = unsafe { LREAL_TO_STRING_EXT(-1.0e300, dest_ptr) };
        assert_eq!("-1.000000e300", terminated_str(&dest));

        // negative values below the threshold keep the plain notation
        dest.fill(0xAA);
        let _ = unsafe { LREAL_TO_STRING_EXT(-99_999_999_999_999.25, dest_ptr) };
        assert_eq!("-99999999999999.250000", terminated_str(&dest));

        dest.fill(0xAA);
        let _ = unsafe { LREAL_TO_STRING_EXT(f64::NEG_INFINITY, dest_ptr) };
        assert_eq!("-inf", terminated_str(&dest));

        dest.fill(0xAA);
        let _ = unsafe { LREAL_TO_STRING_EXT(f64::NAN, dest_ptr) };
        assert_eq!("NaN", terminated_str(&dest));
    }

    #[test]
    fn real_to_string_uses_scientific_notation_by_magnitude() {
        let mut dest = [0xAA_u8; 81];
        let dest_ptr = dest.as_mut_ptr();

        let _ = unsafe { REAL_TO_STRING_EXT(-1.5e7, dest_ptr) };
        assert_eq!("-1.500000e7", terminated_str(&dest));

        dest.fill(0xAA);
        let _ = unsafe { REAL_TO_STRING_EXT(1.5e7, dest_ptr) };
        assert_eq!("1.500000e7", terminated_str(&dest));

        // negative values below the threshold keep the plain notation
        dest.fill(0xAA);
        let _ = unsafe { REAL_TO_STRING_EXT(-999_999.25, dest_ptr) };
        assert_eq!("-999999.250000", terminated_str(&dest));
    }

    #[test]
    fn write_terminated_truncates_overlong_output() {
        let mut dest = [0xAA_u8; 16];

        let written = unsafe { write_terminated(dest.as_mut_ptr(), 16, format_args!("{:x<30}", "abc")) };

        assert_eq!(15, written);
        assert_eq!("abcxxxxxxxxxxxx", terminated_str(&dest));
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
