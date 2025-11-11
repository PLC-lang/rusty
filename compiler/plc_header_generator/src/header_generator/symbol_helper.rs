mod symbol_helper_c;

pub trait SymbolHelper {
    fn format_global_variables(&self, variables: &[String]) -> String;
    fn format_function_parameters(&self, parameters: &[String]) -> String;
    fn format_functions(&self, functions: &[String]) -> String;
    fn format_user_types(&self, user_types: &[String]) -> String;
    fn format_struct_fields(&self, struct_fields: &[String]) -> String;
    fn format_enum_fields(&self, enum_fields: &[String]) -> String;
    fn format_variable_declaration(&self, left: String, right: String) -> String;
    fn get_reference_symbol(&self) -> String;
    fn get_variadic_symbol(&self) -> String;
}
