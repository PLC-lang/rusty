//! Calendar literals (`DATE`, `TIME_OF_DAY`, `DATE_AND_TIME`): the body after the `#`.
//!
//! Parsing only checks the shape (`digits-digits-digits`, `digits:digits[:digits[.digits]]`);
//! whether the fields form a real calendar day or time of day is answered by the value methods,
//! so the compiler can keep reporting syntax and range problems separately.

const SECONDS_PER_DAY: i64 = 86_400;
const NANOS_PER_SECOND: i64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeOfDay {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
    pub nano: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateAndTime {
    pub date: Date,
    pub time: TimeOfDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarError {
    /// The input ended where another field was required (`2024-01`, `12:`, `12:00:00.`).
    MissingField,
    /// A character outside the literal's shape, including a sign or a second decimal point.
    InvalidCharacter,
    /// A field's digits do not fit its type.
    Overflow,
}

/// Parses `year-month-day`.
pub fn parse_date(body: &str) -> Result<Date, CalendarError> {
    let (date, rest) = date_prefix(body)?;
    end(rest)?;
    Ok(date)
}

/// Parses `hour:min[:sec[.fraction]]`. The fraction is truncated to whole nanoseconds.
pub fn parse_time_of_day(body: &str) -> Result<TimeOfDay, CalendarError> {
    let (time, rest) = time_prefix(body)?;
    end(rest)?;
    Ok(time)
}

/// Parses `year-month-day-hour:min[:sec[.fraction]]`.
pub fn parse_date_and_time(body: &str) -> Result<DateAndTime, CalendarError> {
    let (date, rest) = date_prefix(body)?;
    let rest = separator(rest, b'-')?;
    let (time, rest) = time_prefix(rest)?;
    end(rest)?;
    Ok(DateAndTime { date, time })
}

impl Date {
    /// Whether the fields name a day of the proleptic Gregorian calendar.
    pub fn is_valid(&self) -> bool {
        (1..=12).contains(&self.month) && (1..=days_in_month(self.year, self.month)).contains(&self.day)
    }

    /// Days since 1970-01-01, or `None` for an invalid date.
    pub fn days_since_epoch(&self) -> Option<i64> {
        self.is_valid().then(|| days_from_civil(self.year as i64, self.month, self.day))
    }

    /// Seconds since 1970-01-01T00:00:00, or `None` for an invalid date.
    pub fn seconds_since_epoch(&self) -> Option<i64> {
        self.days_since_epoch()?.checked_mul(SECONDS_PER_DAY)
    }

    /// Nanoseconds since 1970-01-01T00:00:00, or `None` for an invalid or unrepresentable date.
    pub fn nanos_since_epoch(&self) -> Option<i64> {
        self.seconds_since_epoch()?.checked_mul(NANOS_PER_SECOND)
    }
}

impl TimeOfDay {
    /// Whether the fields name a time within a day.
    pub fn is_valid(&self) -> bool {
        self.hour < 24 && self.min < 60 && self.sec < 60 && (self.nano as i64) < NANOS_PER_SECOND
    }

    /// Seconds since midnight, or `None` for an invalid time.
    pub fn seconds(&self) -> Option<u32> {
        self.is_valid().then(|| self.hour * 3_600 + self.min * 60 + self.sec)
    }

    /// Nanoseconds since midnight, or `None` for an invalid time.
    pub fn nanos(&self) -> Option<u64> {
        Some(self.seconds()? as u64 * NANOS_PER_SECOND as u64 + self.nano as u64)
    }
}

impl DateAndTime {
    /// Seconds since 1970-01-01T00:00:00, or `None` if either part is invalid.
    pub fn seconds_since_epoch(&self) -> Option<i64> {
        self.date.seconds_since_epoch()?.checked_add(self.time.seconds()? as i64)
    }

    /// Nanoseconds since 1970-01-01T00:00:00, or `None` if either part is invalid or the
    /// result is unrepresentable.
    pub fn nanos_since_epoch(&self) -> Option<i64> {
        let seconds = self.date.seconds_since_epoch()?.checked_add(self.time.seconds()? as i64)?;
        // combine in i128 so the boundary values of i64 stay reachable
        let nanos = seconds as i128 * NANOS_PER_SECOND as i128 + self.time.nano as i128;
        i64::try_from(nanos).ok()
    }
}

fn date_prefix(body: &str) -> Result<(Date, &str), CalendarError> {
    let (year, rest) = field(body)?;
    let year = i32::try_from(year).map_err(|_| CalendarError::Overflow)?;
    let rest = separator(rest, b'-')?;
    let (month, rest) = field(rest)?;
    let rest = separator(rest, b'-')?;
    let (day, rest) = field(rest)?;
    Ok((Date { year, month, day }, rest))
}

fn time_prefix(body: &str) -> Result<(TimeOfDay, &str), CalendarError> {
    let (hour, rest) = field(body)?;
    let rest = separator(rest, b':')?;
    let (min, mut rest) = field(rest)?;
    let mut sec = 0;
    let mut nano = 0;
    if let Some(after_colon) = rest.strip_prefix(':') {
        (sec, rest) = field(after_colon)?;
        if let Some(after_dot) = rest.strip_prefix('.') {
            let digits = after_dot.bytes().take_while(u8::is_ascii_digit).count();
            if digits == 0 {
                return Err(if after_dot.is_empty() {
                    CalendarError::MissingField
                } else {
                    CalendarError::InvalidCharacter
                });
            }
            // the first nine digits are the nanoseconds, zero-padded; anything finer is dropped
            nano = after_dot[..digits]
                .bytes()
                .chain(core::iter::repeat(b'0'))
                .take(9)
                .fold(0u32, |acc, byte| acc * 10 + (byte - b'0') as u32);
            rest = &after_dot[digits..];
        }
    }
    Ok((TimeOfDay { hour, min, sec, nano }, rest))
}

/// Splits off a run of decimal digits and returns it as `u32`.
fn field(input: &str) -> Result<(u32, &str), CalendarError> {
    let digits = input.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return Err(if input.is_empty() {
            CalendarError::MissingField
        } else {
            CalendarError::InvalidCharacter
        });
    }
    let value = input[..digits]
        .bytes()
        .try_fold(0u32, |acc, byte| acc.checked_mul(10)?.checked_add((byte - b'0') as u32))
        .ok_or(CalendarError::Overflow)?;
    Ok((value, &input[digits..]))
}

fn separator(input: &str, expected: u8) -> Result<&str, CalendarError> {
    match input.as_bytes().first() {
        None => Err(CalendarError::MissingField),
        Some(byte) if *byte == expected => Ok(&input[1..]),
        Some(_) => Err(CalendarError::InvalidCharacter),
    }
}

fn end(rest: &str) -> Result<(), CalendarError> {
    rest.is_empty().then_some(()).ok_or(CalendarError::InvalidCharacter)
}

fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year as i64) => 29,
        2 => 28,
        _ => 0,
    }
}

