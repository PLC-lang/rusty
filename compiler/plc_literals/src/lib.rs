//! Allocation-free parsers for the Structured Text literal grammar.
//!
//! The compiler's lexer has already classified a token when these functions run; the standard
//! library's `STRING_TO_*` functions call them on raw runtime input. Both sides share one
//! implementation, so a literal that compiles to a value converts to the same value at runtime.
//!
//! Nothing here allocates or touches floating point: every result is exact.
#![no_std]

pub mod calendar;
pub mod duration;
pub mod integer;

pub use calendar::{
    parse_date, parse_date_and_time, parse_time_of_day, CalendarError, Date, DateAndTime, TimeOfDay,
};
pub use duration::{parse_duration, Duration, DurationError, Leniency, Unit};
pub use integer::{parse_bool, parse_integer, parse_real_as_integer, IntegerError};

/// The whitespace Structured Text treats as insignificant: space, tab, CR, LF, VT, FF.
pub fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n' | '\x0B' | '\x0C')
}

/// Trims [`is_whitespace`] characters from both ends of `input`.
pub fn trim(input: &str) -> &str {
    input.trim_matches(is_whitespace)
}

/// Strips the first of `prefixes` that `input` starts with (ASCII case-insensitively) and returns
/// the remainder, or `None` if no prefix matches.
pub fn strip_prefix_ignore_ascii_case<'a>(input: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    prefixes.iter().find_map(|prefix| {
        let head = input.get(..prefix.len())?;
        head.eq_ignore_ascii_case(prefix).then_some(&input[prefix.len()..])
    })
}

/// Splits off the longest leading digit group `digit+ ('_' digit+)*` for `radix` and returns
/// `(group, rest)`. `group` is empty when `input` does not start with a digit; a trailing `_`
/// is never part of the group.
pub(crate) fn split_digit_group(input: &str, radix: u32) -> (&str, &str) {
    let bytes = input.as_bytes();
    let mut end = 0;
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if (byte as char).is_digit(radix) {
            index += 1;
            end = index;
        } else if byte == b'_' && end == index && end > 0 {
            // an underscore may follow a digit, but only counts once the next digit arrives
            index += 1;
        } else {
            break;
        }
    }
    input.split_at(end)
}

/// Iterates the digit values of a group produced by [`split_digit_group`], skipping underscores.
pub(crate) fn digit_values(group: &str, radix: u32) -> impl DoubleEndedIterator<Item = u32> + Clone + '_ {
    group.chars().filter(|c| *c != '_').map(move |c| c.to_digit(radix).unwrap_or(0))
}

/// Accumulates a digit group into a `u128`, or `None` on overflow.
pub(crate) fn digit_group_value(group: &str, radix: u32) -> Option<u128> {
    digit_values(group, radix)
        .try_fold(0u128, |acc, digit| acc.checked_mul(radix as u128)?.checked_add(digit as u128))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit_groups_stop_at_misplaced_underscores() {
        assert_eq!(split_digit_group("1_000ms", 10), ("1_000", "ms"));
        assert_eq!(split_digit_group("1__0", 10), ("1", "__0"));
        assert_eq!(split_digit_group("1_", 10), ("1", "_"));
        assert_eq!(split_digit_group("_1", 10), ("", "_1"));
        assert_eq!(split_digit_group("F_F", 16), ("F_F", ""));
        assert_eq!(split_digit_group("19", 8), ("1", "9"));
        assert_eq!(split_digit_group("", 10), ("", ""));
    }

    #[test]
    fn prefixes_match_case_insensitively() {
        assert_eq!(strip_prefix_ignore_ascii_case("time#1s", &["TIME#", "T#"]), Some("1s"));
        assert_eq!(strip_prefix_ignore_ascii_case("T#1s", &["TIME#", "T#"]), Some("1s"));
        assert_eq!(strip_prefix_ignore_ascii_case("LT#1s", &["TIME#", "T#"]), None);
        assert_eq!(strip_prefix_ignore_ascii_case("T", &["TIME#", "T#"]), None);
        assert_eq!(strip_prefix_ignore_ascii_case("§#", &["T#"]), None);
    }

    #[test]
    fn trim_covers_the_structured_text_whitespace_set() {
        assert_eq!(trim(" \t\r\n\x0B\x0Cx\x0C\x0B\n\r\t "), "x");
        assert_eq!(trim("\u{A0}x"), "\u{A0}x");
    }
}
