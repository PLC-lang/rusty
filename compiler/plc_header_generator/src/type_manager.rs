use plc::typesystem::{DataType, DataTypeInformation, StringEncoding, BOOL_TYPE, REAL_SIZE};
use plc_ast::ast::{self, TypeNature, UserTypeDeclaration};

use crate::{header_generator::ExtendedTypeName, GenerateLanguage};

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
}

pub struct TypeManager {
    pub language: GenerateLanguage,
}

impl Default for TypeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeManager {
    pub const fn new() -> Self {
        TypeManager { language: GenerateLanguage::C }
    }

    pub fn get_type_name_for_type(
        &self,
        extended_type_name: &ExtendedTypeName,
        builtin_types: &[DataType],
    ) -> TypeInformation {
        match self.language {
            GenerateLanguage::C => get_type_name_for_type_in_c(extended_type_name, builtin_types),
            _ => TypeInformation::new(),
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

fn get_type_name_for_type_in_c(
    extended_type_name: &ExtendedTypeName,
    builtin_types: &[DataType],
) -> TypeInformation {
    if extended_type_name.type_name.is_empty() {
        return TypeInformation {
            name: String::from(C_VOID),
            attribute: determine_type_attribute(extended_type_name.is_variadic, false),
        };
    }

    let builtin_type =
        builtin_types.iter().find(|builtin_type| builtin_type.name == extended_type_name.type_name);

    if builtin_type.is_none() {
        // This is a user-generated type
        return TypeInformation {
            name: extended_type_name.type_name.to_string(),
            attribute: determine_type_attribute(extended_type_name.is_variadic, true),
        };
    }

    let builtin_type = builtin_type.unwrap();
    match &builtin_type.information {
        DataTypeInformation::Integer { signed, size, .. } => {
            // Booleans have their own type
            if extended_type_name.type_name == BOOL_TYPE {
                return TypeInformation {
                    name: String::from(C_BOOL),
                    attribute: determine_type_attribute(extended_type_name.is_variadic, false),
                };
            }

            // Dates have their own type
            if builtin_type.nature == TypeNature::Date || builtin_type.nature == TypeNature::Duration {
                return TypeInformation {
                    name: String::from(C_TIME),
                    attribute: determine_type_attribute(extended_type_name.is_variadic, false),
                };
            }

            let signed_operator = if *signed { "" } else { "u" };
            let constructed_type_name = format!("{signed_operator}int{size}_t");

            TypeInformation {
                name: constructed_type_name,
                attribute: determine_type_attribute(extended_type_name.is_variadic, false),
            }
        }
        DataTypeInformation::Float { size, .. } => {
            if *size == REAL_SIZE {
                TypeInformation {
                    name: String::from(C_FLOAT),
                    attribute: determine_type_attribute(extended_type_name.is_variadic, false),
                }
            } else {
                TypeInformation {
                    name: String::from(C_DOUBLE),
                    attribute: determine_type_attribute(extended_type_name.is_variadic, false),
                }
            }
        }
        DataTypeInformation::Alias { referenced_type, .. } => {
            let referenced_data_type_info =
                ExtendedTypeName { type_name: referenced_type.to_string(), is_variadic: false };

            get_type_name_for_type_in_c(&referenced_data_type_info, builtin_types)
        }
        DataTypeInformation::String { encoding, .. } => match encoding {
            StringEncoding::Utf8 => TypeInformation {
                name: get_type_name_for_string_in_c(&false),
                attribute: determine_type_attribute(extended_type_name.is_variadic, false),
            },
            StringEncoding::Utf16 => TypeInformation {
                name: get_type_name_for_string_in_c(&true),
                attribute: determine_type_attribute(extended_type_name.is_variadic, false),
            },
        },
        _ => {
            log::debug!("{} this type is not yet supported!", extended_type_name.type_name);
            TypeInformation::new()
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
    !matches!(
        user_type.data_type,
        ast::DataType::StringType { .. }
            | ast::DataType::ArrayType { .. }
            | ast::DataType::PointerType { .. }
            | ast::DataType::SubRangeType { .. }
    )
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