/// Days from 1970-01-01 to the given proleptic Gregorian date (Howard Hinnant's algorithm).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = (year - era * 400) as u64;
    let shifted_month = if month > 2 { month - 3 } else { month + 9 } as u64;
    let day_of_year = (153 * shifted_month + 2) / 5 + day as u64 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era as i64 - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> Date {
        Date { year, month, day }
    }
    fn time(hour: u32, min: u32, sec: u32, nano: u32) -> TimeOfDay {
        TimeOfDay { hour, min, sec, nano }
    }

    #[test]
    fn date_shape() {
        assert_eq!(parse_date("2024-01-01"), Ok(date(2024, 1, 1)));
        assert_eq!(parse_date("2024-1-1"), Ok(date(2024, 1, 1)));
        assert_eq!(parse_date("0-0-0"), Ok(date(0, 0, 0)));
        assert_eq!(parse_date("2024-01"), Err(CalendarError::MissingField));
        assert_eq!(parse_date("2024-01-"), Err(CalendarError::MissingField));
        assert_eq!(parse_date(""), Err(CalendarError::MissingField));
        assert_eq!(parse_date("2024-01-01-"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("2024-01-01x"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("2024-01-01T00:00:00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("2024/01/01"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("2024--01"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("+2024-01-01"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("-2024-01-01"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("2024_000-01-01"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date(" 2024-01-01"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date("99999999999-01-01"), Err(CalendarError::Overflow));
        assert_eq!(parse_date("2147483648-01-01"), Err(CalendarError::Overflow));
        assert_eq!(parse_date("2147483647-01-01"), Ok(date(i32::MAX, 1, 1)));
    }

    #[test]
    fn time_shape() {
        assert_eq!(parse_time_of_day("12:00:00"), Ok(time(12, 0, 0, 0)));
        assert_eq!(parse_time_of_day("12:00"), Ok(time(12, 0, 0, 0)));
        assert_eq!(parse_time_of_day("1:2:3"), Ok(time(1, 2, 3, 0)));
        assert_eq!(parse_time_of_day("12:00:00.5"), Ok(time(12, 0, 0, 500_000_000)));
        assert_eq!(parse_time_of_day("12:00:00.500"), Ok(time(12, 0, 0, 500_000_000)));
        assert_eq!(parse_time_of_day("12:00:00.000000001"), Ok(time(12, 0, 0, 1)));
        assert_eq!(parse_time_of_day("12:00:00.0000000019"), Ok(time(12, 0, 0, 1)));
        assert_eq!(parse_time_of_day("23:59:59.9999999999"), Ok(time(23, 59, 59, 999_999_999)));
        assert_eq!(parse_time_of_day("12"), Err(CalendarError::MissingField));
        assert_eq!(parse_time_of_day("12:"), Err(CalendarError::MissingField));
        assert_eq!(parse_time_of_day("12:00:"), Err(CalendarError::MissingField));
        assert_eq!(parse_time_of_day("12:00:00."), Err(CalendarError::MissingField));
        assert_eq!(parse_time_of_day("12:00:00.x"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("12:00:00,5"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("12:00.5"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("12:00:00:00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("12.00.00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("-12:00:00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("12 :00:00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_time_of_day("4294967296:00:00"), Err(CalendarError::Overflow));
    }

    #[test]
    fn date_and_time_shape() {
        assert_eq!(
            parse_date_and_time("2024-01-01-12:00:00"),
            Ok(DateAndTime { date: date(2024, 1, 1), time: time(12, 0, 0, 0) })
        );
        assert_eq!(
            parse_date_and_time("2024-1-1-1:2"),
            Ok(DateAndTime { date: date(2024, 1, 1), time: time(1, 2, 0, 0) })
        );
        assert_eq!(
            parse_date_and_time("2024-01-01-12:00:00.25"),
            Ok(DateAndTime { date: date(2024, 1, 1), time: time(12, 0, 0, 250_000_000) })
        );
        assert_eq!(parse_date_and_time("2024-01-01"), Err(CalendarError::MissingField));
        assert_eq!(parse_date_and_time("2024-01-01-12"), Err(CalendarError::MissingField));
        assert_eq!(parse_date_and_time("2024-01-01T12:00:00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date_and_time("2024-01-01 12:00:00"), Err(CalendarError::InvalidCharacter));
        assert_eq!(parse_date_and_time("2024-01-01-12:00:00x"), Err(CalendarError::InvalidCharacter));
    }

    #[test]
    fn calendar_validity() {
        assert!(date(2024, 2, 29).is_valid());
        assert!(!date(2023, 2, 29).is_valid());
        assert!(date(2000, 2, 29).is_valid());
        assert!(!date(1900, 2, 29).is_valid());
        assert!(!date(2100, 2, 29).is_valid());
        assert!(!date(2024, 2, 30).is_valid());
        assert!(!date(2024, 4, 31).is_valid());
        assert!(!date(2024, 13, 1).is_valid());
        assert!(!date(2024, 0, 1).is_valid());
        assert!(!date(2024, 1, 0).is_valid());
        assert!(time(23, 59, 59, 999_999_999).is_valid());
        assert!(!time(24, 0, 0, 0).is_valid());
        assert!(!time(12, 60, 0, 0).is_valid());
        assert!(!time(12, 0, 60, 0).is_valid());
        assert!(!time(0, 0, 0, 1_000_000_000).is_valid());
    }

    #[test]
    fn epoch_arithmetic() {
        assert_eq!(date(1970, 1, 1).days_since_epoch(), Some(0));
        assert_eq!(date(1970, 1, 2).days_since_epoch(), Some(1));
        assert_eq!(date(1969, 12, 31).days_since_epoch(), Some(-1));
        assert_eq!(date(2024, 1, 1).seconds_since_epoch(), Some(1_704_067_200));
        assert_eq!(date(2000, 2, 29).seconds_since_epoch(), Some(951_782_400));
        assert_eq!(date(2096, 2, 29).seconds_since_epoch(), Some(3_981_312_000));
        assert_eq!(date(2106, 2, 7).seconds_since_epoch(), Some(4_294_944_000));
        assert_eq!(date(1, 1, 1).days_since_epoch(), Some(-719_162));
        assert_eq!(date(0, 1, 1).days_since_epoch(), Some(-719_528));
        assert_eq!(date(-1, 12, 31).days_since_epoch(), Some(-719_529));
        assert_eq!(date(9999, 12, 31).seconds_since_epoch(), Some(253_402_214_400));
        assert_eq!(date(2024, 2, 30).days_since_epoch(), None);
        assert_eq!(date(1969, 12, 31).nanos_since_epoch(), Some(-86_400 * NANOS_PER_SECOND));
        assert_eq!(date(2262, 4, 11).nanos_since_epoch(), Some(9_223_286_400_000_000_000));
        assert_eq!(date(2262, 4, 12).nanos_since_epoch(), None);
        assert_eq!(date(i32::MAX, 1, 1).nanos_since_epoch(), None);
        assert_eq!(date(i32::MIN, 1, 1).nanos_since_epoch(), None);
    }

    #[test]
    fn time_of_day_arithmetic() {
        assert_eq!(time(12, 0, 0, 0).nanos(), Some(43_200 * NANOS_PER_SECOND as u64));
        assert_eq!(time(23, 59, 59, 999_999_999).nanos(), Some(86_400 * NANOS_PER_SECOND as u64 - 1));
        assert_eq!(time(24, 0, 0, 0).nanos(), None);
        assert_eq!(
            DateAndTime { date: date(2106, 2, 7), time: time(6, 28, 15, 0) }.seconds_since_epoch(),
            Some(u32::MAX as i64)
        );
        assert_eq!(
            DateAndTime { date: date(2024, 1, 1), time: time(12, 0, 0, 500_000_000) }.nanos_since_epoch(),
            Some(1_704_110_400 * NANOS_PER_SECOND + 500_000_000)
        );
        assert_eq!(DateAndTime { date: date(2024, 1, 1), time: time(25, 0, 0, 0) }.nanos_since_epoch(), None);
        assert_eq!(
            DateAndTime { date: date(1677, 9, 21), time: time(0, 12, 43, 145_224_192) }.nanos_since_epoch(),
            Some(i64::MIN)
        );
        assert_eq!(
            DateAndTime { date: date(1677, 9, 21), time: time(0, 12, 43, 145_224_191) }.nanos_since_epoch(),
            None
        );
        assert_eq!(
            DateAndTime { date: date(2262, 4, 11), time: time(23, 47, 16, 854_775_807) }.nanos_since_epoch(),
            Some(i64::MAX)
        );
        assert_eq!(
            DateAndTime { date: date(2262, 4, 11), time: time(23, 47, 16, 854_775_808) }.nanos_since_epoch(),
            None
        );
        assert_eq!(
            DateAndTime { date: date(2024, 2, 30), time: time(12, 0, 0, 0) }.nanos_since_epoch(),
            None
        );
    }
}
