//! Integer, real and boolean literals.

use crate::{digit_group_value, digit_values, split_digit_group};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegerError {
    /// A digit group is empty where one is required (`16#`, `123.`, `.5`, `1e`).
    MissingDigits,
    /// A character that is neither a digit of the radix nor part of the grammar.
    InvalidCharacter,
    /// `_` at the start or end of a digit group, or two in a row.
    MisplacedUnderscore,
    /// A sign in front of a based literal (`-16#FF`); based literals are unsigned.
    SignedBasedLiteral,
    /// The value does not fit in `i128`.
    Overflow,
}

/// Parses an integer literal: `[+-]? decimal` or `(2|8|16)# digits`, with single underscores
/// allowed between digits. The whole input must be consumed.
pub fn parse_integer(input: &str) -> Result<i128, IntegerError> {
    let (negative, unsigned) = split_sign(input);
    if let Some((radix, digits)) = split_radix_prefix(unsigned) {
        if unsigned.len() != input.len() {
            return Err(IntegerError::SignedBasedLiteral);
        }
        let value = parse_digit_group(digits, radix)?;
        return i128::try_from(value).map_err(|_| IntegerError::Overflow);
    }
    let magnitude = parse_digit_group(unsigned, 10)?;
    apply_sign(magnitude, negative)
}

/// Parses a decimal real literal (`[+-]? digits [. digits] [(e|E) [+-]? digits]`) and returns its
/// integer part, truncated toward zero. A plain decimal integer is accepted as well, so this is
/// the one entry point for "a number, as the compiler would assign it to an integer".
pub fn parse_real_as_integer(input: &str) -> Result<i128, IntegerError> {
    let (negative, unsigned) = split_sign(input);
    let (mantissa, exponent) = match unsigned.find(['e', 'E']) {
        Some(index) => (&unsigned[..index], Some(&unsigned[index + 1..])),
        None => (unsigned, None),
    };
    let (integer, fraction) = match mantissa.split_once('.') {
        Some((integer, fraction)) => (integer, fraction),
        None => (mantissa, ""),
    };
    check_digit_group(integer, 10)?;
    if mantissa.contains('.') {
        check_digit_group(fraction, 10)?;
    }
    let exponent = match exponent {
        Some(exponent) => parse_exponent(exponent)?,
        None => 0,
    };

    // The digits of `integer` and `fraction` form one stream with the decimal point after
    // `integer.len() + exponent` of them; the truncated value is that many leading digits.
    let digits = digit_values(integer, 10).chain(digit_values(fraction, 10));
    let integer_digits = digit_values(integer, 10).count() as i64 + exponent;
    if integer_digits <= 0 || digits.clone().all(|digit| digit == 0) {
        return Ok(0);
    }
    let mut magnitude = 0u128;
    let mut taken = 0i64;
    for digit in digits.take(integer_digits as usize) {
        magnitude = magnitude
            .checked_mul(10)
            .and_then(|it| it.checked_add(digit as u128))
            .ok_or(IntegerError::Overflow)?;
        taken += 1;
    }
    // digits the exponent asks for beyond what was written are zeros
    for _ in taken..integer_digits {
        magnitude = magnitude.checked_mul(10).ok_or(IntegerError::Overflow)?;
    }
    apply_sign(magnitude, negative)
}

/// Parses a boolean literal or its numeric spelling: `TRUE`/`FALSE` (any case), `1`/`0`.
pub fn parse_bool(input: &str) -> Option<bool> {
    match input {
        "1" => Some(true),
        "0" => Some(false),
        _ if input.eq_ignore_ascii_case("TRUE") => Some(true),
        _ if input.eq_ignore_ascii_case("FALSE") => Some(false),
        _ => None,
    }
}

fn split_sign(input: &str) -> (bool, &str) {
    if let Some(rest) = input.strip_prefix('-') {
        (true, rest)
    } else if let Some(rest) = input.strip_prefix('+') {
        (false, rest)
    } else {
        (false, input)
    }
}

