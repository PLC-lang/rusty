use crate::header_generator::{
    header_generator_c::GeneratedHeaderForC,
    type_helper::{determine_type_attribute, extract_string_size, TypeHelper, TypeInformation},
    ExtendedTypeName,
};
use plc::typesystem::{DataType, DataTypeInformation, StringEncoding, BOOL_TYPE, REAL_SIZE};
use plc_ast::ast::TypeNature;

impl TypeHelper for GeneratedHeaderForC {
    fn get_type_name_for_type(
        &self,
        extended_type_name: &ExtendedTypeName,
        builtin_types: &[DataType],
    ) -> TypeInformation {
        if extended_type_name.type_name.is_empty() {
            return TypeInformation {
                name: String::from(C_VOID),
                attribute: determine_type_attribute(
                    extended_type_name.is_variadic,
                    extended_type_name.is_sized_variadic,
                    false,
                    None,
                ),
            };
        }

        let Some(builtin_type) =
            builtin_types.iter().find(|builtin_type| builtin_type.name == extended_type_name.type_name)
        else {
            // This is a user-generated type
            return TypeInformation {
                name: extended_type_name.type_name.to_string(),
                attribute: determine_type_attribute(
                    extended_type_name.is_variadic,
                    extended_type_name.is_sized_variadic,
                    true,
                    None,
                ),
            };
        };

        match &builtin_type.information {
            DataTypeInformation::Integer { signed, size, .. } => {
                // Booleans have their own type
                if extended_type_name.type_name == BOOL_TYPE {
                    return TypeInformation {
                        name: String::from(C_BOOL),
                        attribute: determine_type_attribute(
                            extended_type_name.is_variadic,
                            extended_type_name.is_sized_variadic,
                            false,
                            None,
                        ),
                    };
                }

                // Dates have their own type
                if builtin_type.nature == TypeNature::Date || builtin_type.nature == TypeNature::Duration {
                    return TypeInformation {
                        name: String::from(C_TIME),
                        attribute: determine_type_attribute(
                            extended_type_name.is_variadic,
                            extended_type_name.is_sized_variadic,
                            false,
                            None,
                        ),
                    };
                }

                let signed_operator = if *signed { "" } else { "u" };
                let constructed_type_name = format!("{signed_operator}int{size}_t");

                TypeInformation {
                    name: constructed_type_name,
                    attribute: determine_type_attribute(
                        extended_type_name.is_variadic,
                        extended_type_name.is_sized_variadic,
                        false,
                        None,
                    ),
                }
            }
            DataTypeInformation::Float { size, .. } => {
                if *size == REAL_SIZE {
                    TypeInformation {
                        name: String::from(C_FLOAT),
                        attribute: determine_type_attribute(
                            extended_type_name.is_variadic,
                            extended_type_name.is_sized_variadic,
                            false,
                            None,
                        ),
                    }
                } else {
                    TypeInformation {
                        name: String::from(C_DOUBLE),
                        attribute: determine_type_attribute(
                            extended_type_name.is_variadic,
                            extended_type_name.is_sized_variadic,
                            false,
                            None,
                        ),
                    }
                }
            }
            DataTypeInformation::Alias { referenced_type, .. } => {
                let referenced_data_type_info = ExtendedTypeName {
                    type_name: referenced_type.to_string(),
                    is_variadic: false,
                    is_sized_variadic: false,
                };

                self.get_type_name_for_type(&referenced_data_type_info, builtin_types)
            }
            DataTypeInformation::String { encoding, size } => match encoding {
                StringEncoding::Utf8 => TypeInformation {
                    name: self.get_type_name_for_string(&false),
                    attribute: determine_type_attribute(
                        extended_type_name.is_variadic,
                        extended_type_name.is_sized_variadic,
                        false,
                        Some(extract_string_size(size)),
                    ),
                },
                StringEncoding::Utf16 => TypeInformation {
                    name: self.get_type_name_for_string(&true),
                    attribute: determine_type_attribute(
                        extended_type_name.is_variadic,
                        extended_type_name.is_sized_variadic,
                        false,
                        Some(extract_string_size(size)),
                    ),
                },
            },
            _ => {
                log::debug!("{} this type is not yet supported!", extended_type_name.type_name);
                TypeInformation::new()
            }
        }
    }

    fn get_type_name_for_string(&self, is_wide: &bool) -> String {
        if *is_wide {
            return String::from(C_INT16);
        }

        String::from(C_CHAR)
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

/// The constant value for the "c" type: int16
const C_INT16: &str = "int16_t";

/// The constant value for the "c" type: char
const C_CHAR: &str = "char";

// ------------------- //
