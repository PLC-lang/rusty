// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use std::{
    hash::Hash,
    mem::size_of,
    ops::{Range, RangeInclusive},
};

use anyhow::{anyhow, Result};
use plc_ast::{
    ast::{AstNode, AutoDerefType, Operator, PouType, TypeNature},
    literals::{AstLiteral, StringValue},
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};

use crate::{
    datalayout::{Bytes, MemoryLocation},
    index::{const_expressions::ConstId, Index, VariableIndexEntry},
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
// pub type NativeByteType = u8;
// pub type NativeWordType = u16;
// pub type NativeDwordType = u32;
// pub type NativeLwordType = u64;
pub type NativeRealType = f32;
pub type NativeLrealType = f64;
pub type NativePointerType = usize;

//TODO should we change this to usize?
pub const U1_SIZE: u32 = 1;
pub const BOOL_SIZE: u32 = BYTE_SIZE;
pub const BYTE_SIZE: u32 = NativeSintType::BITS;
pub const SINT_SIZE: u32 = NativeSintType::BITS;
pub const INT_SIZE: u32 = NativeIntType::BITS;
pub const DINT_SIZE: u32 = NativeDintType::BITS;
pub const LINT_SIZE: u32 = NativeLintType::BITS;
pub const REAL_SIZE: u32 = (size_of::<NativeRealType>() * 8) as u32;
pub const LREAL_SIZE: u32 = (size_of::<NativeLrealType>() * 8) as u32;
pub const DATE_TIME_SIZE: u32 = 64;
pub const POINTER_SIZE: u32 = NativePointerType::BITS;

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
pub const LONG_DATE_TYPE: &str = "LDATE";
pub const LONG_DATE_TYPE_SHORTENED: &str = "LD";
pub const TIME_TYPE: &str = "TIME";
pub const SHORT_TIME_TYPE: &str = "T";
pub const LONG_TIME_TYPE: &str = "LTIME";
pub const LONG_TIME_TYPE_SHORTENED: &str = "LT";
pub const DATE_AND_TIME_TYPE: &str = "DATE_AND_TIME";
pub const SHORT_DATE_AND_TIME_TYPE: &str = "DT";
pub const LONG_DATE_AND_TIME_TYPE: &str = "LDATE_AND_TIME";
pub const LONG_DATE_AND_TIME_TYPE_SHORTENED: &str = "LDT";
pub const TIME_OF_DAY_TYPE: &str = "TIME_OF_DAY";
pub const SHORT_TIME_OF_DAY_TYPE: &str = "TOD";
pub const LONG_TIME_OF_DAY_TYPE: &str = "LTIME_OF_DAY";
pub const LONG_TIME_OF_DAY_TYPE_SHORTENED: &str = "LTOD";
pub const ULINT_TYPE: &str = "ULINT";
pub const REAL_TYPE: &str = "REAL";
pub const LREAL_TYPE: &str = "LREAL";
pub const STRING_TYPE: &str = "STRING";
pub const WSTRING_TYPE: &str = "WSTRING";
pub const CHAR_TYPE: &str = "CHAR";
pub const WCHAR_TYPE: &str = "WCHAR";
pub const VOID_TYPE: &str = "VOID";
pub const VOID_INTERNAL_NAME: &str = "__VOID";
pub const __VLA_TYPE: &str = "__VLA";

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct DataType {
    pub name: String,
    /// the initial value defined on the TYPE-declaration
    pub initial_value: Option<ConstId>,
    pub information: DataTypeInformation,
    pub nature: TypeNature,
    pub location: SourceLocation,
}

impl Hash for DataType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.nature.hash(state);
        self.location.hash(state);
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.nature == other.nature && self.location == other.location
    }
}

