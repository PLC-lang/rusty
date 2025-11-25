use plc_ast::{
    ast::{
        self, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration, ReferenceAccess,
        UserTypeDeclaration,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;

use crate::{
    header_generator::{
        file_helper::FileHelper,
        header_generator_c::GeneratedHeaderForC,
        symbol_helper::SymbolHelper,
        template_helper::{TemplateHelper, Variable, VariableType},
        type_helper::TypeHelper,
    },
    GenerateHeaderOptions, GenerateLanguage,
};

pub mod file_helper;
mod header_generator_c;
mod symbol_helper;
mod template_helper;
mod type_helper;

/// A combined trait containing all of the necessary implementations for generating a header
pub trait GeneratedHeader: FileHelper + TypeHelper + TemplateHelper + SymbolHelper {
    /// Returns whether or not this generated header is empty
    ///
    /// ---
    ///
    /// This must return true if the generation process has not yet occured,
    /// or was aborted in a valid case.
    fn is_empty(&self) -> bool;

    /// Returns the contents of the header file
    fn get_contents(&self) -> &str;

    /// Prepares the template data for this header given a compilation unit
    ///
    /// ---
    ///
    /// The outcome of this method must be a populated [TemplateData](crate::header_generator::template_helper::TemplateData) on the generated header
    /// that contains all of the data necessary to run the templating engine.
    fn prepare_template_data(&mut self, compilation_unit: &CompilationUnit);

    /// Runs the templating engine and generates the header file contents
    ///
    /// ---
    ///
    /// The outcome of this method must be a populated "contents" (accessible via the "get_contents" method)
    /// on the generated header.
    fn generate_headers(&mut self) -> Result<(), Diagnostic>;
}

/// Returns a generated header given the options for generation and a compilation unit.
///
/// ---
///
/// Should the process fail to determine an output directory and path for the header file
/// a none-generated header will be returned, this is a valid outcome as some compilation units
/// are not related to a file.
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

/// A wrapper for a type name with extended information
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

/// Given a name and a field name override, this will return the field name if present, the name if not, or default if the name is empty.
fn coalesce_field_name_override_with_default(
    name: &Option<String>,
    field_name_override: Option<&String>,
) -> String {
    if let Some(field_name_ovr) = field_name_override {
        field_name_ovr.to_string()
    } else {
        name.clone().unwrap_or_default()
    }
}

/// Given an ast node this will extract the enum declarations an return a Vec<Variable>.
///
/// Will return an empty Vec if the statement of the node is not type [ExpressionList](plc_ast::ast::AstStatement::ExpressionList)
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
                                variable_type: VariableType::Default,
                            });
                        } else {
                            enum_declarations.push(Variable {
                                data_type: String::new(),
                                name: left,
                                variable_type: VariableType::Declaration(right),
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

/// Given an AstStatement, this will extract the name of the enum field.
///
/// Will return a new string if the AstStatement type not [ReferenceExpr](plc_ast::ast::AstStatement::ReferenceExpr),
/// the access of that expression is not type [Member](plc_ast::ast::ReferenceAccess::Member)
/// and the statement of that member is not type [Identifier](plc_ast::ast::AstStatement::Identifier).
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

/// Extracts the value from an AstStatement type [Literal](plc_ast::ast::AstStatement::Literal).
///
/// Will return a new string if the AstStatement type is not [Literal](plc_ast::ast::AstStatement::Literal).
fn extract_enum_field_value_from_statement(statement: &AstStatement) -> String {
    match statement {
        AstStatement::Literal(literal) => literal.get_literal_value(),
        _ => String::new(),
    }
}

/// Creates an ExtendedTypeName from a given Option<DataTypeDeclaration>.
///
/// Will return the default for ExtendedTypeName if the data type declaration is not [Reference](plc_ast::ast::DataTypeDeclaration::Reference) or [Definition](plc_ast::ast::DataTypeDeclaration::Definition).
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

/// Given a data type name and a list of user type declarations,
/// this will extract the user type declaration with a data type name that matches the given data type name.
///
/// This will return None in the case that the given data type name does not match the data type name of the user type declaration.
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

/// Given an Option<AstNode> containing a [Literal](plc_ast::ast::AstStatement::Literal), this will determine the [i128] size of a string.
///
/// This will return [i128] default in the case the AstNode is None or does not match the expected [Literal](plc_ast::ast::AstStatement::Literal).
fn extract_string_size(size: &Option<AstNode>) -> i128 {
    if size.is_none() {
        return i128::default();
    }

    let size = size.clone().unwrap();

    match size.stmt {
        // TODO: Verify if the string-termination-marker needs to be accounted for
        AstStatement::Literal(AstLiteral::Integer(value)) => value,
        _ => i128::default(),
    }
}

/// Given an AstNode containing a [RangeStatement](plc_ast::ast::AstStatement::RangeStatement), this will determine the [i128] size of an array.
///
/// This will return [i128] default in the case the AstNode does not match the expected [RangeStatement](plc_ast::ast::AstStatement::RangeStatement).
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

/// Common method for determining if the string representation of the data type name is system generated.
///
/// i.e. Starts with "__"
fn data_type_is_system_generated(data_type: &str) -> bool {
    if data_type.starts_with("__") {
        return true;
    }

    false
}

/// Prepares a method name for a header file
///
/// i.e. Any "." character will be replaced with "__"
fn sanitize_method_name(method_name: &str) -> String {
    method_name.replace(".", "__").to_string()
}
