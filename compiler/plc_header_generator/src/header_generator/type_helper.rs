use crate::header_generator::ExtendedTypeName;
use plc::typesystem::{DataType, TypeSize};

mod type_helper_c;

pub struct TypeInformation {
    pub name: String,
    pub attribute: TypeAttribute,
}

impl Default for TypeInformation {
    fn default() -> Self {
        Self::new()
    }
}

pub enum TypeAttribute {
    /// This type is an unremarkable type
    Default,
    UserGenerated,
    Variadic(bool),
    Array(i128),
}

impl TypeInformation {
    pub const fn new() -> Self {
        TypeInformation { name: String::new(), attribute: TypeAttribute::Default }
    }

    pub fn get_type_name(&self) -> String {
        self.name.clone()
    }
}

pub trait TypeHelper {
    /// Given an extended type name and all of the defined builtin types, this will determine the type name for this type.
    ///
    /// ---
    ///
    /// This must return a [TypeInformation] object, that contains the type name in the language this is implemented for and
    /// some additional information about the type (whether it is UserGenerated, Variadic or Default)
    fn get_type_name_for_type(
        &self,
        extended_type_name: &ExtendedTypeName,
        builtin_types: &[DataType],
    ) -> TypeInformation;

    /// Given a boolean indicating whether or not this is a wide string, this will determine the type name for this string type.
    ///
    /// ---
    ///
    /// This must return a [String] that specifies the type name in the language that this is implemented for.
    fn get_type_name_for_string(&self, is_wide: &bool) -> String;
}

fn determine_type_attribute(
    is_variadic: bool,
    is_sized_variadic: bool,
    is_user_generated: bool,
    array_size_option: Option<i128>,
) -> TypeAttribute {
    if is_variadic || is_sized_variadic {
        return TypeAttribute::Variadic(is_sized_variadic);
    }

    if is_user_generated {
        return TypeAttribute::UserGenerated;
    }

    if let Some(array_size) = array_size_option {
        return TypeAttribute::Array(array_size);
    }

    TypeAttribute::Default
}

fn extract_string_size(type_size: &TypeSize) -> i128 {
    match type_size {
        TypeSize::LiteralInteger(size) => (*size).into(),
        _ => i128::default(),
    }
}
