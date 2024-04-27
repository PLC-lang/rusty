use std::fmt::{Debug, Formatter};

use chrono::NaiveDate;

use crate::ast::AstNode;

macro_rules! impl_getters {
    ($type:ty, [$($name:ident),+], [$($out:ty),+]) => {
        $(impl $type {
            pub fn $name(&self) -> $out {
                self.$name
            }
        })*
    }
}

#[derive(Clone, PartialEq)]
pub enum AstLiteral {
    /// a null literal used to initialize pointers
    Null,
    /// a literal that represents a whole number (e.g. 7)
    Integer(i128),
    /// a literal that represents a date
    Date(Date),
    /// a literal that represents a date and time
    DateAndTime(DateAndTime),
    /// a literal that represents the time of day
    TimeOfDay(TimeOfDay),
    /// a literal that represents a time period
    Time(Time),
    /// a literal that represents a real number (e.g. 7.0)
    Real(String),
    /// a literal that represents a boolean value (true, false)
    Bool(bool),
    /// a literal that represents a string
    String(StringValue),
    /// a literal that represents an array
    Array(Array),
}

macro_rules! impl_try_from {
    (for $($id:ident),+) => {
        $(impl<'ast> TryFrom<&'ast AstNode> for &'ast $id {
            type Error = ();

            fn try_from(value: &'ast AstNode) -> Result<Self, Self::Error> {
                let crate::ast::AstStatement::Literal(AstLiteral::$id(inner)) = value.get_stmt() else {
                    return Err(())
                };
                Ok(inner)
            }
        })*
    };
    (for $($id:ident, $p:path),+) => {
        $(impl<'ast> TryFrom<&'ast AstNode> for &'ast $id {
            type Error = ();

            fn try_from(value: &'ast AstNode) -> Result<Self, Self::Error> {
                let crate::ast::AstStatement::Literal($p(inner)) = value.get_stmt() else {
                    return Err(())
                };
                Ok(inner)
            }
        })*
    };
}

impl_try_from!(for Date, DateAndTime, TimeOfDay, Time, Array);
// XXX: String::try_from(..) is ambiguous between `AstLiteral::Real` and `AstStatement::Identifier`
impl_try_from!(for i128, AstLiteral::Integer, String, AstLiteral::Real, bool, AstLiteral::Bool, StringValue, AstLiteral::String);

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateAndTime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeOfDay {
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    pub day: f64,
    pub hour: f64,
    pub min: f64,
    pub sec: f64,
    pub milli: f64,
    pub micro: f64,
    pub nano: u32,
    pub negative: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringValue {
    pub value: String,
    pub is_wide: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub elements: Option<Box<AstNode>>, // expression-list
}

/// calculates the nanoseconds since 1970-01-01-00:00:00 for the given
/// point in time
fn calculate_date_time(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
) -> Result<i64, String> {
    NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_nano_opt(hour, min, sec, nano))
        .ok_or_else(|| format!("Invalid Date {year}-{month}-{day}-{hour}:{min}:{sec}.{nano}"))
        .and_then(|date_time| {
            date_time
                .and_utc()
                .timestamp_nanos_opt()
                .ok_or_else(|| format!("Out of range Date {year}-{month}-{day}-{hour}:{min}:{sec}.{nano}"))
        })
}

impl DateAndTime {
    /// the value of the date and time in nanoseconds since 1970-01-01-00:00:00
    pub fn value(&self) -> Result<i64, String> {
        calculate_date_time(self.year, self.month, self.day, self.hour, self.min, self.sec, self.nano)
    }
}

impl Time {
    /// the nanos represented by the given time-period
    pub fn value(&self) -> i64 {
        let dhm_seconds = {
            let hours = self.day * 24_f64 + self.hour;
            let mins = hours * 60_f64 + self.min;
            mins * 60_f64 + self.sec
        };
        let millis = dhm_seconds * 1000_f64 + self.milli;
        let micro = millis * 1000_f64 + self.micro;
        let nano = micro * 1000_f64 + self.nano as f64;
        //go to full micro
        let nanos = nano.round() as i64;

        if self.negative {
            -nanos
        } else {
            nanos
        }
    }
}

impl TimeOfDay {
    /// the value of the time of day in nanoseconds since 1970-01-01-00:00:00
    pub fn value(&self) -> Result<i64, String> {
        calculate_date_time(1970, 1, 1, self.hour, self.min, self.sec, self.nano)
    }
}

impl Date {
    /// the value of the date in nanoseconds since 1970-01-01-00:00:00
    /// the time-part of the returned value is set to 00:00:00
    pub fn value(&self) -> Result<i64, String> {
        calculate_date_time(self.year, self.month, self.day, 0, 0, 0, 0)
    }
}

impl_getters! { Date, [year, month, day], [i32, u32, u32] }
impl_getters! { DateAndTime, [year, month, day, hour, min, sec, nano], [i32, u32, u32, u32, u32, u32, u32]}
impl_getters! { TimeOfDay, [hour, min, sec, nano], [u32, u32, u32, u32]}
impl_getters! { Time, [day, hour, min, sec, milli, micro, nano], [f64, f64, f64, f64, f64, f64, u32]}

