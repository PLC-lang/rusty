// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::ops::Range;

use crate::{
    ast::{Dimension, Statement},
    index::Index,
};

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

type VarArgs = Option<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum StringEncoding {
    Utf8,
    Utf16,
}

impl StringEncoding {
    pub fn get_bytes_per_char(&self) -> u32 {
        match self {
            StringEncoding::Utf8 => 1,
            StringEncoding::Utf16 => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeInformation {
    Struct {
        name: String,
        member_names: Vec<String>,
        varargs: Option<VarArgs>,
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
        encoding: StringEncoding,
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

    pub fn is_variadic(&self) -> bool {
        matches!(
            self,
            DataTypeInformation::Struct {
                varargs: Some(_),
                ..
            }
        )
    }

    pub fn get_variadic_type(&self) -> Option<&str> {
        if let DataTypeInformation::Struct {
            varargs: Some(inner_type),
            ..
        } = &self
        {
            inner_type.as_ref().map(String::as_str)
        } else {
            None
        }
    }

    pub fn get_size(&self) -> u32 {
        match self {
            DataTypeInformation::Integer { size, .. } => *size,
            DataTypeInformation::Float { size, .. } => *size,
            DataTypeInformation::String { size, .. } => *size,
            DataTypeInformation::Struct { .. } => 0, //TODO : Should we fill in the struct members here for size calculation or save the struct size.
            DataTypeInformation::Array { .. } => unimplemented!("array"), //Propably length * inner type size
            DataTypeInformation::Pointer { .. } => unimplemented!("pointer"),
            DataTypeInformation::SubRange { .. } => unimplemented!("subrange"),
            DataTypeInformation::Alias { .. } => unimplemented!("alias"),
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
        DataType {
            name: "DATE".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "DATE".into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: "TIME".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "TIME".into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: "DATE_AND_TIME".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "DATE_AND_TIME".into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: "TIME_OF_DAY".into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: "TIME_OF_DAY".into(),
                signed: true,
                size: 64,
            },
        },
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
                encoding: StringEncoding::Utf8,
            },
        },
        DataType {
            name: "WSTRING".into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: DEFAULT_STRING_LEN + 1,
                encoding: StringEncoding::Utf16,
            },
        },
    ]
}

pub fn new_string_information(len: u32) -> DataTypeInformation {
    DataTypeInformation::String {
        size: len + 1,
        encoding: StringEncoding::Utf8,
    }
}

pub fn new_wide_string_information(len: u32) -> DataTypeInformation {
    DataTypeInformation::String {
        size: len + 1,
        encoding: StringEncoding::Utf16,
    }
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

pub fn get_bigger_type_borrow<'t>(
    ltype: &'t DataTypeInformation,
    rtype: &'t DataTypeInformation,
    index: &'t Index,
) -> &'t DataTypeInformation {
    if is_same_type_nature(&ltype, &rtype) {
        if get_rank(&ltype) < get_rank(&rtype) {
            rtype
        } else {
            ltype
        }
    } else {
        let real_type = index
            .get_type("REAL")
            .map(|it| it.get_type_information())
            .unwrap();
        let real_size = real_type.get_size();
        if ltype.get_size() > real_size || rtype.get_size() > real_size {
            index.get_type("LREAL").unwrap().get_type_information()
        } else {
            real_type
        }
    }
}

/// returns the signed version of the given data_type if its a signed int-type
/// returns the original type if it is no signed int-type
pub fn get_signed_type<'t>(
    data_type: &'t DataTypeInformation,
    index: &'t Index,
) -> Option<&'t DataTypeInformation> {
    if data_type.is_int() {
        let signed_type = match data_type.get_name() {
            "BYTE" => "SINT",
            "USINT" => "SINT",
            "WORD" => "INT",
            "UINT" => "INT",
            "DWORD" => "DINT",
            "UDINT" => "DINT",
            "ULINT" => "LINT",
            "LWORD" => "LINT",
            _ => data_type.get_name(),
        };
        return index
            .get_type(signed_type)
            .ok()
            .map(|t| t.get_type_information());
    }
    None
}
