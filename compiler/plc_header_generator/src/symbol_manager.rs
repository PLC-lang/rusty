use crate::GenerateLanguage;

pub struct SymbolManager {
    pub language: GenerateLanguage,
}

impl SymbolManager {
    pub const fn new() -> Self {
        SymbolManager { language: GenerateLanguage::C }
    }

    pub fn format_global_variables(&self, variables: &[String]) -> String {
        match &self.language {
            GenerateLanguage::C => format_global_variables_in_c(variables),
            _ => panic!("{:?} for formatting global variables is not yet supported!", self.language),
        }
    }

    pub fn format_function_parameters(&self, parameters: &[String]) -> String {
        match &self.language {
            GenerateLanguage::C => format_function_parameters_in_c(parameters),
            _ => panic!("{:?} for formatting function parameters is not yet supported!", self.language),
        }
    }

    pub fn format_functions(&self, functions: &[String]) -> String {
        match &self.language {
            GenerateLanguage::C => format_functions_or_user_types_in_c(functions),
            _ => panic!("{:?} for formatting functions is not yet supported!", self.language),
        }
    }

    pub fn format_user_types(&self, user_types: &[String]) -> String {
        match &self.language {
            GenerateLanguage::C => format_functions_or_user_types_in_c(user_types),
            _ => panic!("{:?} for formatting user types is not yet supported!", self.language),
        }
    }

    pub fn format_struct_fields(&self, struct_fields: &[String]) -> String {
        match &self.language {
            GenerateLanguage::C => format_struct_fields_in_c(struct_fields),
            _ => panic!("{:?} for formatting structs is not yet supported!", self.language),
        }
    }

    pub fn format_enum_fields(&self, enum_fields: &[String]) -> String {
        match &self.language {
            GenerateLanguage::C => format_enum_fields_in_c(enum_fields),
            _ => panic!("{:?} for formatting enum fields is not yet supported!", self.language),
        }
    }

    pub fn format_variable_declaration(&self, left: String, right: String) -> String {
        match &self.language {
            GenerateLanguage::C => format_variable_declaration_in_c(left, right),
            _ => panic!("{:?} for formatting variable declarations is not yet supported!", self.language),
        }
    }

    pub fn get_reference_symbol(&self) -> String {
        match &self.language {
            GenerateLanguage::C => String::from(C_REFERENCE_SYMBOL),
            _ => panic!("{:?} for getting the reference symbol is not yet supported!", self.language),
        }
    }

    pub fn get_variadic_symbol(&self) -> String {
        match &self.language {
            GenerateLanguage::C => String::from(C_VARIADIC_SYMBOL),
            _ => panic!("{:?} for getting the variadic symbol is not yet supported!", self.language),
        }
    }
}

fn format_global_variables_in_c(variables: &[String]) -> String {
    if variables.is_empty() {
        return String::new();
    }

    format!("extern {};", variables.join(";\nextern "))
}

fn format_function_parameters_in_c(parameters: &[String]) -> String {
    if parameters.is_empty() {
        return String::new();
    }

    parameters.join(", ")
}

fn format_functions_or_user_types_in_c(functions_or_user_types: &[String]) -> String {
    if functions_or_user_types.is_empty() {
        return String::new();
    }

    format!("{};", functions_or_user_types.join(";\n\n"))
}

fn format_struct_fields_in_c(struct_fields: &[String]) -> String {
    if struct_fields.is_empty() {
        return String::new();
    }

    format!("{};", struct_fields.join(";\n\t"))
}

fn format_enum_fields_in_c(enum_fields: &[String]) -> String {
    if enum_fields.is_empty() {
        return String::new();
    }

    enum_fields.join(",\n\t")
}

fn format_variable_declaration_in_c(left: String, right: String) -> String {
    let right = if right.is_empty() { String::new() } else { format!(" = {right}") };
    format!("{left}{right}")
}

impl Default for SymbolManager {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------- //
// -- "C" Constants -- //

/// The constant value for the "c" reference symbol
const C_REFERENCE_SYMBOL: &str = "*";

/// The constant value for the "c" variadic symbol
const C_VARIADIC_SYMBOL: &str = "...";

// ------------------- //