impl StringValue {
    pub fn is_wide(&self) -> bool {
        self.is_wide
    }

    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Time {
    pub fn is_negative(&self) -> bool {
        self.negative
    }
}

impl Array {
    pub fn elements(&self) -> Option<&AstNode> {
        self.elements.as_ref().map(|it| it.as_ref())
    }
}

impl AstLiteral {
    /// Creates a new literal array
    pub fn new_array(elements: Option<Box<AstNode>>) -> Self {
        AstLiteral::Array(Array { elements })
    }
    /// Creates a new literal integer
    pub fn new_integer(value: i128) -> Self {
        AstLiteral::Integer(value)
    }
    /// Creates a new literal real
    pub fn new_real(value: String) -> Self {
        AstLiteral::Real(value)
    }
    /// Creates a new literal bool
    pub fn new_bool(value: bool) -> Self {
        AstLiteral::Bool(value)
    }
    /// Creates a new literal string
    pub fn new_string(value: String, is_wide: bool) -> Self {
        AstLiteral::String(StringValue { value, is_wide })
    }

    /// Creates a new literal date
    pub fn new_date(year: i32, month: u32, day: u32) -> Self {
        AstLiteral::Date(Date { year, month, day })
    }

    /// Creates a new literal date and time
    pub fn new_date_and_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    ) -> Self {
        AstLiteral::DateAndTime(DateAndTime { year, month, day, hour, min, sec, nano })
    }

    /// Creates a new literal time of day
    pub fn new_time_of_day(hour: u32, min: u32, sec: u32, nano: u32) -> Self {
        AstLiteral::TimeOfDay(TimeOfDay { hour, min, sec, nano })
    }

    /// Creates a new literal null
    pub fn new_null() -> Self {
        AstLiteral::Null
    }

    pub fn get_literal_value(&self) -> String {
        match self {
            AstLiteral::String(StringValue { value, is_wide: true, .. }) => format!(r#""{value}""#),
            AstLiteral::String(StringValue { value, is_wide: false, .. }) => format!(r#"'{value}'"#),
            AstLiteral::Bool(value) => {
                format!("{value}")
            }
            AstLiteral::Integer(value) => {
                format!("{value}")
            }
            AstLiteral::Real(value) => value.clone(),
            _ => format!("{self:#?}"),
        }
    }

    pub fn is_cast_prefix_eligible(&self) -> bool {
        // TODO: figure out a better name for this...
        matches!(
            self,
            AstLiteral::Bool { .. }
                | AstLiteral::Integer { .. }
                | AstLiteral::Real { .. }
                | AstLiteral::String { .. }
                | AstLiteral::Time { .. }
                | AstLiteral::Date { .. }
                | AstLiteral::TimeOfDay { .. }
                | AstLiteral::DateAndTime { .. }
        )
    }

    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            AstLiteral::Integer { .. }
                | AstLiteral::Real { .. }
                | AstLiteral::Time { .. }
                | AstLiteral::Date { .. }
                | AstLiteral::TimeOfDay { .. }
                | AstLiteral::DateAndTime { .. }
        )
    }

    pub fn is_zero(&self) -> bool {
        match self {
            AstLiteral::Integer(0) => true,
            AstLiteral::Real(val) => val == "0" || val == "0.0",
            _ => false,
        }
    }

    pub fn get_literal_integer_value(&self) -> Option<i128> {
        let Self::Integer(val) = self else { return None };
        Some(*val)
    }
}

impl Debug for AstLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AstLiteral::Null => f.debug_struct("LiteralNull").finish(),
            AstLiteral::Integer(value) => f.debug_struct("LiteralInteger").field("value", value).finish(),
            AstLiteral::Date(Date { year, month, day, .. }) => f
                .debug_struct("LiteralDate")
                .field("year", year)
                .field("month", month)
                .field("day", day)
                .finish(),
            AstLiteral::DateAndTime(DateAndTime { year, month, day, hour, min, sec, nano, .. }) => f
                .debug_struct("LiteralDateAndTime")
                .field("year", year)
                .field("month", month)
                .field("day", day)
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("nano", nano)
                .finish(),
            AstLiteral::TimeOfDay(TimeOfDay { hour, min, sec, nano, .. }) => f
                .debug_struct("LiteralTimeOfDay")
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("nano", nano)
                .finish(),
            AstLiteral::Time(Time { day, hour, min, sec, milli, micro, nano, negative, .. }) => f
                .debug_struct("LiteralTime")
                .field("day", day)
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("milli", milli)
                .field("micro", micro)
                .field("nano", nano)
                .field("negative", negative)
                .finish(),
            AstLiteral::Real(value) => f.debug_struct("LiteralReal").field("value", value).finish(),
            AstLiteral::Bool(value) => f.debug_struct("LiteralBool").field("value", value).finish(),
            AstLiteral::String(StringValue { value, is_wide, .. }) => {
                f.debug_struct("LiteralString").field("value", value).field("is_wide", is_wide).finish()
            }
            AstLiteral::Array(Array { elements, .. }) => {
                f.debug_struct("LiteralArray").field("elements", elements).finish()
            }
        }
    }
}
