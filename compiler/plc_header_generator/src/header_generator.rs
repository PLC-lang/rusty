use plc_ast::{
    ast::{
        self, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration, ReferenceAccess,
        UserTypeDeclaration,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;
use serde::{Deserialize, Serialize};

use crate::{
    header_generator::{
        file_helper::FileHelper, header_generator_c::GeneratedHeaderForC, symbol_helper::SymbolHelper,
        template_helper::TemplateHelper, type_helper::TypeHelper,
    },
    GenerateHeaderOptions, GenerateLanguage,
};

mod file_helper;
mod header_generator_c;
mod symbol_helper;
mod template_helper;
mod type_helper;

pub trait GeneratedHeader: FileHelper + TypeHelper + TemplateHelper + SymbolHelper {
    fn is_empty(&self) -> bool;
    fn get_directory(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_contents(&self) -> &str;
    fn get_template_data(&self) -> &TemplateData;
    fn prepare_template_data(&mut self, compilation_unit: &CompilationUnit);
    fn generate_headers(&mut self) -> Result<(), Diagnostic>;
}

pub fn get_generated_header(
    generate_header_options: &GenerateHeaderOptions,
    compilation_unit: &CompilationUnit,
) -> Result<Box<dyn GeneratedHeader>, Diagnostic> {
    let mut generated_header: Box<dyn GeneratedHeader> = match generate_header_options.language {
        GenerateLanguage::C => {
            let generated_header = GeneratedHeaderForC::new();
            Box::new(generated_header)
        }
        language => panic!("This language '{:?}' is not yet implemented!", language),
    };

    // Determine file and directory
    // If the directories could not be configured with an acceptable outcome, then we exit without performing generation for this compilation unit
    if !generated_header.determine_header_file_information(generate_header_options, compilation_unit) {
        return Ok(generated_header);
    }

    // Prepare the template data
    generated_header.prepare_template_data(compilation_unit);

    // Generate the headers
    generated_header.generate_headers()?;

    Ok(generated_header)
}

#[derive(Serialize, Deserialize)]
pub struct TemplateData {
    pub user_defined_types: UserDefinedTypes,
    pub global_variables: Vec<Variable>,
    pub functions: Vec<Function>
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
            functions: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserDefinedTypes {
    pub aliases: Vec<Variable>,
    pub structs: Vec<UserType>,
    pub enums: Vec<UserType>
}

impl Default for UserDefinedTypes {
    fn default() -> Self {
        Self::new()
    }
}

impl UserDefinedTypes {
    pub const fn new() -> Self {
        UserDefinedTypes {
            aliases: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserType {
    pub name: String,
    pub variables: Vec<Variable>
}

#[derive(Serialize, Deserialize)]
pub struct Variable {
    pub data_type: String,
    pub name: String,
    pub variable_type: VariableType
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VariableType {
    Default,
    Array(i128),
    Declaration(String),
    Alias(String),
    Variadic,
    Struct
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Variable>
}

pub struct ExtendedTypeName {
    pub type_name: String,
    pub is_variadic: bool,
}

impl Default for ExtendedTypeName {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtendedTypeName {
    pub const fn new() -> Self {
        ExtendedTypeName { type_name: String::new(), is_variadic: false }
    }
}

fn coalesce_optional_strings_with_default(
    name: &Option<String>,
    field_name_override: Option<&String>,
) -> String {
    if let Some(field_name_ovr) = field_name_override {
        field_name_ovr.to_string()
    } else {
        name.clone().unwrap_or_default()
    }
}

fn extract_enum_declaration_from_elements(node: &AstNode) -> Vec<Variable> {
    let mut enum_declarations: Vec<Variable> = Vec::new();

    match &node.stmt {
        AstStatement::ExpressionList(exp_nodes) => {
            for exp_node in exp_nodes {
                match &exp_node.stmt {
                    AstStatement::Assignment(assignment) => {
                        let left = extract_enum_field_name_from_statement(&assignment.left.stmt);
                        let right = extract_enum_field_value_from_statement(&assignment.right.stmt);

                        if right.is_empty() {
                            enum_declarations.push(Variable {
                                data_type: String::new(),
                                name: left,
                                variable_type: VariableType::Default
                            });
                        } else {
                            enum_declarations.push(Variable {
                                data_type: String::new(),
                                name: left,
                                variable_type: VariableType::Declaration(right)
                            });
                        }
                    }
                    _ => continue,
                }
            }
        }
        _ => todo!(),
    }

    enum_declarations
}

fn extract_enum_field_name_from_statement(statement: &AstStatement) -> String {
    match statement {
        AstStatement::ReferenceExpr(reference_expression) => match &reference_expression.access {
            ReferenceAccess::Member(member_node) => {
                let member_statement = member_node.get_stmt();
                match member_statement {
                    AstStatement::Identifier(enum_field) => enum_field.to_string(),
                    _ => String::new(),
                }
            }
            _ => String::new(),
        },
        _ => String::new(),
    }
}

fn extract_enum_field_value_from_statement(statement: &AstStatement) -> String {
    match statement {
        AstStatement::Literal(literal) => literal.get_literal_value(),
        _ => String::new(),
    }
}

fn get_type_from_data_type_decleration(
    data_type_declaration: &Option<DataTypeDeclaration>,
) -> ExtendedTypeName {
    match data_type_declaration {
        Some(DataTypeDeclaration::Reference { referenced_type, .. }) => {
            ExtendedTypeName { type_name: referenced_type.clone(), is_variadic: false }
        }
        Some(DataTypeDeclaration::Definition { data_type, .. }) => {
            let type_name: String = data_type.get_name().unwrap_or("").to_string();
            let is_variadic = matches!(&**data_type, ast::DataType::VarArgs { .. });

            ExtendedTypeName { type_name, is_variadic }
        }
        _ => ExtendedTypeName::new(),
    }
}

fn get_user_generated_type_by_name<'a>(
    name: &'a str,
    user_types: &'a [UserTypeDeclaration],
) -> Option<&'a UserTypeDeclaration> {
    for user_type in user_types {
        if let Some(data_type_name) = user_type.data_type.get_name() {
            if data_type_name == name {
                return Some(user_type);
            }
        }
    }

    None
}

fn extract_string_size(size: &Option<AstNode>) -> i128 {
    if size.is_none() {
        return i128::default();
    }

    let size = size.clone().unwrap();

    match size.stmt {
        // TODO: Verify this is necessary
        // +1 character for the string-termination-marker
        AstStatement::Literal(AstLiteral::Integer(value)) => value + 1,
        _ => i128::default(),
    }
}

fn extract_array_size(bounds: &AstNode) -> i128 {
    match &bounds.stmt {
        AstStatement::RangeStatement(range_stmt) => {
            let start_value = match range_stmt.start.get_stmt() {
                AstStatement::Literal(AstLiteral::Integer(value)) => *value,
                _ => i128::default(),
            };

            let end_value = match range_stmt.end.get_stmt() {
                AstStatement::Literal(AstLiteral::Integer(value)) => *value,
                _ => i128::default(),
            };

            end_value - start_value + 1
        }
        _ => i128::default(),
    }
}
