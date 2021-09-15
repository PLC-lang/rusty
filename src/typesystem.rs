// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::ops::Range;


use crate::{ast::{AstStatement, Dimension}, index::{Index, const_expressions::{ConstId}}};

pub const DEFAULT_STRING_LEN: u32 = 80;

pub const RANGE_CHECK_S_FN: &str = "CheckRangeSigned";
pub const RANGE_CHECK_LS_FN: &str = "CheckLRangeSigned";
pub const RANGE_CHECK_U_FN: &str = "CheckRangeUnsigned";
pub const RANGE_CHECK_LU_FN: &str = "CheckLRangeUnsigned";

pub const INT_SIZE: u32 = 16;
pub const DINT_SIZE: u32 = 2 * INT_SIZE;

pub const BOOL_TYPE: &str = "BOOL";
pub const BYTE_TYPE: &str = "BYTE";
pub const SINT_TYPE: &str = "SINT";
pub const USINT_TYPE: &str = "USINT";
pub const WORD_TYPE: &str = "WORD";
pub const INT_TYPE: &str = "INT";
pub const UINT_TYPE: &str = "UINT";
pub const DWORD_TYPE: &str = "DWORD";
pub const DINT_TYPE: &str = "DINT";
pub const UDINT_TYPE: &str = "UDINT";
pub const LWORD_TYPE: &str = "LWORD";
pub const LINT_TYPE: &str = "LINT";
pub const DATE_TYPE: &str = "DATE";
pub const SHORT_DATE_TYPE: &str = "D";
pub const TIME_TYPE: &str = "TIME";
pub const SHORT_TIME_TYPE: &str = "T";
pub const DATE_AND_TIME_TYPE: &str = "DATE_AND_TIME";
pub const SHORT_DATE_AND_TIME_TYPE: &str = "DT";
pub const TIME_OF_DAY_TYPE: &str = "TIME_OF_DAY";
pub const SHORT_TIME_OF_DAY_TYPE: &str = "TOD";
pub const ULINT_TYPE: &str = "ULINT";
pub const REAL_TYPE: &str = "REAL";
pub const LREAL_TYPE: &str = "LREAL";
pub const STRING_TYPE: &str = "STRING";
pub const WSTRING_TYPE: &str = "WSTRING";

pub const VOID_TYPE: &str = "VOID";

#[derive(Debug, PartialEq)]
pub struct DataType {
    pub name: String,
    /// the initial value defined on the TYPE-declration
    pub initial_value: Option<ConstId>,
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
    Enum {
        name: String,
        elements: Vec<String>,
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
        sub_range: Range<AstStatement>,
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
            DataTypeInformation::String {
                encoding: StringEncoding::Utf8,
                ..
            } => "STRING",
            DataTypeInformation::String {
                encoding: StringEncoding::Utf16,
                ..
            } => "WSTRING",
            DataTypeInformation::SubRange { name, .. } => name,
            DataTypeInformation::Void => "VOID",
            DataTypeInformation::Alias { name, .. } => name,
            DataTypeInformation::Enum { name, .. } => name,
        }
    }

    pub fn is_int(&self) -> bool {
        // internally an enum is represented as a DINT
        matches!(
            self,
            DataTypeInformation::Integer { .. } | DataTypeInformation::Enum { .. }
        )
    }

    pub fn is_unsigned_int(&self) -> bool {
        matches!(self, DataTypeInformation::Integer { signed: false, .. })
    }

    pub fn is_signed_int(&self) -> bool {
        matches!(self, DataTypeInformation::Integer { signed: true, .. })
    }

    pub fn is_float(&self) -> bool {
        matches!(self, DataTypeInformation::Float { .. })
    }

    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            DataTypeInformation::Integer { .. }
                | DataTypeInformation::Float { .. }
                | &DataTypeInformation::Enum { .. } // internally an enum is represented as a DINT
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
            DataTypeInformation::Enum { .. } => DINT_SIZE,
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
            name: BOOL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: BOOL_TYPE.into(),
                signed: true,
                size: 1,
            },
        },
        DataType {
            name: BYTE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: BYTE_TYPE.into(),
                signed: false,
                size: 8,
            },
        },
        DataType {
            name: SINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: SINT_TYPE.into(),
                signed: true,
                size: 8,
            },
        },
        DataType {
            name: USINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: USINT_TYPE.into(),
                signed: false,
                size: 8,
            },
        },
        DataType {
            name: WORD_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: WORD_TYPE.into(),
                signed: false,
                size: 16,
            },
        },
        DataType {
            name: INT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: INT_TYPE.into(),
                signed: true,
                size: 16,
            },
        },
        DataType {
            name: UINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: UINT_TYPE.into(),
                signed: false,
                size: 16,
            },
        },
        DataType {
            name: DWORD_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DWORD_TYPE.into(),
                signed: false,
                size: DINT_SIZE,
            },
        },
        DataType {
            name: DINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DINT_TYPE.into(),
                signed: true,
                size: DINT_SIZE,
            },
        },
        DataType {
            name: UDINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: UDINT_TYPE.into(),
                signed: false,
                size: DINT_SIZE,
            },
        },
        DataType {
            name: LWORD_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: LWORD_TYPE.into(),
                signed: false,
                size: 64,
            },
        },
        DataType {
            name: LINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: LINT_TYPE.into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: DATE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DATE_TYPE.into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: TIME_TYPE.into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: DATE_AND_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DATE_AND_TIME_TYPE.into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: TIME_OF_DAY_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: TIME_OF_DAY_TYPE.into(),
                signed: true,
                size: 64,
            },
        },
        DataType {
            name: ULINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: ULINT_TYPE.into(),
                signed: false,
                size: 64,
            },
        },
        DataType {
            name: REAL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Float {
                name: REAL_TYPE.into(),
                size: 32,
            },
        },
        DataType {
            name: LREAL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Float {
                name: LREAL_TYPE.into(),
                size: 64,
            },
        },
        DataType {
            name: STRING_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: DEFAULT_STRING_LEN + 1,
                encoding: StringEncoding::Utf8,
            },
        },
        DataType {
            name: WSTRING_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: DEFAULT_STRING_LEN + 1,
                encoding: StringEncoding::Utf16,
            },
        },
        DataType {
            name: SHORT_DATE_AND_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_DATE_AND_TIME_TYPE.into(),
                referenced_type: DATE_AND_TIME_TYPE.into(),
            },
        },
        DataType {
            name: SHORT_DATE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_DATE_TYPE.into(),
                referenced_type: DATE_TYPE.into(),
            },
        },
        DataType {
            name: SHORT_TIME_OF_DAY_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_TIME_OF_DAY_TYPE.into(),
                referenced_type: TIME_OF_DAY_TYPE.into(),
            },
        },
        DataType {
            name: SHORT_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_TIME_TYPE.into(),
                referenced_type: TIME_TYPE.into(),
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
        name: REAL_TYPE.into(),
        size: 32,
    }
}

