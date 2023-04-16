use std::fmt::{Debug, Formatter, Result};

use crate::typesystem::{
    BOOL_TYPE, DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, INT_TYPE, LINT_TYPE, LREAL_TYPE, SINT_TYPE,
    STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, UDINT_TYPE, UINT_TYPE, ULINT_TYPE, USINT_TYPE, VOID_TYPE,
    WSTRING_TYPE,
};

use super::AstStatement;

//returns a range with the min and max value of the given type
macro_rules! is_covered_by {
    ($t:ty, $e:expr) => {
        <$t>::MIN as i128 <= $e as i128 && $e as i128 <= <$t>::MAX as i128
    };
}

#[derive(Clone, PartialEq)]
pub enum LiteralKind {
    LiteralNull,
    LiteralInteger {
        value: i128,
    },
    LiteralDate {
        year: i32,
        month: u32,
        day: u32,
    },
    LiteralDateAndTime {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    },
    LiteralTimeOfDay {
        hour: u32,
        min: u32,
        sec: u32,
        nano: u32,
    },
    LiteralTime {
        day: f64,
        hour: f64,
        min: f64,
        sec: f64,
        milli: f64,
        micro: f64,
        nano: u32,
        negative: bool,
    },
    LiteralReal {
        value: String,
    },
    LiteralBool {
        value: bool,
    },
    LiteralString {
        value: String,
        is_wide: bool,
    },
    LiteralArray {
        elements: Option<Box<AstStatement>>, // expression-list
    },
}

impl LiteralKind {
    /// Creates a new literal array
    pub fn new_array(elements: Option<Box<AstStatement>>) -> Self {
        LiteralKind::LiteralArray { elements }
    }
    /// Creates a new literal integer
    pub fn new_integer(value: i128) -> Self {
        LiteralKind::LiteralInteger { value }
    }
    /// Creates a new literal real
    pub fn new_real(value: String) -> Self {
        LiteralKind::LiteralReal { value }
    }
    // Creates a new literal bool
    pub fn new_bool(value: bool) -> Self {
        LiteralKind::LiteralBool { value }
    }
    // Creates a new literal string
    pub fn new_string(value: String, is_wide: bool) -> Self {
        LiteralKind::LiteralString { value, is_wide }
    }
    // Creates a new literal null
    pub fn null() -> Self {
        LiteralKind::LiteralNull
    }

    pub fn get_literal_actual_signed_type_name(&self, signed: bool) -> Option<&str> {
        match self {
            LiteralKind::LiteralInteger { value, .. } => match signed {
                _ if *value == 0_i128 || *value == 1_i128 => Some(BOOL_TYPE),
                true if is_covered_by!(i8, *value) => Some(SINT_TYPE),
                true if is_covered_by!(i16, *value) => Some(INT_TYPE),
                true if is_covered_by!(i32, *value) => Some(DINT_TYPE),
                true if is_covered_by!(i64, *value) => Some(LINT_TYPE),

                false if is_covered_by!(u8, *value) => Some(USINT_TYPE),
                false if is_covered_by!(u16, *value) => Some(UINT_TYPE),
                false if is_covered_by!(u32, *value) => Some(UDINT_TYPE),
                false if is_covered_by!(u64, *value) => Some(ULINT_TYPE),
                _ => Some(VOID_TYPE),
            },
            LiteralKind::LiteralBool { .. } => Some(BOOL_TYPE),
            LiteralKind::LiteralString { is_wide: true, .. } => Some(WSTRING_TYPE),
            LiteralKind::LiteralString { is_wide: false, .. } => Some(STRING_TYPE),
            LiteralKind::LiteralReal { .. } => Some(LREAL_TYPE),
            LiteralKind::LiteralDate { .. } => Some(DATE_TYPE),
            LiteralKind::LiteralDateAndTime { .. } => Some(DATE_AND_TIME_TYPE),
            LiteralKind::LiteralTime { .. } => Some(TIME_TYPE),
            LiteralKind::LiteralTimeOfDay { .. } => Some(TIME_OF_DAY_TYPE),
            _ => None,
        }
    }

    pub fn get_literal_value(&self) -> String {
        match self {
            LiteralKind::LiteralString { value, is_wide: true, .. } => format!(r#""{value}""#),
            LiteralKind::LiteralString { value, is_wide: false, .. } => format!(r#"'{value}'"#),
            LiteralKind::LiteralBool { value, .. } => {
                format!("{value}")
            }
            LiteralKind::LiteralInteger { value, .. } => {
                format!("{value}")
            }
            LiteralKind::LiteralReal { value, .. } => value.clone(),
            _ => format!("{self:#?}"),
        }
    }

    pub fn is_cast_prefix_eligible(&self) -> bool {
        // TODO: figure out a better name for this...
        matches!(
            self,
            LiteralKind::LiteralBool { .. }
                | LiteralKind::LiteralInteger { .. }
                | LiteralKind::LiteralReal { .. }
                | LiteralKind::LiteralString { .. }
                | LiteralKind::LiteralTime { .. }
                | LiteralKind::LiteralDate { .. }
                | LiteralKind::LiteralTimeOfDay { .. }
                | LiteralKind::LiteralDateAndTime { .. }
        )
    }
}

impl Debug for LiteralKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LiteralKind::LiteralNull => f.debug_struct("LiteralNull").finish(),
            LiteralKind::LiteralInteger { value, .. } => {
                f.debug_struct("LiteralInteger").field("value", value).finish()
            }
            LiteralKind::LiteralDate { year, month, day, .. } => f
                .debug_struct("LiteralDate")
                .field("year", year)
                .field("month", month)
                .field("day", day)
                .finish(),
            LiteralKind::LiteralDateAndTime { year, month, day, hour, min, sec, nano, .. } => f
                .debug_struct("LiteralDateAndTime")
                .field("year", year)
                .field("month", month)
                .field("day", day)
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("nano", nano)
                .finish(),
            LiteralKind::LiteralTimeOfDay { hour, min, sec, nano, .. } => f
                .debug_struct("LiteralTimeOfDay")
                .field("hour", hour)
                .field("min", min)
                .field("sec", sec)
                .field("nano", nano)
                .finish(),
            LiteralKind::LiteralTime { day, hour, min, sec, milli, micro, nano, negative, .. } => f
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
            LiteralKind::LiteralReal { value, .. } => {
                f.debug_struct("LiteralReal").field("value", value).finish()
            }
            LiteralKind::LiteralBool { value, .. } => {
                f.debug_struct("LiteralBool").field("value", value).finish()
            }
            LiteralKind::LiteralString { value, is_wide, .. } => {
                f.debug_struct("LiteralString").field("value", value).field("is_wide", is_wide).finish()
            }
            LiteralKind::LiteralArray { elements, .. } => {
                f.debug_struct("LiteralArray").field("elements", elements).finish()
            }
        }
    }
}
