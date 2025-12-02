use serde::{Deserialize, Serialize};

mod template_helper_c;

pub trait TemplateHelper {
    /// Returns the template data of the header file, which is an object representation of the data that will be written to the header template
    fn get_template_data(&self) -> &TemplateData;

    /// Returns the template for the defined language based on the given template type
    fn get_template(&self, template_type: TemplateType) -> Template;
}

pub struct Template {
    pub content: String,
    pub name: String,
}

pub enum TemplateType {
    Header,
}

/// The template data used by the templating engine to build a header
#[derive(Serialize, Deserialize)]
pub struct TemplateData {
    pub user_defined_types: UserDefinedTypes,
    pub global_variables: Vec<Variable>,
    pub functions: Vec<Function>,
}

impl Default for TemplateData {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateData {
    pub const fn new() -> Self {
        TemplateData {
            user_defined_types: UserDefinedTypes::new(),
            global_variables: Vec::new(),
            functions: Vec::new(),
        }
    }
}

/// A representation of the possible user types used by the template data
#[derive(Serialize, Deserialize)]
pub struct UserDefinedTypes {
    pub aliases: Vec<Variable>,
    pub structs: Vec<UserType>,
    pub enums: Vec<UserType>,
}

impl Default for UserDefinedTypes {
    fn default() -> Self {
        Self::new()
    }
}

impl UserDefinedTypes {
    pub const fn new() -> Self {
        UserDefinedTypes { aliases: Vec::new(), structs: Vec::new(), enums: Vec::new() }
    }
}

/// A representation of a user type used by the template data
#[derive(Serialize, Deserialize)]
pub struct UserType {
    pub name: String,
    pub variables: Vec<Variable>,
}

/// A representation of a variable used by the template data
#[derive(Serialize, Deserialize)]
pub struct Variable {
    pub data_type: String,
    pub name: String,
    pub variable_type: VariableType,
}

/// The type of variable used by the template data
#[derive(Serialize, Deserialize, Clone)]
pub enum VariableType {
    /// A non-special variable
    Default,
    /// A variable that represents an array with the size of the array wrapped within
    Array(i128),
    /// A variable that declares a value with the value as a string wrapped within
    Declaration(String),
    /// A variable that is variadic
    Variadic,
    /// A variable that represents a struct
    Struct,
}

/// A representation of a function used by the template data
#[derive(Serialize, Deserialize)]
pub struct Function {
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Variable>,
}