fn get_lreal_type() -> DataTypeInformation {
    DataTypeInformation::Float {
        name: LREAL_TYPE.into(),
        size: 64,
    }
}

pub fn get_bigger_type(
    ltype: &DataTypeInformation,
    rtype: &DataTypeInformation,
) -> DataTypeInformation {
    if is_same_type_nature(ltype, rtype) {
        if get_rank(ltype) < get_rank(rtype) {
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
    if is_same_type_nature(ltype, rtype) {
        if get_rank(ltype) < get_rank(rtype) {
            rtype
        } else {
            ltype
        }
    } else {
        let real_type = index
            .get_type(REAL_TYPE)
            .map(|it| it.get_type_information())
            .unwrap();
        let real_size = real_type.get_size();
        if ltype.get_size() > real_size || rtype.get_size() > real_size {
            index.get_type(LREAL_TYPE).unwrap().get_type_information()
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
            BYTE_TYPE => SINT_TYPE,
            USINT_TYPE => SINT_TYPE,
            WORD_TYPE => INT_TYPE,
            UINT_TYPE => INT_TYPE,
            DWORD_TYPE => DINT_TYPE,
            UDINT_TYPE => DINT_TYPE,
            ULINT_TYPE => LINT_TYPE,
            LWORD_TYPE => LINT_TYPE,
            _ => data_type.get_name(),
        };
        return index
            .get_type(signed_type)
            .ok()
            .map(|t| t.get_type_information());
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::CompilationUnit,
        index::visitor::visit,
        typesystem::{
            get_signed_type, BYTE_TYPE, DINT_TYPE, DWORD_TYPE, INT_TYPE, LINT_TYPE, LWORD_TYPE,
            SINT_TYPE, UDINT_TYPE, UINT_TYPE, ULINT_TYPE, USINT_TYPE, WORD_TYPE,
        },
    };

    macro_rules! assert_signed_type {
        ($expected:expr, $actual:expr, $index:expr) => {
            assert_eq!(
                $index.find_type_information($expected).as_ref(),
                get_signed_type(
                    $index.find_type_information($actual).as_ref().unwrap(),
                    &$index
                )
            );
        };
    }

    #[test]
    pub fn signed_types_tests() {
        // Given an initialized index
        let index = visit(&CompilationUnit::default());
        assert_signed_type!(SINT_TYPE, BYTE_TYPE, index);
        assert_signed_type!(SINT_TYPE, USINT_TYPE, index);
        assert_signed_type!(INT_TYPE, WORD_TYPE, index);
        assert_signed_type!(INT_TYPE, UINT_TYPE, index);
        assert_signed_type!(DINT_TYPE, DWORD_TYPE, index);
        assert_signed_type!(DINT_TYPE, UDINT_TYPE, index);
        assert_signed_type!(LINT_TYPE, ULINT_TYPE, index);
        assert_signed_type!(LINT_TYPE, LWORD_TYPE, index);

        assert_eq!(
            None,
            get_signed_type(
                index.find_type_information("STRING").as_ref().unwrap(),
                &index
            )
        );
    }
}
