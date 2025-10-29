use plc::typesystem::{DataType, DataTypeInformation, BOOL_TYPE, REAL_SIZE};
use plc_ast::ast::{self, TypeNature, UserTypeDeclaration};

use crate::GenerateLanguage;

pub struct TypeManager {
    pub language: GenerateLanguage,
}

impl TypeManager {
    pub const fn new() -> Self {
        TypeManager { language: GenerateLanguage::C }
    }

    pub fn get_type_name_for_type(&self, type_name: &str, builtin_types: &[DataType]) -> (bool, String) {
        match self.language {
            GenerateLanguage::C => get_type_name_for_type_in_c(type_name, builtin_types),
            _ => (false, String::new()),
        }
    }

    pub fn get_type_name_for_string(&self, is_wide: &bool) -> String {
        match self.language {
            GenerateLanguage::C => get_type_name_for_string_in_c(is_wide),
            _ => String::new(),
        }
    }

    pub fn user_type_can_be_declared_outside_of_a_function(&self, user_type: &UserTypeDeclaration) -> bool {
        match self.language {
            GenerateLanguage::C => user_type_can_be_declared_outside_of_a_function_in_c(user_type),
            _ => false,
        }
    }
}

impl Default for TypeManager {
    fn default() -> Self {
        Self::new()
    }
}

fn get_type_name_for_type_in_c(type_name: &str, builtin_types: &[DataType]) -> (bool, String) {
    if type_name.is_empty() {
        return (false, String::from(C_VOID));
    }

    let builtin_type = builtin_types.iter().find(|builtin_type| builtin_type.name == type_name);

    if builtin_type.is_none() {
        // This is a user-generated type
        return (true, String::from(type_name));
    }

    let builtin_type = builtin_type.unwrap();
    match &builtin_type.information {
        DataTypeInformation::Integer { signed, size, .. } => {
            // Booleans have their own type
            if type_name == BOOL_TYPE {
                return (false, String::from(C_BOOL));
            }

            // Dates have their own type
            if builtin_type.nature == TypeNature::Date || builtin_type.nature == TypeNature::Duration {
                return (false, String::from(C_TIME));
            }

            let signed_operator = if *signed { "" } else { "u" };
            let constructed_type_name = format!("{signed_operator}int{size}_t");

            (false, constructed_type_name)
        }
        DataTypeInformation::Float { size, .. } => {
            if *size == REAL_SIZE {
                (false, String::from(C_FLOAT))
            } else {
                (false, String::from(C_DOUBLE))
            }
        }
        DataTypeInformation::Alias { referenced_type, .. } => {
            get_type_name_for_type_in_c(referenced_type, builtin_types)
        }
        _ => {
            log::debug!("{type_name} this type is not yet supported!");
            (false, String::new())
        }
    }
}

fn get_type_name_for_string_in_c(is_wide: &bool) -> String {
    if *is_wide {
        return String::from(C_INT16);
    }

    String::from(C_CHAR)
}

fn user_type_can_be_declared_outside_of_a_function_in_c(user_type: &UserTypeDeclaration) -> bool {
    !matches!(user_type.data_type, ast::DataType::StringType { .. } | ast::DataType::ArrayType { .. })
}

// ------------------- //
// -- "C" Constants -- //

/// The constant value for the "c" type: float
const C_FLOAT: &str = "float_t";

/// The constant value for the "c" type: double
const C_DOUBLE: &str = "double_t";

/// The constant value for the "c" type: time
const C_TIME: &str = "time_t";

/// The constant value for the "c" type: bool
const C_BOOL: &str = "bool";

/// The constant value for the "c" type: void
const C_VOID: &str = "void";

/// The constant value for the "c" type: int16
const C_INT16: &str = "int16_t";

/// The constant value for the "c" type: char
const C_CHAR: &str = "char";

// ------------------- //