impl Eq for DataType {}

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
        type_nature.derives_from(nature)
    }

    pub fn is_method(&self) -> bool {
        self.information.is_method()
    }

    pub fn is_void(&self) -> bool {
        self.information.is_void()
    }

    pub fn is_numerical(&self) -> bool {
        self.nature.is_numerical()
    }

    pub fn is_real(&self) -> bool {
        self.nature.is_real()
    }

    pub fn is_bit(&self) -> bool {
        self.nature.is_bit()
    }

    /// returns true if this type is an internal, auto-generated type
    pub fn is_internal(&self) -> bool {
        self.location.is_builtin_internal()
    }

    pub fn is_struct(&self) -> bool {
        self.get_type_information().is_struct()
    }

    /// returns true if this type is an array
    pub fn is_array(&self) -> bool {
        self.get_type_information().is_array()
    }

    /// returns true if this type is an enum
    pub fn is_enum(&self) -> bool {
        self.get_type_information().is_enum()
    }

    pub fn is_vla(&self) -> bool {
        self.get_type_information().is_vla()
    }

    pub fn is_pointer(&self) -> bool {
        self.get_type_information().is_pointer()
    }

    pub fn is_ptr_sized_int(&self) -> bool {
        self.get_type_information().is_ptr_sized_int()
    }

    pub fn is_type_safe_pointer(&self) -> bool {
        self.get_type_information().is_type_safe_pointer()
    }

    /// returns true if this type is an array, struct or string
    pub fn is_aggregate_type(&self) -> bool {
        self.get_type_information().is_aggregate()
    }

    pub fn is_string(&self) -> bool {
        self.get_type_information().is_string()
    }

    pub fn get_nature(&self) -> TypeNature {
        self.nature
    }

    pub fn find_member(&self, name: &str) -> Option<&VariableIndexEntry> {
        match self.get_type_information() {
            DataTypeInformation::Struct { members, .. }
            | DataTypeInformation::Enum { variants: members, .. } => {
                members.iter().find(|member| member.get_name().eq_ignore_ascii_case(name))
            }
            _ => None,
        }
    }

    pub fn get_struct_members(&self) -> &[VariableIndexEntry] {
        match self.get_type_information() {
            DataTypeInformation::Struct { members, .. } => members,
            _ => &[],
        }
    }

    pub fn get_members(&self) -> &[VariableIndexEntry] {
        match self.get_type_information() {
            DataTypeInformation::Struct { members, .. }
            | DataTypeInformation::Enum { variants: members, .. } => members,
            _ => &[],
        }
    }

    pub fn find_declared_parameter_by_location(&self, location: u32) -> Option<&VariableIndexEntry> {
        if let DataTypeInformation::Struct { members, .. } = self.get_type_information() {
            members
                .iter()
                .filter(|item| item.is_parameter() && !item.is_variadic())
                .find(|member| member.get_location_in_parent() == location)
        } else {
            None
        }
    }

    pub fn find_variadic_member(&self) -> Option<&VariableIndexEntry> {
        if let DataTypeInformation::Struct { members, .. } = self.get_type_information() {
            members.iter().find(|member| member.is_variadic())
        } else {
            None
        }
    }

    pub fn find_return_variable(&self) -> Option<&VariableIndexEntry> {
        if let DataTypeInformation::Struct { members, .. } = self.get_type_information() {
            members.iter().find(|member| member.is_return())
        } else {
            None
        }
    }

    pub fn is_compatible_with_type(&self, other: &DataType) -> bool {
        match self.nature {
            TypeNature::Real
            | TypeNature::Int
            | TypeNature::Signed
            | TypeNature::Unsigned
            | TypeNature::Duration
            | TypeNature::Date
            | TypeNature::Bit => {
                other.is_numerical()
                    || matches!(other.nature, TypeNature::Bit | TypeNature::Date | TypeNature::Duration)
            }
            TypeNature::Char => matches!(other.nature, TypeNature::Char | TypeNature::String),
            TypeNature::String => matches!(other.nature, TypeNature::String),
            TypeNature::Any => true,
            TypeNature::Derived => matches!(other.nature, TypeNature::Derived),
            TypeNature::__VLA => matches!(other.nature, TypeNature::__VLA),
            _ => false,
        }
    }

    pub fn get_enum_variants(&self) -> Option<&Vec<VariableIndexEntry>> {
        self.information.get_enum_variants()
    }

    pub(crate) fn is_backed_by_struct(&self) -> bool {
        if let DataTypeInformation::Struct { source: StructSource::Pou(pou_type), .. } =
            self.get_type_information()
        {
            pou_type.is_stateful()
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VarArgs {
    Sized(Option<String>),
    Unsized(Option<String>),
}

impl VarArgs {
    pub fn is_sized(&self) -> bool {
        matches!(self, VarArgs::Sized(..))
    }

    pub fn as_typed(&self, new_type: &str) -> VarArgs {
        match self {
            VarArgs::Sized(Some(_)) => VarArgs::Sized(Some(new_type.to_string())),
            VarArgs::Unsized(Some(_)) => VarArgs::Unsized(Some(new_type.to_string())),
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Enum for ranges and aggregate type sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeSize {
    LiteralInteger(i64),
    ConstExpression(ConstId),
    Undetermined,
}

impl TypeSize {
    pub fn from_literal(v: i64) -> TypeSize {
        TypeSize::LiteralInteger(v)
    }

    pub fn from_expression(id: ConstId) -> TypeSize {
        TypeSize::ConstExpression(id)
    }

    /// tries to compile-time evaluate the size-expression to an i64
    pub fn as_int_value(&self, index: &Index) -> Result<i64, String> {
        match self {
            TypeSize::LiteralInteger(v) => Ok(*v),
            TypeSize::ConstExpression(id) => {
                index.get_const_expressions().get_constant_int_statement_value(id).map(|it| it as i64)
            }
            TypeSize::Undetermined => Ok(POINTER_SIZE as i64),
        }
    }

    /// returns the const expression represented by this TypeSize or None if this TypeSize
    /// is a compile-time literal
    pub fn as_const_expression<'i>(&self, index: &'i Index) -> Option<&'i AstNode> {
        match self {
            TypeSize::LiteralInteger(_) => None,
            TypeSize::ConstExpression(id) => index.get_const_expressions().get_constant_statement(id),
            TypeSize::Undetermined => unreachable!(),
        }
    }

    /// Converts this TypeSize to an AstNode, creating a literal for integer values
    /// or returning the stored const expression
    pub fn to_ast_node(&self, index: &Index, id_provider: &IdProvider) -> Option<AstNode> {
        match self {
            TypeSize::LiteralInteger(v) => Some(AstNode::new_literal(
                AstLiteral::new_integer(*v as i128),
                id_provider.clone().next_id(),
                SourceLocation::internal(),
            )),
            TypeSize::ConstExpression(id) => {
                index.get_const_expressions().get_constant_statement(id).cloned()
            }
            TypeSize::Undetermined => None,
        }
    }
}

/// indicates where this Struct origins from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructSource {
    OriginalDeclaration,
    Pou(PouType),
    Internal(InternalType),
}

impl StructSource {
    pub fn get_type_nature(&self) -> TypeNature {
        match self {
            StructSource::Internal(InternalType::VariableLengthArray { .. }) => TypeNature::__VLA,
            _ => TypeNature::Derived,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InternalType {
    VariableLengthArray { inner_type_name: String, ndims: usize },
    __VLA, // used for error-reporting only
}

type TypeId = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum DataTypeInformation {
    Struct {
        name: TypeId,
        members: Vec<VariableIndexEntry>,
        source: StructSource,
    },
    Enum {
        name: TypeId,
        referenced_type: TypeId,
        variants: Vec<VariableIndexEntry>,
    },
    Array {
        name: TypeId,
        inner_type_name: TypeId,
        dimensions: Vec<Dimension>,
    },
    Pointer {
        name: TypeId,
        inner_type_name: TypeId,
        auto_deref: Option<AutoDerefType>,
        type_safe: bool,
        is_function: bool,
    },
    Integer {
        name: TypeId,
        signed: bool,
        /// the number of bit stored in memory
        size: u32,
        /// the numer of bits represented by this type (may differ from the num acutally stored)
        semantic_size: Option<u32>,
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
        sub_range: Range<TypeSize>,
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
            DataTypeInformation::String { encoding: StringEncoding::Utf8, .. } => "STRING",
            DataTypeInformation::String { encoding: StringEncoding::Utf16, .. } => "WSTRING",
            DataTypeInformation::Void => "VOID",
        }
    }

    pub fn get_inner_name(&self) -> &str {
        match self {
            DataTypeInformation::Pointer { inner_type_name, .. } => inner_type_name,
            _ => self.get_name(),
        }
    }

    pub fn is_void(&self) -> bool {
        matches!(self, DataTypeInformation::Void)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, DataTypeInformation::String { .. })
    }

    pub fn is_string_utf8(&self) -> bool {
        matches!(self, DataTypeInformation::String { encoding: StringEncoding::Utf8, .. })
    }

    pub fn is_string_utf16(&self) -> bool {
        matches!(self, DataTypeInformation::String { encoding: StringEncoding::Utf16, .. })
    }

    pub fn is_character(&self) -> bool {
        match self {
            DataTypeInformation::Integer { name, .. } => name == WCHAR_TYPE || name == CHAR_TYPE,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        // includes enums as they are represented as integers internally
        matches!(self, DataTypeInformation::Integer { .. } | DataTypeInformation::Enum { .. })
    }

    pub fn is_ptr_sized_int(&self) -> bool {
        matches!(self, DataTypeInformation::Integer { size: 64, .. })
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, DataTypeInformation::Integer { semantic_size: Some(1), .. })
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { .. })
    }

    pub fn is_type_safe_pointer(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { type_safe: true, .. })
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

    pub fn is_vla(&self) -> bool {
        matches!(
            self,
            DataTypeInformation::Struct {
                source: StructSource::Internal(InternalType::VariableLengthArray { .. }),
                ..
            }
        )
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, DataTypeInformation::Enum { .. })
    }

    pub fn is_numerical(&self) -> bool {
        matches!(
            self,
            DataTypeInformation::Integer { .. }
                | DataTypeInformation::Float { .. }
                | &DataTypeInformation::Enum { .. } // internally an enum is represented as a DINT
        )
    }

    pub fn is_function(&self) -> bool {
        matches!(self, DataTypeInformation::Struct { source: StructSource::Pou(PouType::Function), .. })
    }

    pub fn is_method(&self) -> bool {
        matches!(self, DataTypeInformation::Struct { source: StructSource::Pou(PouType::Method { .. }), .. })
    }

    pub fn is_class(&self) -> bool {
        matches!(self, DataTypeInformation::Struct { source: StructSource::Pou(PouType::Class), .. })
    }

    pub fn is_function_block(&self) -> bool {
        matches!(self, DataTypeInformation::Struct { source: StructSource::Pou(PouType::FunctionBlock), .. })
    }

    pub fn get_dimension_count(&self) -> Option<usize> {
        match self {
            DataTypeInformation::Array { dimensions, .. } => Some(dimensions.len()),
            DataTypeInformation::Struct {
                source: StructSource::Internal(InternalType::VariableLengthArray { ndims, .. }),
                ..
            } => Some(*ndims),

            _ => None,
        }
    }

    pub fn get_dimensions(&self) -> Option<&Vec<Dimension>> {
        match self {
            DataTypeInformation::Array { dimensions, .. } => Some(dimensions),
            _ => None,
        }
    }

    pub fn get_vla_referenced_type(&self) -> Option<&str> {
        let DataTypeInformation::Struct {
            source: StructSource::Internal(InternalType::VariableLengthArray { inner_type_name, .. }),
            ..
        } = self
        else {
            return None;
        };

        Some(inner_type_name)
    }

    pub fn is_generic(&self, index: &Index) -> bool {
        match self {
            DataTypeInformation::Array { inner_type_name, .. }
            | DataTypeInformation::Pointer { inner_type_name, .. }
            | DataTypeInformation::Alias { referenced_type: inner_type_name, .. } => index
                .find_effective_type_by_name(inner_type_name)
                .map(|dt| dt.get_type_information().is_generic(index))
                .unwrap_or(false),
            DataTypeInformation::Generic { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the variable was declared as `REFERENCE TO`, e.g. `foo : REFERENCE TO DINT`.
    pub fn is_reference_to(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { auto_deref: Some(AutoDerefType::Reference), .. })
    }

    /// Returns true if the variable was declared as `REFERENCE TO`, e.g. `foo : REFERENCE TO DINT`.
    pub fn is_alias(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { auto_deref: Some(AutoDerefType::Alias), .. })
    }

    pub fn is_auto_deref(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { auto_deref: Some(_), .. })
    }

    pub fn is_aggregate(&self) -> bool {
        matches!(
            self,
            DataTypeInformation::Struct { .. }
                | DataTypeInformation::Array { .. }
                | DataTypeInformation::String { .. }
        )
    }

    pub fn get_auto_deref_type(&self) -> Option<AutoDerefType> {
        if let DataTypeInformation::Pointer { auto_deref: kind, .. } = self {
            return *kind;
        }

        None
    }

    pub fn is_date_or_time_type(&self) -> bool {
        matches!(self.get_name(), DATE_TYPE | DATE_AND_TIME_TYPE | TIME_OF_DAY_TYPE | TIME_TYPE)
    }

    pub fn is_function_pointer(&self) -> bool {
        matches!(self, DataTypeInformation::Pointer { is_function: true, .. })
    }

    /// returns the number of bits of this type, as understood by IEC61131 (may be smaller than get_size(...))
    pub fn get_semantic_size(&self, index: &Index) -> u32 {
        if let DataTypeInformation::Integer { semantic_size: Some(s), .. } = self {
            return *s;
        }
        self.get_size_in_bits(index).unwrap_or_default()
    }

    /// returns the number of bits used to store this type
    pub fn get_size_in_bits(&self, index: &Index) -> Result<u32> {
        self.get_size(index).map(|it| it.bits())
    }

    pub fn get_size(&self, index: &Index) -> Result<Bytes> {
        self.get_size_recursive(index, &mut FxHashSet::default())
    }

    fn get_size_recursive<'b>(&'b self, index: &'b Index, seen: &mut FxHashSet<&'b str>) -> Result<Bytes> {
        if self.is_struct() && !seen.insert(self.get_name()) {
            return Err(anyhow!("Recursive type detected: {}", self.get_name()));
        }
        let res = match self {
            DataTypeInformation::Integer { size, .. } => Ok(Bytes::from_bits(*size)),
            DataTypeInformation::Float { size, .. } => Ok(Bytes::from_bits(*size)),
            DataTypeInformation::String { size, encoding } => Ok(size
                .as_int_value(index)
                .map(|size| encoding.get_bytes_per_char() * size as u32)
                .map(Bytes::new)
                .unwrap()),
            DataTypeInformation::Struct { members, .. } => members
                .iter()
                .map(|it| it.get_type_name())
                .try_fold(MemoryLocation::new(0), |prev, it| {
                    let type_info: &DataTypeInformation = index.get_type_information_or_void(it);
                    let size = type_info.get_size_recursive(index, seen)?.value();
                    Ok(MemoryLocation::new(prev.value() + size))
                })
                .map(Into::into),
            DataTypeInformation::Array { inner_type_name, dimensions, .. } => {
                let inner_type = index.get_type_information_or_void(inner_type_name);
                let inner_size = inner_type.get_size_in_bits(index)?;
                let element_count: u32 =
                    dimensions.iter().map(|dim| dim.get_length(index).unwrap()).product();
                Ok(Bytes::from_bits(inner_size * element_count))
            }
            DataTypeInformation::Pointer { .. } => Ok(Bytes::from_bits(POINTER_SIZE)),
            DataTypeInformation::Alias { referenced_type, .. }
            | DataTypeInformation::SubRange { referenced_type, .. } => {
                let inner_type = index.get_type_information_or_void(referenced_type);
                inner_type.get_size_recursive(index, seen)
            }
            DataTypeInformation::Enum { referenced_type, .. } => index
                .find_effective_type_info(referenced_type)
                .map(|it| it.get_size(index))
                .unwrap_or_else(|| Ok(Bytes::from_bits(DINT_SIZE))),
            DataTypeInformation::Generic { .. } | DataTypeInformation::Void => Ok(Bytes::from_bits(0)),
        };
        seen.remove(self.get_name());
        res
    }

    /// Returns the String encoding's alignment (character)
    pub fn get_string_character_width(&self, index: &Index) -> Bytes {
        let type_layout = index.get_type_layout();
        match self {
            DataTypeInformation::String { encoding: StringEncoding::Utf8, .. } => type_layout.i8,
            DataTypeInformation::String { encoding: StringEncoding::Utf16, .. } => type_layout.i16,
            _ => unreachable!("Expected string found {}", self.get_name()),
        }
    }

    pub fn get_inner_array_type_name(&self) -> Option<&str> {
        match self {
            DataTypeInformation::Array { inner_type_name, .. } => Some(inner_type_name),
            DataTypeInformation::Struct {
                source: StructSource::Internal(InternalType::VariableLengthArray { inner_type_name, .. }),
                ..
            } => Some(inner_type_name),
            _ => None,
        }
    }

    /// Recursively retrieves all type names for nested arrays.
    ///
    /// This is needed because a nested array such as `foo : ARRAY[1..5] OF ARRAY[5..10] OF DINT`
    /// provides range information for `[1..5]` and `[5..10]` in two different types stored in
    /// the index.
    pub fn get_inner_array_types<'a>(&'a self, types: &mut Vec<&'a DataTypeInformation>, index: &'a Index) {
        if let DataTypeInformation::Array { name, inner_type_name, .. } = self {
            if name != inner_type_name {
                types.push(self);

                if let Some(ty) = index.find_type(inner_type_name).map(DataType::get_type_information) {
                    ty.get_inner_array_types(types, index);
                }
            }
        }
    }

    pub fn get_inner_pointer_type_name(&self) -> Option<&str> {
        match self {
            DataTypeInformation::Pointer { inner_type_name, .. } => Some(inner_type_name),
            _ => None,
        }
    }

    pub fn is_compatible_char_and_string(&self, other: &DataTypeInformation) -> bool {
        match self.get_name() {
            CHAR_TYPE => matches!(other, DataTypeInformation::String { encoding: StringEncoding::Utf8, .. }),
            WCHAR_TYPE => {
                matches!(other, DataTypeInformation::String { encoding: StringEncoding::Utf16, .. })
            }
            _ => false,
        }
    }

    /// Returns the array length if [`DataTypeInformation`] is of variant [`DataTypeInformation::Array`] and
    /// None otherwise.
    ///
    /// For example calling this function on `ARRAY[1..5] OF DINT`, `ARRAY[1..2, 1..5] OF DINT` and
    /// `ARRAY[1..3] OF ARRAY[1..5]` yields `5`, `10` and `15` respectively.
    pub fn get_array_length(&self, index: &Index) -> Option<usize> {
        fn intrinsic_array_type<'index>(index: &'index Index, name: &str) -> &'index DataType {
            let effective_type = index.get_effective_type_or_void_by_name(name);

            match effective_type.get_type_information() {
                DataTypeInformation::Array { inner_type_name, .. } => {
                    intrinsic_array_type(index, inner_type_name)
                }
                _ => effective_type,
            }
        }

        let DataTypeInformation::Array { inner_type_name, .. } = self else { return None };
        let inner_type_info = intrinsic_array_type(index, inner_type_name).get_type_information();
        let inner_type_size = inner_type_info.get_size_in_bits(index).ok()?;
        let arr_size = self.get_size_in_bits(index).ok()?;

        if inner_type_size == 0 {
            return None;
        }

        Some((arr_size / inner_type_size) as usize)
    }

    pub fn get_enum_variants(&self) -> Option<&Vec<VariableIndexEntry>> {
        if let DataTypeInformation::Enum { variants, .. } = self {
            return Some(variants);
        }

        None
    }

    pub fn get_struct_source(&self) -> Option<StructSource> {
        match &self.get_type_information() {
            DataTypeInformation::Struct { source, .. } => Some(source.clone()),
            _ => None,
        }
    }

    pub fn get_method_owner(&self) -> Option<String> {
        match self.get_struct_source() {
            Some(StructSource::Pou(PouType::Method { parent, .. })) => Some(parent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

    pub fn get_range(&self, index: &Index) -> Result<Range<i64>, String> {
        let start = self.start_offset.as_int_value(index)?;
        let end = self.end_offset.as_int_value(index)?;
        Ok(start..end)
    }

    /// Identical to [`get_range`] except for adding 1 to the end of the range.
    /// For example if the start and end values are 1 and 5 respectively, the range will be `1..6`
    ///
    /// Primarily used by Inkwell which calculates the array length as `end - start` which would
    /// generate an off-by-one error in the array size with [`get_range`] because ST ranges are inclusive.
    pub fn get_range_plus_one(&self, index: &Index) -> Result<Range<i64>, String> {
        let start = self.start_offset.as_int_value(index)?;
        let end = self.end_offset.as_int_value(index)?;
        Ok(start..end + 1)
    }

    pub fn get_range_inclusive(&self, index: &Index) -> Result<RangeInclusive<i64>, String> {
        let start = self.start_offset.as_int_value(index)?;
        let end = self.end_offset.as_int_value(index)?;
        Ok(start..=end)
    }

    pub fn is_undetermined(&self) -> bool {
        matches!((self.start_offset, self.end_offset), (TypeSize::Undetermined, TypeSize::Undetermined))
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
            name: VOID_INTERNAL_NAME.into(),
            initial_value: None,
            information: DataTypeInformation::Void,
            nature: TypeNature::Any,
            location: SourceLocation::internal(),
        },
        DataType {
            name: "__VLA".into(),
            initial_value: None,
            information: DataTypeInformation::Struct {
                name: "VARIABLE LENGTH ARRAY".to_string(),
                members: vec![],
                source: StructSource::Internal(InternalType::__VLA),
            },
            nature: TypeNature::__VLA,
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
        },
        DataType {
            name: REAL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Float { name: REAL_TYPE.into(), size: REAL_SIZE },
            nature: TypeNature::Real,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LREAL_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Float { name: LREAL_TYPE.into(), size: LREAL_SIZE },
            nature: TypeNature::Real,
            location: SourceLocation::internal(),
        },
        DataType {
            name: STRING_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: TypeSize::from_literal((DEFAULT_STRING_LEN + 1).into()),
                encoding: StringEncoding::Utf8,
            },
            nature: TypeNature::String,
            location: SourceLocation::internal(),
        },
        DataType {
            name: WSTRING_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::String {
                size: TypeSize::from_literal((DEFAULT_STRING_LEN + 1).into()),
                encoding: StringEncoding::Utf16,
            },
            nature: TypeNature::String,
            location: SourceLocation::internal(),
        },
        DataType {
            name: SHORT_DATE_AND_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_DATE_AND_TIME_TYPE.into(),
                referenced_type: DATE_AND_TIME_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_DATE_AND_TIME_TYPE_SHORTENED.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_DATE_AND_TIME_TYPE_SHORTENED.into(),
                referenced_type: DATE_AND_TIME_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_DATE_AND_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_DATE_AND_TIME_TYPE.into(),
                referenced_type: DATE_AND_TIME_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: SHORT_DATE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_DATE_TYPE.into(),
                referenced_type: DATE_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_DATE_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_DATE_TYPE.into(),
                referenced_type: DATE_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_DATE_TYPE_SHORTENED.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_DATE_TYPE_SHORTENED.into(),
                referenced_type: DATE_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: SHORT_TIME_OF_DAY_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_TIME_OF_DAY_TYPE.into(),
                referenced_type: TIME_OF_DAY_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_TIME_OF_DAY_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_TIME_OF_DAY_TYPE.into(),
                referenced_type: TIME_OF_DAY_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_TIME_OF_DAY_TYPE_SHORTENED.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_TIME_OF_DAY_TYPE_SHORTENED.into(),
                referenced_type: TIME_OF_DAY_TYPE.into(),
            },
            nature: TypeNature::Date,
            location: SourceLocation::internal(),
        },
        DataType {
            name: SHORT_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: SHORT_TIME_TYPE.into(),
                referenced_type: TIME_TYPE.into(),
            },
            nature: TypeNature::Duration,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_TIME_TYPE.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_TIME_TYPE.into(),
                referenced_type: TIME_TYPE.into(),
            },
            nature: TypeNature::Duration,
            location: SourceLocation::internal(),
        },
        DataType {
            name: LONG_TIME_TYPE_SHORTENED.into(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: LONG_TIME_TYPE_SHORTENED.into(),
                referenced_type: TIME_TYPE.into(),
            },
            nature: TypeNature::Duration,
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            location: SourceLocation::internal(),
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
            TypeSize::LiteralInteger(size) => (*size).try_into().unwrap(),
            TypeSize::ConstExpression(_) => todo!("String rank with CONSTANTS"),
            TypeSize::Undetermined => unreachable!("Strings will never have undetermined size"),
        },
        DataTypeInformation::Enum { referenced_type, .. } => {
            index.find_effective_type_info(referenced_type).map(|it| get_rank(it, index)).unwrap_or(DINT_SIZE)
        }
        DataTypeInformation::SubRange { name, .. } | DataTypeInformation::Alias { name, .. } => {
            get_rank(index.get_intrinsic_type_by_name(name).get_type_information(), index)
        }
        _ => type_information.get_size_in_bits(index).unwrap_or_default(),
    }
}

