// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::ops::Range;

use crate::ast::{Dimension, Statement};

pub const DEFAULT_STRING_LEN: u32 = 80;

//CheckRan­geSigned, CheckLRangeSigned or CheckRangeUnsigned, CheckLRangeUnsigned
pub const RANGE_CHECK_S_FN: &str = "CheckRangeSigned";
pub const RANGE_CHECK_LS_FN: &str = "CheckLRangeSigned";
pub const RANGE_CHECK_U_FN: &str = "CheckRangeUnsigned";
pub const RANGE_CHECK_LU_FN: &str = "CheckLRangeUnsigned";

#[derive(Debug, PartialEq)]
pub struct DataType {
    pub name: String,
    /// the initial value defined on the TYPE-declration
    pub initial_value: Option<Statement>,
    pub information: DataTypeInformation,
    //TODO : Add location information
}

impl DataType {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_type_information(&self) -> &DataTypeInformation {
        &self.information
    }

    pub fn clone_type_information(&self) -> DataTypeInformation {
        self.information.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeInformation {
    Struct {
        name: String,
        member_names: Vec<String>,
    },
    Array {
        name: String,
        inner_type_name: String,
        dimensions: Vec<Dimension>,
    },
    Pointer {
        name: String,
        inner_type_name: String,
        auto_deref: bool,
    },
    Integer {
        name: String,
        signed: bool,
        size: u32,
    },
    Float {
        name: String,
        size: u32,
    },
    String {
        size: u32,
    },
    SubRange {
        name: String,
        referenced_type: String,
        sub_range: Range<Statement>,
    },
    Alias {
        name: String,
        referenced_type: String,
    },
    Void,
}

impl DataTypeInformation {
    pub fn get_name(&self) -> &str {
        match self {
            DataTypeInformation::Struct { name, .. } => name,
            DataTypeInformation::Array { name, .. } => name,
            DataTypeInformation::Pointer { name, .. } => name,
            DataTypeInformation::Integer { name, .. } => name,
            DataTypeInformation::Float { name, .. } => name,
            DataTypeInformation::String { .. } => "String",
            DataTypeInformation::SubRange { name, .. } => name,
            DataTypeInformation::Void => "Void",
            DataTypeInformation::Alias { name, .. } => name,
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self, DataTypeInformation::Integer { .. })
    }

    pub fn is_float(&self) -> bool {
        matches!(self, DataTypeInformation::Float { .. })
    }

    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            DataTypeInformation::Integer { .. } | DataTypeInformation::Float { .. }
        )
    }

    pub fn get_size(&self) -> u32 {
        match self {
            DataTypeInformation::Integer { size, .. } => *size,
            DataTypeInformation::Float { size, .. } => *size,
            DataTypeInformation::String { size, .. } => *size,
            DataTypeInformation::Struct { .. } => 0, //TODO : Should we fill in the struct members here for size calculation or save the struct size.
            DataTypeInformation::Array { .. } => unimplemented!(), //Propably length * inner type size
            DataTypeInformation::Pointer { .. } => unimplemented!(),
            DataTypeInformation::SubRange { .. } => unimplemented!(),
            DataTypeInformation::Alias { .. } => unimplemented!(),
            DataTypeInformation::Void => 0,
        }
    }
}

pub fn get_builtin_types() -> Vec<DataType> {
    vec![
        DataType {
            name: "__VOID".into(),
            initial_value: None,
            information: DataTypeInformation::Void,
        },
        DataType {
            name: "BOOL".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "BOOL".into(),
                signed: true,
                size: 1,
            },
        },
        DataType {
            name: "BYTE".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "BYTE".into(),
                signed: false,
                size: 8,
            },
        },
        DataType {
            name: "SINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "SINT".into(),
                signed: true,
                size: 8,
            },
        },
        DataType {
            name: "USINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "USINT".into(),
                signed: false,
                size: 8,
            },
        },
        DataType {
            name: "WORD".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "WORD".into(),
                signed: false,
                size: 16,
            },
        },
        DataType {
            name: "INT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "INT".into(),
                signed: true,
                size: 16,
            },
        },
        DataType {
            name: "UINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "UINT".into(),
                signed: false,
                size: 16,
            },
        },
        DataType {
            name: "DWORD".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "DWORD".into(),
                signed: false,
                size: 32,
            },
        },
        DataType {
            name: "DINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "DINT".into(),
                signed: true,
                size: 32,
            },
        },
        DataType {
            name: "UDINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "UDINT".into(),
                signed: false,
                size: 32,
            },
        },
        DataType {
            name: "LWORD".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "LWORD".into(),
                signed: false,
                size: 64,
            },
        },
        DataType {
            name: "LINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "LINT".into(),
                signed: true,
                size: 64,
            },
        },
        create_date_type(),
        create_time_type(),
        create_date_and_time_type(),
        create_time_of_day_type(),
        DataType {
            name: "ULINT".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "ULINT".into(),
                signed: false,
                size: 64,
            },
        },
        DataType {
            name: "REAL".into(),
            initial_value: None,
            information: DataTypeInformation::Float {
                name: "REAL".into(),
                size: 32,
            },
        },
        DataType {
            name: "LREAL".into(),
            initial_value: None,
            information: DataTypeInformation::Float {
                name: "LREAL".into(),
                size: 64,
            },
        },
        DataType {
            name: "STRING".into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: DEFAULT_STRING_LEN + 1,
            },
        },
    ]
}

