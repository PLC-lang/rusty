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

const NANOS_PER_MILLISECOND: i64 = 1_000 * 1_000;
const NANOS_PER_SECOND: i64 = 1_000 * NANOS_PER_MILLISECOND;

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

/// Rounds half away from zero, then reduces the result modulo 2^64 like a two's complement cast
/// of an arbitrarily wide integer, so a negative value counts back from the largest duration.
/// Values at or beyond 2^127 are multiples of 2^64, and NaN and the infinities have no integer
/// value, so all of them give 0.
fn round_wrapping(input: f64) -> u64 {
    const EXACT_LIMIT: f64 = i128::MAX as f64;
    let rounded = input.round();
    if rounded.is_finite() && rounded.abs() < EXACT_LIMIT {
        rounded as i128 as u64
    } else {
        0
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_TIME(input: f64) -> u32 {
    round_wrapping(input) as u32
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LTIME(input: f64) -> i64 {
    round_wrapping(input) as i64
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

    #[test]
    fn lreal_to_time_rounds_half_away_from_zero() {
        assert_eq!(0, LREAL_TO_TIME(0.499_999_97));
        assert_eq!(1, LREAL_TO_TIME(0.5));
        assert_eq!(2, LREAL_TO_TIME(1.5));
        assert_eq!(3, LREAL_TO_TIME(2.5));
        assert_eq!(0, LREAL_TO_TIME(-0.4));
        assert_eq!(u32::MAX, LREAL_TO_TIME(-0.5));
        assert_eq!(u32::MAX - 2, LREAL_TO_TIME(-2.5));
    }

    #[test]
    fn lreal_to_time_wraps_like_a_twos_complement_cast() {
        assert_eq!(4_294_966_296, LREAL_TO_TIME(-1000.0));
        assert_eq!(2_147_483_648, LREAL_TO_TIME(2_147_483_648.0));
        assert_eq!(4_294_967_040, LREAL_TO_TIME(4_294_967_040.0));
        assert_eq!(0, LREAL_TO_TIME(4_294_967_296.0));
        assert_eq!(1_410_065_408, LREAL_TO_TIME(1.0e10));
        assert_eq!(0, LREAL_TO_TIME(9_223_372_036_854_775_808.0));
        assert_eq!(0, LREAL_TO_TIME(-4_294_967_296.0));
    }

    #[test]
    fn lreal_to_ltime_rounds_half_away_from_zero() {
        assert_eq!(0, LREAL_TO_LTIME(0.499_999_97));
        assert_eq!(1, LREAL_TO_LTIME(0.5));
        assert_eq!(3, LREAL_TO_LTIME(2.5));
        assert_eq!(0, LREAL_TO_LTIME(-0.4));
        assert_eq!(-1, LREAL_TO_LTIME(-0.5));
        assert_eq!(-3, LREAL_TO_LTIME(-2.5));
    }

    #[test]
    fn lreal_to_ltime_wraps_like_a_twos_complement_cast() {
        assert_eq!(-1000, LREAL_TO_LTIME(-1000.0));
        assert_eq!(-4_294_967_296, LREAL_TO_LTIME(-4_294_967_296.0));
        assert_eq!(10_000_000_000, LREAL_TO_LTIME(1.0e10));
        assert_eq!(9_223_371_487_098_961_920, LREAL_TO_LTIME(9.223_371_487_098_961_920e18));
        assert_eq!(i64::MIN, LREAL_TO_LTIME(9_223_372_036_854_775_808.0));
        assert_eq!(18_446_742_974_197_923_840_u64 as i64, LREAL_TO_LTIME(1.844_674_297_419_792_384e19));
        assert_eq!(0, LREAL_TO_LTIME(18_446_744_073_709_551_616.0));
        assert_eq!(4_096, LREAL_TO_LTIME(18_446_744_073_709_555_712.0));
    }

    #[test]
    fn lreal_to_duration_gives_zero_for_values_without_an_exact_integer() {
        assert_eq!(0, LREAL_TO_TIME(3.4e38));
        assert_eq!(0, LREAL_TO_TIME(f64::MAX));
        assert_eq!(0, LREAL_TO_TIME(f64::NAN));
        assert_eq!(0, LREAL_TO_TIME(f64::INFINITY));
        assert_eq!(0, LREAL_TO_TIME(f64::NEG_INFINITY));
        assert_eq!(0, LREAL_TO_LTIME(3.4e38));
        assert_eq!(0, LREAL_TO_LTIME(-f64::MAX));
        assert_eq!(0, LREAL_TO_LTIME(f64::NAN));
        assert_eq!(0, LREAL_TO_LTIME(f64::INFINITY));
        assert_eq!(0, LREAL_TO_LTIME(f64::NEG_INFINITY));
    }
}