/// Returns true if provided types have the same type nature
/// i.e. Both are numeric or both are floats
pub fn is_same_type_class(ltype: &DataTypeInformation, rtype: &DataTypeInformation, index: &Index) -> bool {
    let ltype = index.get_intrinsic_type_information(ltype);
    let rtype = index.get_intrinsic_type_information(rtype);

    match ltype {
        DataTypeInformation::Integer { .. } => matches!(rtype, DataTypeInformation::Integer { .. }),
        DataTypeInformation::Float { .. } => matches!(rtype, DataTypeInformation::Float { .. }),
        DataTypeInformation::String { encoding: lenc, .. } => {
            matches!(rtype, DataTypeInformation::String { encoding, .. } if encoding == lenc)
        }

        // We have to handle 2 different cases here:
        // 1. foo := ADR(bar)
        // 2. foo := REF(bar)
        DataTypeInformation::Pointer { .. } => match rtype {
            // Case 1: ADR(bar) returns a LWORD value, thus check if we're working with a LWORD
            DataTypeInformation::Integer { size, .. } => *size == POINTER_SIZE,

            // Case 2:
            // REF(bar) returns a pointer, thus deduce their inner types and check if they're equal
            DataTypeInformation::Pointer { .. } => {
                let ldetails = index.find_elementary_pointer_type(ltype);
                let rdetails = index.find_elementary_pointer_type(rtype);

                is_same_type_class(ldetails, rdetails, index)
            }

            // If nothing applies we can assume the types to be different
            _ => false,
        },
        DataTypeInformation::Array { inner_type_name: l_inner_type_name, .. } => match rtype {
            DataTypeInformation::Array { inner_type_name: r_inner_type_name, .. } => {
                let l_inner_type = index.get_type_information_or_void(l_inner_type_name);
                let r_inner_type = index.get_type_information_or_void(r_inner_type_name);
                is_same_type_class(l_inner_type, r_inner_type, index)
                    && ltype.get_size(index).unwrap_or_default() == rtype.get_size(index).unwrap_or_default()
            }
            _ => false,
        },
        _ => ltype == rtype,
    }
}

