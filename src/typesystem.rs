// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::{mem::size_of, ops::Range};

use crate::{
    ast::{AstStatement, GenericBinding, Operator, PouType, TypeNature},
    index::{const_expressions::ConstId, Index},
};

pub const DEFAULT_STRING_LEN: u32 = 80;

// Ranged type check functions names
pub const RANGE_CHECK_S_FN: &str = "CheckRangeSigned";
pub const RANGE_CHECK_LS_FN: &str = "CheckLRangeSigned";
pub const RANGE_CHECK_U_FN: &str = "CheckRangeUnsigned";
pub const RANGE_CHECK_LU_FN: &str = "CheckLRangeUnsigned";

pub type NativeSintType = i8;
pub type NativeIntType = i16;
pub type NativeDintType = i32;
pub type NativeLintType = i64;
pub type NativeByteType = u8;
pub type NativeWordType = u16;
pub type NativeDwordType = u32;
pub type NativeLwordType = u64;
pub type NativeRealType = f32;
pub type NativeLrealType = f64;

//TODO should we change this to usize?
pub const U1_SIZE: u32 = 1;
pub const BOOL_SIZE: u32 = BYTE_SIZE;
pub const BYTE_SIZE: u32 = (size_of::<NativeSintType>() * 8) as u32;
pub const SINT_SIZE: u32 = (size_of::<NativeSintType>() * 8) as u32;
pub const INT_SIZE: u32 = (size_of::<NativeIntType>() * 8) as u32;
pub const DINT_SIZE: u32 = (size_of::<NativeDintType>() * 8) as u32;
pub const LINT_SIZE: u32 = (size_of::<NativeLintType>() * 8) as u32;
pub const REAL_SIZE: u32 = (size_of::<NativeRealType>() * 8) as u32;
pub const LREAL_SIZE: u32 = (size_of::<NativeLrealType>() * 8) as u32;
pub const DATE_TIME_SIZE: u32 = 64;

pub const U1_TYPE: &str = "__U1";
/// used internally for forced casts to u1
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
pub const CHAR_TYPE: &str = "CHAR";
pub const WCHAR_TYPE: &str = "WCHAR";
pub const VOID_TYPE: &str = "VOID";

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq)]
pub struct DataType {
    pub name: String,
    /// the initial value defined on the TYPE-declration
    pub initial_value: Option<ConstId>,
    pub information: DataTypeInformation,
    pub nature: TypeNature,
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

    pub fn has_nature(&self, nature: TypeNature, index: &Index) -> bool {
        let type_nature = index.get_intrinsic_type_by_name(self.get_name()).nature;
        type_nature.derives(nature)
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
pub enum TypeSize {
    LiteralInteger(u32),
    ConstExpression(ConstId),
}

impl TypeSize {
    pub fn from_literal(v: u32) -> TypeSize {
        TypeSize::LiteralInteger(v)
    }

    pub fn from_expression(id: ConstId) -> TypeSize {
        TypeSize::ConstExpression(id)
    }

    /// tries to compile-time evaluate the size-expression to an i64
    pub fn as_int_value(&self, index: &Index) -> Result<i64, String> {
        match self {
            TypeSize::LiteralInteger(v) => Ok(*v as i64),
            TypeSize::ConstExpression(id) => index
                .get_const_expressions()
                .get_constant_int_statement_value(id)
                .map(|it| it as i64),
        }
    }

    /// returns the const expression represented by this TypeSize or None if this TypeSize
    /// is a compile-time literal
    pub fn as_const_expression<'i>(&self, index: &'i Index) -> Option<&'i AstStatement> {
        match self {
            TypeSize::LiteralInteger(_) => None,
            TypeSize::ConstExpression(id) => {
                index.get_const_expressions().get_constant_statement(id)
            }
        }
    }
}

/// indicates where this Struct origins from.
#[derive(Debug, Clone, PartialEq)]
pub enum StructSource {
    OriginalDeclaration,
    Pou(PouType),
}

