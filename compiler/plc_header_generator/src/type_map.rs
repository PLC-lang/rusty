use plc::typesystem::{DataType, DataTypeInformation, BOOL_TYPE, REAL_SIZE};
use plc_ast::ast::TypeNature;

use crate::GenerateLanguage;

pub fn get_type_name_for_type_by_language(language: GenerateLanguage, type_name: &str, builtin_types: &[DataType]) -> String {

    // TODO: Handle complex and user-defined types

    match language {
        GenerateLanguage::C => get_type_name_for_type_in_c(type_name, builtin_types),
        _ => String::new()
    }
}

fn get_type_name_for_type_in_c(type_name: &str, builtin_types: &[DataType]) -> String {
    if type_name.is_empty() {
        return String::from(C_VOID);
    }

    let builtin_type = builtin_types
        .iter()
        .find(|builtin_type| builtin_type.name == type_name);

    if builtin_type.is_none() {
        // TODO: Maybe we should log here? But this probably means it is a complex type...
        return String::from(type_name);
    }

    let builtin_type = builtin_type.unwrap();
    match &builtin_type.information {
        DataTypeInformation::Integer { signed, size, .. } => {
            // Booleans have their own type
            if type_name == BOOL_TYPE {
                return String::from(C_BOOL);
            }

            // Dates have their own type
            if builtin_type.nature == TypeNature::Date || builtin_type.nature == TypeNature::Duration {
                return String::from(C_TIME);
            }

            let signed_operator = if *signed { "" } else { "u" };
            let constructed_type_name = format!("{signed_operator}int{size}_t");

            constructed_type_name
        },
        DataTypeInformation::Float { size, .. } => {
            if *size == REAL_SIZE { String::from(C_FLOAT) } else { String::from(C_DOUBLE) }
        },
        DataTypeInformation::Alias { referenced_type, .. } => get_type_name_for_type_in_c(referenced_type, builtin_types),
        _ => {
            log::debug!("{type_name} this type is not yet supported!");
            String::new()
        }
    }
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

// ------------------- //