/// Returns the bigger of the two provided types
pub fn get_bigger_type<'t, T: DataTypeInformationProvider<'t> + std::convert::From<&'t DataType>>(
    left_type: T,
    right_type: T,
    index: &'t Index,
) -> T {
    let lt = left_type.get_type_information();
    let rt = right_type.get_type_information();

    let ldt = index.get_type(lt.get_name());
    let rdt = index.get_type(rt.get_name());

    // if left and right have the same type, check which ranks higher
    if is_same_type_class(lt, rt, index) {
        if get_rank(lt, index) < get_rank(rt, index) {
            return right_type;
        }
    } else if let (Ok(ldt), Ok(rdt)) = (ldt, rdt) {
        // check is_numerical() on TypeNature e.g. DataTypeInformation::Integer is numerical but also used for CHARS which are not considered as numerical
        if (ldt.is_numerical() && rdt.is_numerical()) && (ldt.is_real() || rdt.is_real()) {
            let real_type = index.get_type_or_panic(REAL_TYPE);
            let real_size = real_type.get_type_information().get_size_in_bits(index).unwrap();
            if lt.get_size_in_bits(index).unwrap_or_default() > real_size
                || rt.get_size_in_bits(index).unwrap_or_default() > real_size
            {
                return index.get_type_or_panic(LREAL_TYPE).into();
            } else {
                return real_type.into();
            }
        } else if lt.is_string() & rt.is_character() {
            return left_type;
        } else if rt.is_string() & lt.is_character() {
            return right_type;
        }
    }

    left_type
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
        return index.get_type(signed_type).ok().map(|t| t.get_type_information());
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

    suffix.map(|suffix| format!("{type_name}_{suffix}")) // TODO: Naming convention (see plc_util/src/convention.rs)
}

pub fn get_literal_actual_signed_type_name(lit: &AstLiteral, signed: bool) -> Option<&str> {
    // Returns a range with the min and max value of the given type
    macro_rules! is_covered_by {
        ($t:ty, $e:expr) => {
            <$t>::MIN as i128 <= $e as i128 && $e as i128 <= <$t>::MAX as i128
        };
    }

    match lit {
        AstLiteral::Integer(value) => match signed {
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
        AstLiteral::Bool { .. } => Some(BOOL_TYPE),
        AstLiteral::String(StringValue { is_wide: true, .. }) => Some(WSTRING_TYPE),
        AstLiteral::String(StringValue { is_wide: false, .. }) => Some(STRING_TYPE),
        AstLiteral::Real { .. } => Some(LREAL_TYPE),
        AstLiteral::Date { .. } => Some(DATE_TYPE),
        AstLiteral::DateAndTime { .. } => Some(DATE_AND_TIME_TYPE),
        AstLiteral::Time { .. } => Some(TIME_TYPE),
        AstLiteral::TimeOfDay { .. } => Some(TIME_OF_DAY_TYPE),
        _ => None,
    }
}