type TypeId = String;

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeInformation {
    Struct {
        name: TypeId,
        member_names: Vec<String>,
        varargs: Option<VarArgs>,
        source: StructSource,
        generics: Vec<GenericBinding>,
    },
    Array {
        name: TypeId,
        inner_type_name: TypeId,
        dimensions: Vec<Dimension>,
    },
    Pointer {
        name: TypeId,
        inner_type_name: TypeId,
        auto_deref: bool,
    },
    Integer {
        name: TypeId,
        signed: bool,
        /// the number of bit stored in memory
        size: u32,
        /// the numer of bits represented by this type (may differ from the num acutally stored)
        semantic_size: Option<u32>,
    },
    Enum {
        name: TypeId,
        referenced_type: TypeId,
        elements: Vec<String>,
    },
    Float {
        name: TypeId,
        size: u32,
    },
    String {
        size: TypeSize,
        encoding: StringEncoding,
    },
    SubRange {
        name: TypeId,
        referenced_type: TypeId,
        sub_range: Range<AstStatement>,
    },
    Alias {
        name: TypeId,
        referenced_type: TypeId,
    },
    Generic {
        name: TypeId,
        generic_symbol: String,
        nature: TypeNature,
    },
    Void,
}

impl DataTypeInformation {
    pub fn get_name(&self) -> &str {
        match self {
            DataTypeInformation::Struct { name, .. }
            | DataTypeInformation::Array { name, .. }
            | DataTypeInformation::Pointer { name, .. }
            | DataTypeInformation::Integer { name, .. }
            | DataTypeInformation::Float { name, .. }
            | DataTypeInformation::SubRange { name, .. }
            | DataTypeInformation::Alias { name, .. }
            | DataTypeInformation::Enum { name, .. }
            | DataTypeInformation::Generic { name, .. } => name,
            DataTypeInformation::String {
                encoding: StringEncoding::Utf8,
                ..
            } => "STRING",
            DataTypeInformation::String {
                encoding: StringEncoding::Utf16,
                ..
            } => "WSTRING",
            DataTypeInformation::Void => "VOID",
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, DataTypeInformation::String { .. })
    }

