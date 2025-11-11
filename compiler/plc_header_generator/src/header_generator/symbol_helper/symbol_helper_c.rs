use crate::header_generator::{header_generator_c::GeneratedHeaderForC, symbol_helper::SymbolHelper};

impl SymbolHelper for GeneratedHeaderForC {
    fn format_global_variables(&self, variables: &[String]) -> String {
        if variables.is_empty() {
            return String::new();
        }

        format!("extern {};", variables.join(";\nextern "))
    }

    fn format_function_parameters(&self, parameters: &[String]) -> String {
        if parameters.is_empty() {
            return String::new();
        }

        parameters.join(", ")
    }

    fn format_functions(&self, functions: &[String]) -> String {
        format_functions_or_user_types(functions)
    }

    fn format_user_types(&self, user_types: &[String]) -> String {
        format_functions_or_user_types(user_types)
    }

    fn format_struct_fields(&self, struct_fields: &[String]) -> String {
        if struct_fields.is_empty() {
            return String::new();
        }

        format!("{};", struct_fields.join(";\n\t"))
    }

    fn format_enum_fields(&self, enum_fields: &[String]) -> String {
        if enum_fields.is_empty() {
            return String::new();
        }

        enum_fields.join(",\n\t")
    }

    fn format_variable_declaration(&self, left: String, right: String) -> String {
        let right = if right.is_empty() { String::new() } else { format!(" = {right}") };
        format!("{left}{right}")
    }

    fn get_reference_symbol(&self) -> String {
        String::from(C_REFERENCE_SYMBOL)
    }

    fn get_variadic_symbol(&self) -> String {
        String::from(C_VARIADIC_SYMBOL)
    }
}

fn format_functions_or_user_types(functions_or_user_types: &[String]) -> String {
    if functions_or_user_types.is_empty() {
        return String::new();
    }

    format!("{};", functions_or_user_types.join(";\n\n"))
}

// ------------------- //
// -- "C" Constants -- //

/// The constant value for the "c" reference symbol
const C_REFERENCE_SYMBOL: &str = "*";

/// The constant value for the "c" variadic symbol
const C_VARIADIC_SYMBOL: &str = "...";

// ------------------- //