pub fn new_string_information(len: u32) -> DataTypeInformation {
    DataTypeInformation::String { size: len + 1 }
}

fn get_rank(type_information: &DataTypeInformation) -> u32 {
    match type_information {
        DataTypeInformation::Integer { signed, size, .. } => {
            if *signed {
                *size + 1
            } else {
                *size
            }
        }
        DataTypeInformation::Float { size, .. } => size + 1000,
        _ => unreachable!(),
    }
}

fn is_same_type_nature(ltype: &DataTypeInformation, rtype: &DataTypeInformation) -> bool {
    (ltype.is_int() && ltype.is_int() == rtype.is_int())
        || (ltype.is_float() && ltype.is_float() == rtype.is_float())
}

fn get_real_type() -> DataTypeInformation {
    DataTypeInformation::Float {
        name: "REAL".into(),
        size: 32,
    }
}

fn get_lreal_type() -> DataTypeInformation {
    DataTypeInformation::Float {
        name: "LREAL".into(),
        size: 64,
    }
}

pub fn get_bigger_type(
    ltype: &DataTypeInformation,
    rtype: &DataTypeInformation,
) -> DataTypeInformation {
    if is_same_type_nature(&ltype, &rtype) {
        if get_rank(&ltype) < get_rank(&rtype) {
            rtype.clone()
        } else {
            ltype.clone()
        }
    } else {
        let real_type = get_real_type();
        let real_size = real_type.get_size();
        if ltype.get_size() > real_size || rtype.get_size() > real_size {
            get_lreal_type()
        } else {
            real_type
        }
    }
}

/// Create the TIME's DataType description
///
/// The TIME datatype is used to represent a time-span. A TIME value is stored as an
/// i64 value with a precision in microseconds.
/// TIME literals start with `TIME#` or `T#` followed by the TIME segements. TIME segements
/// are:
/// - d ... `f64` days
/// - h ... `f64` hours
/// - m ... `f64`minutes
/// - s ... `f64` seconds
/// - ms ... `f64` milliseconds
///
/// Note that only the last segment of a TIME literal can have a fraction.
///
/// # Example TIME Literals
/// - `TIME#2d4h6m8s10ms`
/// - `T#2d4.2h
fn create_time_type() -> DataType {
    DataType {
        name: "TIME".into(),
        initial_value: None,
        information: DataTypeInformation::Integer {
            name: "TIME".into(),
            signed: true,
            size: 64,
        },
    }
}

/// Create the DATE's DataType description
///
/// The DATE datatype is used to represent a Date in the Gregorian Calendar. A DATE value is stored as an
/// i64 value with a precision in milliseconds and denotes the number of milliseconds that have elapsed since
/// January 1, 1970 UTC not counting leap seconds.
/// DATE literals start with `DATE#` or `D#` followed by a date in the format of `yyyy-mm-dd`.
///
/// # Example DATE Literals
/// - `DATE#2021-05-02`
/// - `DATE#1-12-24`
/// - `D#2000-1-1`
fn create_date_type() -> DataType {
    DataType {
        name: "DATE".into(),
        initial_value: None,
        information: DataTypeInformation::Integer {
            name: "DATE".into(),
            signed: true,
            size: 64,
        },
    }
}

/// Create the DATE_AND_TIME's DataType description
///
/// The DATE_AND_TIME datatype is used to represent a certain point in time in the Gregorian Calendar.
/// A DATE_AND_TIME value is stored as an i64 value with a precision in milliseconds and denotes the
/// number of milliseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
/// DATE_AND_TIME literals start with `DATE_AND_TIME#` or `DT#` followed by a date and time in the
/// format of `yyyy-mm-dd-hh:mm:ss`.
///
/// Note that only the seconds-segment can have a fraction denoting the milliseconds.
///
/// # Example DATE Literals
/// - `DATE_AND_TIME#2021-05-02-14:20:10.25`
/// - `DATE_AND_TIME#1-12-24-00:00:1`
/// - `DT#1999-12-31-23:59:59.999`
fn create_date_and_time_type() -> DataType {
    DataType {
        name: "DATE_AND_TIME".into(),
        initial_value: None,
        information: DataTypeInformation::Integer {
            name: "DATE_AND_TIME".into(),
            signed: true,
            size: 64,
        },
    }
}

/// Create the TIME_OF_DAY's DataType description
///
/// The TIME_OF_DY datatype is used to represent a specific moment in time in a day.
/// A TIME_OF_DAY value is stored as an i64 value with a precision in milliseconds and denotes the
/// number of milliseconds that have elapsed since January 1, 1970 UTC not counting leap seconds.
/// Hence this value is stored as a DATE_AND_TIME with the day fixed to 1970-01-01.
/// TIME_OF_DAY literals start with `TIME_OF_DAY#` or `TOD#` followed by a time in the
/// format of `hh:mm:ss`.
///
/// Note that only the seconeds-segment can have a fraction denoting the milliseconds.
///
/// # Example TIME_OF_DAY Literals
/// - `TIME_OF_DAY#14:20:10.25`
/// - `TIME_OF_DY#0:00:1`
/// - `TOD#23:59:59.999`
/// - `TOD#23:59:59.999`
fn create_time_of_day_type() -> DataType {
    DataType {
        name: "TIME_OF_DAY".into(),
        initial_value: None,
        information: DataTypeInformation::Integer {
            name: "TIME_OF_DAY".into(),
            signed: true,
            size: 64,
        },
    }
}