    pub fn is_character(&self) -> bool {
        match self {
            DataTypeInformation::Integer { name, .. } => name == WCHAR_TYPE || name == CHAR_TYPE,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        // internally an enum is represented as a DINT
        matches!(
            self,
            DataTypeInformation::Integer { .. } | DataTypeInformation::Enum { .. }
        )
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { .. })
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

    pub fn is_struct(&self) -> bool {
        matches!(self, DataTypeInformation::Struct { .. })
    }

    pub fn is_array(&self) -> bool {
        matches!(self, DataTypeInformation::Array { .. })
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

    pub fn is_generic(&self) -> bool {
        match self {
            DataTypeInformation::Struct { generics, .. } => !generics.is_empty(),
            DataTypeInformation::Generic { .. } => true,
            _ => false,
        }
    }

    /// returns the number of bits of this type, as understood by IEC61131 (may be smaller than get_size(...))
    pub fn get_semantic_size(&self) -> u32 {
        if let DataTypeInformation::Integer {
            semantic_size: Some(s),
            ..
        } = self
        {
            return *s;
        }
        self.get_size()
    }

    /// returns the number of bits used to store this type
    pub fn get_size(&self) -> u32 {
        match self {
            DataTypeInformation::Integer { size, .. } => *size,
            DataTypeInformation::Float { size, .. } => *size,
            DataTypeInformation::String { .. } => unimplemented!("string"),
            DataTypeInformation::Struct { .. } => 0, //TODO : Should we fill in the struct members here for size calculation or save the struct size.
            DataTypeInformation::Array { .. } => unimplemented!("array"), //Propably length * inner type size
            DataTypeInformation::Pointer { .. } => unimplemented!("pointer"),
            DataTypeInformation::SubRange { .. } => unimplemented!("subrange"),
            DataTypeInformation::Alias { .. } => unimplemented!("alias"),
            DataTypeInformation::Void => 0,
            DataTypeInformation::Enum { .. } => DINT_SIZE,
            DataTypeInformation::Generic { .. } => unimplemented!("generics"),
        }
    }

    pub fn get_alignment(&self) -> u32 {
        match self {
            DataTypeInformation::String { encoding, .. } if encoding == &StringEncoding::Utf8 => 1,
            DataTypeInformation::String { encoding, .. } if encoding == &StringEncoding::Utf16 => 1,
            _ => unimplemented!("Alignment for {}", self.get_name()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dimension {
    pub start_offset: TypeSize,
    pub end_offset: TypeSize,
}

impl Dimension {
    pub fn get_length(&self, index: &Index) -> Result<u32, String> {
        let end = self.end_offset.as_int_value(index)?;
        let start = self.start_offset.as_int_value(index)?;
        Ok((end - start + 1) as u32)
    }

    pub fn get_range(&self, index: &Index) -> Result<Range<i128>, String> {
        let start = self.start_offset.as_int_value(index)? as i128;
        let end = self.end_offset.as_int_value(index)? as i128;
        Ok(start..end)
    }
}

pub trait DataTypeInformationProvider<'a>: Into<&'a DataTypeInformation> {
    fn get_type_information(&self) -> &DataTypeInformation;
}

impl<'a> DataTypeInformationProvider<'a> for &'a DataTypeInformation {
    fn get_type_information(&self) -> &'a DataTypeInformation {
        self
    }
}

impl<'a> From<&'a DataType> for &'a DataTypeInformation {
    fn from(dt: &'a DataType) -> Self {
        dt.get_type_information()
    }
}

impl<'a> DataTypeInformationProvider<'a> for &'a DataType {
    fn get_type_information(&self) -> &DataTypeInformation {
        DataType::get_type_information(self)
    }
}

pub fn get_builtin_types() -> Vec<DataType> {
    vec![
        DataType {
            name: "__VOID".into(),
            initial_value: None,
            information: DataTypeInformation::Void,
            nature: TypeNature::Any,
        },
        DataType {
            name: U1_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: U1_TYPE.into(),
                signed: false,
                size: U1_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Any,
        },
        DataType {
            name: BOOL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: BOOL_TYPE.into(),
                signed: false,
                size: BOOL_SIZE,
                semantic_size: Some(1),
            },
            nature: TypeNature::Bit,
        },
        DataType {
            name: BYTE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: BYTE_TYPE.into(),
                signed: false,
                size: BYTE_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Bit,
        },
        DataType {
            name: SINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: SINT_TYPE.into(),
                signed: true,
                size: SINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Signed,
        },
        DataType {
            name: USINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: USINT_TYPE.into(),
                signed: false,
                size: SINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Unsigned,
        },
        DataType {
            name: WORD_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: WORD_TYPE.into(),
                signed: false,
                size: INT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Bit,
        },
        DataType {
            name: INT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: INT_TYPE.into(),
                signed: true,
                size: INT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Signed,
        },
        DataType {
            name: UINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: UINT_TYPE.into(),
                signed: false,
                size: INT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Unsigned,
        },
        DataType {
            name: DWORD_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DWORD_TYPE.into(),
                signed: false,
                size: DINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Bit,
        },
        DataType {
            name: DINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DINT_TYPE.into(),
                signed: true,
                size: DINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Signed,
        },
        DataType {
            name: UDINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: UDINT_TYPE.into(),
                signed: false,
                size: DINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Unsigned,
        },
        DataType {
            name: LWORD_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: LWORD_TYPE.into(),
                signed: false,
                size: LINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Bit,
        },
        DataType {
            name: LINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: LINT_TYPE.into(),
                signed: true,
                size: LINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Signed,
        },
        DataType {
            name: DATE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DATE_TYPE.into(),
                signed: true,
                size: DATE_TIME_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Date,
        },
        DataType {
            name: TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: TIME_TYPE.into(),
                signed: true,
                size: DATE_TIME_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Duration,
        },
        DataType {
            name: DATE_AND_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: DATE_AND_TIME_TYPE.into(),
                signed: true,
                size: DATE_TIME_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Date,
        },
        DataType {
            name: TIME_OF_DAY_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: TIME_OF_DAY_TYPE.into(),
                signed: true,
                size: DATE_TIME_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Date,
        },
        DataType {
            name: ULINT_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: ULINT_TYPE.into(),
                signed: false,
                size: LINT_SIZE,
                semantic_size: None,
            },
            nature: TypeNature::Unsigned,
        },
        DataType {
            name: REAL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Float {
                name: REAL_TYPE.into(),
                size: REAL_SIZE,
            },
            nature: TypeNature::Real,
        },
        DataType {
            name: LREAL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Float {
                name: LREAL_TYPE.into(),
                size: LREAL_SIZE,
            },
            nature: TypeNature::Real,
        },
        DataType {
            name: STRING_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: TypeSize::from_literal(DEFAULT_STRING_LEN + 1),
                encoding: StringEncoding::Utf8,
            },
            nature: TypeNature::String,
        },
        DataType {
            name: WSTRING_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: TypeSize::from_literal(DEFAULT_STRING_LEN + 1),
                encoding: StringEncoding::Utf16,
            },
            nature: TypeNature::String,
        },
        DataType {
            name: SHORT_DATE_AND_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_DATE_AND_TIME_TYPE.into(),
                referenced_type: DATE_AND_TIME_TYPE.into(),
            },
            nature: TypeNature::Date,
        },
        DataType {
            name: SHORT_DATE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_DATE_TYPE.into(),
                referenced_type: DATE_TYPE.into(),
            },
            nature: TypeNature::Date,
        },
        DataType {
            name: SHORT_TIME_OF_DAY_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_TIME_OF_DAY_TYPE.into(),
                referenced_type: TIME_OF_DAY_TYPE.into(),
            },
            nature: TypeNature::Date,
        },
        DataType {
            name: SHORT_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_TIME_TYPE.into(),
                referenced_type: TIME_TYPE.into(),
            },
            nature: TypeNature::Duration,
        },
        DataType {
            name: CHAR_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: CHAR_TYPE.into(),
                signed: false,
                size: 8,
                semantic_size: None,
            },
            nature: TypeNature::Char,
        },
        DataType {
            name: WCHAR_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Integer {
                name: WCHAR_TYPE.into(),
                signed: false,
                size: 16,
                semantic_size: None,
            },
            nature: TypeNature::Char,
        },
    ]
}