fn split_radix_prefix(input: &str) -> Option<(u32, &str)> {
    [("16#", 16), ("8#", 8), ("2#", 2)]
        .into_iter()
        .find_map(|(prefix, radix)| input.strip_prefix(prefix).map(|digits| (radix, digits)))
}

fn apply_sign(magnitude: u128, negative: bool) -> Result<i128, IntegerError> {
    if negative {
        if magnitude == i128::MIN.unsigned_abs() {
            return Ok(i128::MIN);
        }
        i128::try_from(magnitude).map(|it| -it).map_err(|_| IntegerError::Overflow)
    } else {
        i128::try_from(magnitude).map_err(|_| IntegerError::Overflow)
    }
}

/// Requires `input` to be exactly one digit group of `radix`.
fn check_digit_group(input: &str, radix: u32) -> Result<(), IntegerError> {
    let (group, rest) = split_digit_group(input, radix);
    if group.is_empty() {
        return Err(match input.chars().next() {
            None => IntegerError::MissingDigits,
            Some('_') => IntegerError::MisplacedUnderscore,
            Some(_) => IntegerError::InvalidCharacter,
        });
    }
    if !rest.is_empty() {
        return Err(if rest.starts_with('_') {
            IntegerError::MisplacedUnderscore
        } else {
            IntegerError::InvalidCharacter
        });
    }
    Ok(())
}

fn parse_digit_group(input: &str, radix: u32) -> Result<u128, IntegerError> {
    check_digit_group(input, radix)?;
    digit_group_value(input, radix).ok_or(IntegerError::Overflow)
}

