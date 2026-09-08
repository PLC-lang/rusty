//! Duration literals (`TIME`, `LTIME`): the body after the `#`.

use crate::{digit_values, is_whitespace, split_digit_group};

/// A parsed duration. `nanos` is the magnitude; the sign is kept separately so that callers can
/// decide what a negative duration means for their target type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration {
    pub negative: bool,
    pub nanos: u64,
}

impl Duration {
    /// The duration as signed nanoseconds, or `None` if the magnitude exceeds `i64`.
    pub fn signed_nanos(&self) -> Option<i64> {
        let nanos = i64::try_from(self.nanos).ok()?;
        Some(if self.negative { -nanos } else { nanos })
    }
}

/// The segment units of a duration literal, largest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

impl Unit {
    /// The unit expressed in nanoseconds as `mantissa * 10^power`.
    fn factor(self) -> (u128, u32) {
        match self {
            Unit::Day => (864, 11),
            Unit::Hour => (36, 11),
            Unit::Minute => (6, 10),
            Unit::Second => (1, 9),
            Unit::Millisecond => (1, 6),
            Unit::Microsecond => (1, 3),
            Unit::Nanosecond => (1, 0),
        }
    }

    fn from_symbol(symbol: &str) -> Option<Unit> {
        [
            ("d", Unit::Day),
            ("h", Unit::Hour),
            ("m", Unit::Minute),
            ("s", Unit::Second),
            ("ms", Unit::Millisecond),
            ("us", Unit::Microsecond),
            ("ns", Unit::Nanosecond),
        ]
        .into_iter()
        .find_map(|(name, unit)| symbol.eq_ignore_ascii_case(name).then_some(unit))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationError {
    /// No segment at all (`T#`, `T#-`).
    Empty,
    /// A decimal point without digits in front of or after it.
    MissingDigits,
    /// Digits that are not followed by a unit.
    MissingUnit,
    /// Letters that are not one of `d h m s ms us ns`; `start..end` is their byte range in the body.
    UnknownUnit { start: usize, end: usize },
    /// A unit larger than the previous one (strict mode only).
    SegmentOutOfOrder,
    /// A unit that already appeared (strict mode only).
    DuplicateSegment,
    /// Anything else, including whitespace or `_` where the leniency does not allow it.
    InvalidCharacter,
    /// More than `u64::MAX` nanoseconds.
    Overflow,
}

/// What the parser accepts beyond the literal grammar of the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Leniency {
    /// Whitespace and `_` may separate segments (they never split a number or a unit).
    pub separators: bool,
    /// Segments may repeat and appear in any order; their values are summed.
    pub unordered_segments: bool,
}

impl Leniency {
    /// Exactly the literal grammar: `d h m s ms us ns`, each at most once, in that order.
    pub const STRICT: Leniency = Leniency { separators: false, unordered_segments: false };
    /// Everything `STRICT` accepts, plus separated, repeated and unordered segments.
    pub const RUNTIME: Leniency = Leniency { separators: true, unordered_segments: true };
}

/// Parses the body of a duration literal: `[+-]? (digits [. digits] unit)+`, where digit groups
/// may contain single underscores and units are matched case-insensitively. Fractions are exact and
/// truncated below one nanosecond.
pub fn parse_duration(body: &str, leniency: Leniency) -> Result<Duration, DurationError> {
    let (negative, mut rest) = match body.as_bytes().first() {
        Some(b'-') => (true, &body[1..]),
        Some(b'+') => (false, &body[1..]),
        _ => (false, body),
    };
    let mut nanos: u128 = 0;
    let mut previous: Option<Unit> = None;

    loop {
        if leniency.separators && previous.is_some() {
            let trimmed = rest.trim_start_matches(|c: char| is_whitespace(c) || c == '_');
            if trimmed.is_empty() && !rest.is_empty() {
                return Err(DurationError::InvalidCharacter);
            }
            rest = trimmed;
        }
        if rest.is_empty() {
            break;
        }

        let (integer, after_integer) = split_digit_group(rest, 10);
        if integer.is_empty() {
            return Err(match after_integer.as_bytes()[0] {
                b'.' => DurationError::MissingDigits,
                _ => DurationError::InvalidCharacter,
            });
        }
        let (fraction, after_number) = match after_integer.strip_prefix('.') {
            Some(after_dot) => {
                let (fraction, after_fraction) = split_digit_group(after_dot, 10);
                if fraction.is_empty() {
                    return Err(DurationError::MissingDigits);
                }
                (fraction, after_fraction)
            }
            None => ("", after_integer),
        };

        let unit_len = after_number.bytes().take_while(u8::is_ascii_alphabetic).count();
        if unit_len == 0 {
            return Err(DurationError::MissingUnit);
        }
        let start = body.len() - after_number.len();
        let unit = Unit::from_symbol(&after_number[..unit_len])
            .ok_or(DurationError::UnknownUnit { start, end: start + unit_len })?;
        rest = &after_number[unit_len..];

        if let Some(previous) = previous.filter(|_| !leniency.unordered_segments) {
            if unit == previous {
                return Err(DurationError::DuplicateSegment);
            }
            if unit < previous {
                return Err(DurationError::SegmentOutOfOrder);
            }
        }
        previous = Some(unit);

        nanos = nanos.checked_add(segment_nanos(integer, fraction, unit)?).ok_or(DurationError::Overflow)?;
    }

    if previous.is_none() {
        return Err(DurationError::Empty);
    }
    let nanos = u64::try_from(nanos).map_err(|_| DurationError::Overflow)?;
    Ok(Duration { negative, nanos })
}

/// `integer.fraction` units in nanoseconds, truncated. With the unit being `mantissa * 10^power`
/// nanoseconds, the decimal point moves `power` places right; whatever fraction digits remain are
/// multiplied by `mantissa` with schoolbook carries from the right, which yields the exact floor.
fn segment_nanos(integer: &str, fraction: &str, unit: Unit) -> Result<u128, DurationError> {
    let (mantissa, power) = unit.factor();
    let mut fraction_digits = digit_values(fraction, 10);
    let shifted = (0..power).map(|_| fraction_digits.next().unwrap_or(0));
    let mut whole: u128 = 0;
    for digit in digit_values(integer, 10).chain(shifted) {
        whole = whole
            .checked_mul(10)
            .and_then(|it| it.checked_add(digit as u128))
            .ok_or(DurationError::Overflow)?;
    }
    let carry = fraction_digits.rev().fold(0u128, |carry, digit| (mantissa * digit as u128 + carry) / 10);
    whole.checked_mul(mantissa).and_then(|it| it.checked_add(carry)).ok_or(DurationError::Overflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NANOS_PER_MILLI: u64 = 1_000_000;
    const NANOS_PER_SECOND: u64 = 1_000 * NANOS_PER_MILLI;
    const NANOS_PER_MINUTE: u64 = 60 * NANOS_PER_SECOND;
    const NANOS_PER_HOUR: u64 = 60 * NANOS_PER_MINUTE;
    const NANOS_PER_DAY: u64 = 24 * NANOS_PER_HOUR;

    fn strict(body: &str) -> Result<Duration, DurationError> {
        parse_duration(body, Leniency::STRICT)
    }
    fn runtime(body: &str) -> Result<Duration, DurationError> {
        parse_duration(body, Leniency::RUNTIME)
    }
    fn positive(nanos: u64) -> Result<Duration, DurationError> {
        Ok(Duration { negative: false, nanos })
    }

    #[test]
    fn every_unit() {
        assert_eq!(strict("1d"), positive(NANOS_PER_DAY));
        assert_eq!(strict("1h"), positive(NANOS_PER_HOUR));
        assert_eq!(strict("1m"), positive(NANOS_PER_MINUTE));
        assert_eq!(strict("1s"), positive(NANOS_PER_SECOND));
        assert_eq!(strict("1ms"), positive(NANOS_PER_MILLI));
        assert_eq!(strict("1us"), positive(1_000));
        assert_eq!(strict("1ns"), positive(1));
        assert_eq!(strict("1D2H3M4S5MS6US7NS"), strict("1d2h3m4s5ms6us7ns"));
        assert_eq!(
            strict("1d2h3m4s5ms6us7ns"),
            positive(
                NANOS_PER_DAY + 2 * NANOS_PER_HOUR + 3 * NANOS_PER_MINUTE + 4 * NANOS_PER_SECOND + 5_006_007
            )
        );
    }

    #[test]
    fn signs() {
        assert_eq!(strict("-1s"), Ok(Duration { negative: true, nanos: NANOS_PER_SECOND }));
        assert_eq!(strict("+1s"), positive(NANOS_PER_SECOND));
        assert_eq!(strict("-1s").unwrap().signed_nanos(), Some(-(NANOS_PER_SECOND as i64)));
        assert_eq!(strict("+-1s"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("1s+1s"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("-"), Err(DurationError::Empty));
    }

    #[test]
    fn fractions_are_exact_and_truncate() {
        assert_eq!(strict("1.5s"), positive(1_500 * NANOS_PER_MILLI));
        assert_eq!(strict("0.5s"), positive(500 * NANOS_PER_MILLI));
        assert_eq!(strict("1.5h"), positive(90 * NANOS_PER_MINUTE));
        assert_eq!(strict("1.5h30m"), positive(120 * NANOS_PER_MINUTE));
        assert_eq!(strict("1.0004ms"), positive(1_000_400));
        assert_eq!(strict("0.000000000001d"), positive(86));
        assert_eq!(strict("0.9999999999999999999999999999s"), positive(999_999_999));
        assert_eq!(strict("1.5ns"), positive(1));
        assert_eq!(strict("0.1us"), positive(100));
        assert_eq!(strict("2.75s"), positive(2_750 * NANOS_PER_MILLI));
        assert_eq!(strict("1.s"), Err(DurationError::MissingDigits));
        assert_eq!(strict(".5s"), Err(DurationError::MissingDigits));
        assert_eq!(strict("1,5s"), Err(DurationError::MissingUnit));
    }

    #[test]
    fn digit_groups_take_underscores() {
        assert_eq!(strict("1_000ms"), positive(NANOS_PER_SECOND));
        assert_eq!(strict("1_000.000_5ms"), positive(1_000_000_500));
        assert_eq!(strict("1__0ms"), Err(DurationError::MissingUnit));
        assert_eq!(strict("_1ms"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("1_ms"), Err(DurationError::MissingUnit));
    }

    #[test]
    fn strict_order_and_uniqueness() {
        assert_eq!(strict("1s1ms"), positive(1_001 * NANOS_PER_MILLI));
        assert_eq!(strict("1ms1s"), Err(DurationError::SegmentOutOfOrder));
        assert_eq!(strict("30m1h"), Err(DurationError::SegmentOutOfOrder));
        assert_eq!(strict("1h1h"), Err(DurationError::DuplicateSegment));
        assert_eq!(strict("1h 30m"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("1h_30m"), Err(DurationError::InvalidCharacter));
    }

    #[test]
    fn runtime_leniencies() {
        assert_eq!(runtime("30m1h"), positive(90 * NANOS_PER_MINUTE));
        assert_eq!(runtime("1h1h"), positive(2 * NANOS_PER_HOUR));
        assert_eq!(runtime("1ms1s"), positive(1_001 * NANOS_PER_MILLI));
        assert_eq!(runtime("1h 30m"), positive(90 * NANOS_PER_MINUTE));
        assert_eq!(runtime("1h_30m"), positive(90 * NANOS_PER_MINUTE));
        assert_eq!(runtime("1h\t\x0B 30m"), positive(90 * NANOS_PER_MINUTE));
        assert_eq!(runtime("1h__30m"), positive(90 * NANOS_PER_MINUTE));
        assert_eq!(runtime(" 1s"), Err(DurationError::InvalidCharacter));
        assert_eq!(runtime("1s "), Err(DurationError::InvalidCharacter));
        assert_eq!(runtime("1s_"), Err(DurationError::InvalidCharacter));
        assert_eq!(runtime("1 0s"), Err(DurationError::MissingUnit));
        assert_eq!(runtime("1 s"), Err(DurationError::MissingUnit));
        assert_eq!(runtime("1h§30m"), Err(DurationError::InvalidCharacter));
    }

    #[test]
    fn malformed_segments() {
        assert_eq!(strict(""), Err(DurationError::Empty));
        assert_eq!(strict("1"), Err(DurationError::MissingUnit));
        assert_eq!(strict("1s5"), Err(DurationError::MissingUnit));
        assert_eq!(strict("1x"), Err(DurationError::UnknownUnit { start: 1, end: 2 }));
        assert_eq!(strict("1sx"), Err(DurationError::UnknownUnit { start: 1, end: 3 }));
        assert_eq!(strict("1h30mo"), Err(DurationError::UnknownUnit { start: 4, end: 6 }));
        assert_eq!(strict("1s§"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("1s//x"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("1s(*x*)"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("1s{x}"), Err(DurationError::InvalidCharacter));
        assert_eq!(strict("§"), Err(DurationError::InvalidCharacter));
    }

    #[test]
    fn range() {
        assert_eq!(strict("5000000000ns"), positive(5 * NANOS_PER_SECOND));
        assert_eq!(strict("106751d23h47m16s854ms775us807ns"), positive(i64::MAX as u64));
        assert_eq!(strict("106751d23h47m16s854ms775us807ns").unwrap().signed_nanos(), Some(i64::MAX));
        assert_eq!(strict("106751d23h47m16s854ms775us808ns").unwrap().signed_nanos(), None);
        assert_eq!(strict("18446744073709551615ns"), positive(u64::MAX));
        assert_eq!(strict("18446744073709551616ns"), Err(DurationError::Overflow));
        assert_eq!(strict("213504d"), Err(DurationError::Overflow));
        assert_eq!(strict("99999999999999999999999999999999999999999d"), Err(DurationError::Overflow));
        assert_eq!(runtime("18446744073709551615ns1ns"), Err(DurationError::Overflow));
    }
}