fn get_rank(type_information: &DataTypeInformation, index: &Index) -> u32 {
    match type_information {
        DataTypeInformation::Integer { signed, size, .. } => {
            if *signed {
                *size + 1
            } else {
                *size
            }
        }
        DataTypeInformation::Float { size, .. } => size + 1000,
        DataTypeInformation::String { size, .. } => match size {
            TypeSize::LiteralInteger(size) => *size,
            TypeSize::ConstExpression(_) => todo!("String rank with CONSTANTS"),
        },
        DataTypeInformation::Enum {
            referenced_type, ..
        } => index
            .find_effective_type_info(referenced_type)
            .map(|it| get_rank(it, index))
            .unwrap_or(DINT_SIZE),
        _ => todo!("{:?}", type_information),
    }
}

/// Returns true if provided types have the same type nature
/// i.e. Both are numeric or both are floats
pub fn is_same_type_class(
    ltype: &DataTypeInformation,
    rtype: &DataTypeInformation,
    index: &Index,
) -> bool {
    let ltype = index.find_intrinsic_type(ltype);
    let rtype = index.find_intrinsic_type(rtype);
    match ltype {
        DataTypeInformation::Integer { .. } => matches!(rtype, DataTypeInformation::Integer { .. }),
        DataTypeInformation::Float { .. } => matches!(rtype, DataTypeInformation::Float { .. }),
        DataTypeInformation::String { encoding: lenc, .. } => {
            matches!(rtype, DataTypeInformation::String { encoding, .. } if encoding == lenc)
        }
        _ => ltype == rtype,
    }
}

/// Returns the bigger of the two provided types
pub fn get_bigger_type<
    't,
    T: DataTypeInformationProvider<'t> + std::convert::From<&'t DataType>,
>(
    left_type: T,
    right_type: T,
    index: &'t Index,
) -> T {
    let lt = left_type.get_type_information();
    let rt = right_type.get_type_information();
    if is_same_type_class(lt, rt, index) {
        if get_rank(lt, index) < get_rank(rt, index) {
            right_type
        } else {
            left_type
        }
    } else if lt.is_numerical() && rt.is_numerical() {
        let real_type = index.get_type_or_panic(REAL_TYPE);
        let real_size = real_type.get_type_information().get_size();
        if lt.get_size() > real_size || rt.get_size() > real_size {
            index.get_type_or_panic(LREAL_TYPE).into()
        } else {
            real_type.into()
        }
    } else {
        //Return the first
        left_type
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
    Some(data_type)
}

/**
 * returns the compare-function name for the given type and operator.
 * Returns None if the given operator is no comparison operator
 */
pub fn get_equals_function_name_for(type_name: &str, operator: &Operator) -> Option<String> {
    let suffix = match operator {
        Operator::Equal => Some("EQUAL"),
        Operator::Less => Some("LESS"),
        Operator::Greater => Some("GREATER"),
        _ => None,
    };

    suffix.map(|suffix| format!("{}_{}", type_name, suffix))
}