/// Parses `[+-]? digits` into an exponent. Values beyond `i32` are clamped: with a non-zero
/// mantissa they overflow (positive) or truncate to zero (negative) anyway.
fn parse_exponent(input: &str) -> Result<i64, IntegerError> {
    let (negative, digits) = split_sign(input);
    if digits.is_empty() {
        return Err(IntegerError::MissingDigits);
    }
    if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(IntegerError::InvalidCharacter);
    }
    let magnitude = digit_group_value(digits, 10).unwrap_or(u128::MAX).min(i32::MAX as u128) as i64;
    Ok(if negative { -magnitude } else { magnitude })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_integers() {
        assert_eq!(parse_integer("12"), Ok(12));
        assert_eq!(parse_integer("+12"), Ok(12));
        assert_eq!(parse_integer("-12"), Ok(-12));
        assert_eq!(parse_integer("007"), Ok(7));
        assert_eq!(parse_integer("-0"), Ok(0));
        assert_eq!(parse_integer("1_000_000"), Ok(1_000_000));
        assert_eq!(parse_integer(""), Err(IntegerError::MissingDigits));
        assert_eq!(parse_integer("-"), Err(IntegerError::MissingDigits));
        assert_eq!(parse_integer("--1"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("- 1"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("12abc"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("12 34"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("1.9"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("1e3"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("_1"), Err(IntegerError::MisplacedUnderscore));
        assert_eq!(parse_integer("1_"), Err(IntegerError::MisplacedUnderscore));
        assert_eq!(parse_integer("1__0"), Err(IntegerError::MisplacedUnderscore));
    }

    #[test]
    fn based_integers() {
        assert_eq!(parse_integer("16#FF"), Ok(255));
        assert_eq!(parse_integer("16#ff"), Ok(255));
        assert_eq!(parse_integer("16#F_F"), Ok(255));
        assert_eq!(parse_integer("8#77"), Ok(63));
        assert_eq!(parse_integer("2#1111"), Ok(15));
        assert_eq!(parse_integer("16#FFFFFFFFFFFFFFFF"), Ok(u64::MAX as i128));
        assert_eq!(parse_integer("16#"), Err(IntegerError::MissingDigits));
        assert_eq!(parse_integer("16#G"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("16#FG"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("8#19"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("2#12"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("10#12"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("#FF"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("0xFF"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_integer("16#_FF"), Err(IntegerError::MisplacedUnderscore));
        assert_eq!(parse_integer("16#FF_"), Err(IntegerError::MisplacedUnderscore));
        assert_eq!(parse_integer("16#F__F"), Err(IntegerError::MisplacedUnderscore));
        assert_eq!(parse_integer("-16#FF"), Err(IntegerError::SignedBasedLiteral));
        assert_eq!(parse_integer("+16#FF"), Err(IntegerError::SignedBasedLiteral));
        assert_eq!(parse_integer("16#-FF"), Err(IntegerError::InvalidCharacter));
    }

    #[test]
    fn integer_range_is_i128() {
        assert_eq!(parse_integer("170141183460469231731687303715884105727"), Ok(i128::MAX));
        assert_eq!(parse_integer("170141183460469231731687303715884105728"), Err(IntegerError::Overflow));
        assert_eq!(parse_integer("-170141183460469231731687303715884105728"), Ok(i128::MIN));
        assert_eq!(parse_integer("-170141183460469231731687303715884105729"), Err(IntegerError::Overflow));
        assert_eq!(parse_integer("16#FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"), Err(IntegerError::Overflow));
        assert_eq!(
            parse_integer("99999999999999999999999999999999999999999999"),
            Err(IntegerError::Overflow)
        );
    }

    #[test]
    fn reals_truncate_toward_zero() {
        assert_eq!(parse_real_as_integer("12"), Ok(12));
        assert_eq!(parse_real_as_integer("1.9"), Ok(1));
        assert_eq!(parse_real_as_integer("-1.9"), Ok(-1));
        assert_eq!(parse_real_as_integer("1.0"), Ok(1));
        assert_eq!(parse_real_as_integer("0.999"), Ok(0));
        assert_eq!(parse_real_as_integer("1e3"), Ok(1_000));
        assert_eq!(parse_real_as_integer("1E3"), Ok(1_000));
        assert_eq!(parse_real_as_integer("1e+3"), Ok(1_000));
        assert_eq!(parse_real_as_integer("1.5e2"), Ok(150));
        assert_eq!(parse_real_as_integer("1.55e1"), Ok(15));
        assert_eq!(parse_real_as_integer("123.456e-1"), Ok(12));
        assert_eq!(parse_real_as_integer("1e-3"), Ok(0));
        assert_eq!(parse_real_as_integer("0e999999999999"), Ok(0));
        assert_eq!(parse_real_as_integer("0.0e-999999999999"), Ok(0));
        assert_eq!(parse_real_as_integer("1e-999999999999"), Ok(0));
        assert_eq!(parse_real_as_integer("1_000.5"), Ok(1_000));
        assert_eq!(parse_real_as_integer("1.000_5"), Ok(1));
        assert_eq!(parse_real_as_integer("1e38"), Ok(100_000_000_000_000_000_000_000_000_000_000_000_000));
    }

    #[test]
    fn real_grammar_is_strict() {
        assert_eq!(parse_real_as_integer("123."), Err(IntegerError::MissingDigits));
        assert_eq!(parse_real_as_integer(".5"), Err(IntegerError::MissingDigits));
        assert_eq!(parse_real_as_integer("1e"), Err(IntegerError::MissingDigits));
        assert_eq!(parse_real_as_integer("e3"), Err(IntegerError::MissingDigits));
        assert_eq!(parse_real_as_integer("1.9.9"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_real_as_integer("1.5e2.5"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_real_as_integer("1,5"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_real_as_integer("1e3_0"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_real_as_integer("16#FF"), Err(IntegerError::InvalidCharacter));
        assert_eq!(parse_real_as_integer("1e39"), Err(IntegerError::Overflow));
        assert_eq!(parse_real_as_integer("1e999999999999"), Err(IntegerError::Overflow));
    }

    #[test]
    fn booleans() {
        assert_eq!(parse_bool("TRUE"), Some(true));
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("True"), Some(true));
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("FALSE"), Some(false));
        assert_eq!(parse_bool("0"), Some(false));
        assert_eq!(parse_bool("TRUEX"), None);
        assert_eq!(parse_bool("2"), None);
        assert_eq!(parse_bool("01"), None);
        assert_eq!(parse_bool("T"), None);
        assert_eq!(parse_bool(""), None);
    }
}
