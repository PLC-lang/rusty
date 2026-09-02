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
use std::str::FromStr;

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
pub extern "C" fn TIME() -> i64 {
    let dt = Local::now();
    dt.num_seconds_from_midnight() as i64 * 1e9 as i64 + dt.nanosecond() as i64
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_TIME(input: f64) -> i64 {
    input.round() as i64
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LTIME(input: f64) -> i64 {
    input.round() as i64
}

#[cfg(test)]
mod test {
    use super::*;

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
}
