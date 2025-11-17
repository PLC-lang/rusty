use crate::header_generator::ExtendedTypeName;
use plc::typesystem::DataType;

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
    Other,
    UserGenerated,
    Variadic,
}

impl TypeInformation {
    pub const fn new() -> Self {
        TypeInformation { name: String::new(), attribute: TypeAttribute::Other }
    }

    pub fn get_type_name(&self) -> String {
        self.name.clone()
    }
}

pub trait TypeHelper {
    fn get_type_name_for_type(
        &self,
        extended_type_name: &ExtendedTypeName,
        builtin_types: &[DataType],
    ) -> TypeInformation;

    fn get_type_name_for_string(&self, is_wide: &bool) -> String;
}

fn determine_type_attribute(is_variadic: bool, is_user_generated: bool) -> TypeAttribute {
    if is_variadic {
        return TypeAttribute::Variadic;
    }

    if is_user_generated {
        return TypeAttribute::UserGenerated;
    }

    TypeAttribute::Other
}
