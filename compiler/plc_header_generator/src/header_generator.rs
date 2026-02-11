use std::path::PathBuf;

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
        file_helper::{format_path, FileHelper},
        header_generator_c::GeneratedHeaderForC,
        symbol_helper::SymbolHelper,
        template_helper::{TemplateData, TemplateHelper, Variable, VariableType},
        type_helper::TypeHelper,
    },
    GenerateHeaderOptions, GenerateLanguage,
};

pub mod file_helper;
mod header_generator_c;
mod symbol_helper;
pub mod template_helper;
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
    // Prepare the template data
    let mut generated_header =
        prepare_template_data_for_header_generation(generate_header_options, compilation_unit)?;

    // Generate the headers
    generated_header.generate_headers()?;

    Ok(generated_header)
}

/// Prepares the header data for generation given the options for generation and a compilation unit.
///
/// ---
///
/// Should the process fail to determine an output directory and path for the header file
/// a none-generated header will be returned, this is a valid outcome as some compilation units
/// are not related to a file.
pub fn prepare_template_data_for_header_generation(
    generate_header_options: &GenerateHeaderOptions,
    compilation_unit: &CompilationUnit,
) -> Result<Box<dyn GeneratedHeader>, Diagnostic> {
    let mut generated_header = get_empty_generated_header_from_options(generate_header_options)?;

    // Determine file and directory
    // If the directories could not be configured with an acceptable outcome, then we exit without performing generation for this compilation unit
    if !generated_header.determine_header_file_information(generate_header_options, compilation_unit) {
        return Ok(generated_header);
    }

    // Prepare the template data
    generated_header.prepare_template_data(compilation_unit);

    Ok(generated_header)
}

/// Combines several generated headers and returns a single generated header containing the contents of all of them
pub fn combine_generated_headers(
    generate_header_options: &GenerateHeaderOptions,
    generated_headers: Vec<Box<dyn GeneratedHeader>>,
    output_file: String,
) -> Result<Box<dyn GeneratedHeader>, Diagnostic> {
    let mut generated_header = get_empty_generated_header_from_options(generate_header_options)?;

    // We assume the first generated header in the list's location is the location we would like to place the header
    let some_generated_header = generated_headers.iter().find(|p| !p.is_empty());
    if some_generated_header.is_none() {
        return Ok(generated_header);
    }

    let non_empty_generated_header = some_generated_header.unwrap();

    generated_header.set_directory(non_empty_generated_header.get_directory());
    let binding = PathBuf::from(non_empty_generated_header.get_directory()).join(&output_file);
    let path = binding.to_str().unwrap_or_default();
    generated_header.set_path(path);
    generated_header.set_formatted_path(&format_path(path));
    generated_header.set_file_name(&output_file);

    let mut template_data = TemplateData::new();
    for mut t_generated_header in generated_headers {
        if !t_generated_header.is_empty() {
            let user_defined_types = t_generated_header.get_mutable_template_data_user_defined_types();
            template_data.user_defined_types.aliases.append(&mut user_defined_types.aliases);
            template_data.user_defined_types.structs.append(&mut user_defined_types.structs);
            template_data.user_defined_types.enums.append(&mut user_defined_types.enums);
            template_data
                .global_variables
                .append(t_generated_header.get_mutable_template_data_global_variables());
            template_data.functions.append(t_generated_header.get_mutable_template_data_functions());
        }
    }

    // Overwrite the template data in the header with the combined template data
    generated_header.set_template_data(template_data);

    // Generate the headers
    generated_header.generate_headers()?;

    Ok(generated_header)
}

/// Generates an appropriate header for the given header options
pub fn get_empty_generated_header_from_options(
    generate_header_options: &GenerateHeaderOptions,
) -> Result<Box<dyn GeneratedHeader>, Diagnostic> {
    let generated_header: Result<Box<dyn GeneratedHeader>, Diagnostic> = match generate_header_options
        .language
    {
        GenerateLanguage::C => {
            let generated_header = GeneratedHeaderForC::new();
            Ok(Box::new(generated_header))
        }
        language => Err(Diagnostic::new(format!("This language '{:?}' is not yet implemented!", language))),
    };

    generated_header
}

/// A wrapper for a type name with extended information
pub struct ExtendedTypeName {
    pub type_name: String,
    pub is_variadic: bool,
    pub is_sized_variadic: bool,
}

impl Default for ExtendedTypeName {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtendedTypeName {
    pub const fn new() -> Self {
        ExtendedTypeName { type_name: String::new(), is_variadic: false, is_sized_variadic: false }
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
fn extract_enum_declaration_from_elements(enum_name: &str, node: &AstNode) -> Vec<Variable> {
    let mut enum_declarations: Vec<Variable> = Vec::new();

    match &node.stmt {
        AstStatement::ExpressionList(exp_nodes) => {
            let mut declared_value: i128 = 0;
            for exp_node in exp_nodes {
                match &exp_node.stmt {
                    AstStatement::Assignment(assignment) => {
                        let option_left = extract_enum_field_name_from_statement(&assignment.left.stmt);

                        if let Some(left) = option_left {
                            let option_right =
                                extract_enum_field_value_from_statement(&assignment.right.stmt);

                            let name = format!("{enum_name}_{left}");

                            if let Some(right) = option_right {
                                declared_value = right;
                            } else {
                                declared_value += 1;
                            };

                            enum_declarations.push(Variable {
                                data_type: enum_name.to_string(),
                                name,
                                variable_type: VariableType::Declaration(declared_value),
                            });
                        } else {
                            log::warn!(
                                "Unable to extract the enum field name from the given assignment {:?}!",
                                assignment
                            )
                        }
                    }
                    _ => log::warn!(
                        "Given node {:?} is not an Assignment, unable to extract the enum declaration!",
                        exp_node
                    ),
                }
            }
        }
        _ => log::warn!(
            "Given node {:?} is not an Expression list, unable to extract the enum declaration!",
            node
        ),
    }

    enum_declarations
}

/// Given an AstStatement, this will extract the name of the enum field.
///
/// Will return a new string if the AstStatement type not [ReferenceExpr](plc_ast::ast::AstStatement::ReferenceExpr),
/// the access of that expression is not type [Member](plc_ast::ast::ReferenceAccess::Member)
/// and the statement of that member is not type [Identifier](plc_ast::ast::AstStatement::Identifier).
fn extract_enum_field_name_from_statement(statement: &AstStatement) -> Option<String> {
    match statement {
        AstStatement::ReferenceExpr(reference_expression) => match &reference_expression.access {
            ReferenceAccess::Member(member_node) => {
                let member_statement = member_node.get_stmt();
                match member_statement {
                    AstStatement::Identifier(enum_field) => Some(enum_field.to_string()),
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// Extracts the value from an AstStatement type [Literal](plc_ast::ast::AstStatement::Literal).
///
/// Will return a new string if the AstStatement type is not [Literal](plc_ast::ast::AstStatement::Literal).
fn extract_enum_field_value_from_statement(statement: &AstStatement) -> Option<i128> {
    match statement {
        AstStatement::Literal(AstLiteral::Integer(value)) => Some(*value),
        _ => None,
    }
}

/// Creates an ExtendedTypeName from a given Option<DataTypeDeclaration>.
///
/// Will return the default for ExtendedTypeName if the data type declaration is not [Reference](plc_ast::ast::DataTypeDeclaration::Reference) or [Definition](plc_ast::ast::DataTypeDeclaration::Definition).
fn get_type_from_data_type_decleration(
    data_type_declaration: &Option<DataTypeDeclaration>,
    enforce_no_variadic: bool,
) -> ExtendedTypeName {
    match data_type_declaration {
        Some(DataTypeDeclaration::Reference { referenced_type, .. }) => ExtendedTypeName {
            type_name: referenced_type.clone(),
            is_variadic: false,
            is_sized_variadic: false,
        },
        Some(DataTypeDeclaration::Definition { data_type, .. }) => {
            let type_name: String = data_type.get_name().unwrap_or("").to_string();
            let (is_variadic, is_sized_variadic) = if enforce_no_variadic {
                (false, false)
            } else {
                match &**data_type {
                    ast::DataType::VarArgs { sized, .. } => {
                        if *sized {
                            (false, true)
                        } else {
                            (true, false)
                        }
                    }
                    _ => (false, false),
                }
            };

            ExtendedTypeName { type_name, is_variadic, is_sized_variadic }
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
        // The string-termination-marker needs to be accounted for
        AstStatement::Literal(AstLiteral::Integer(value)) => value + 1,
        _ => i128::default(),
    }
}

/// Determines what type of array this is, standard or multidimensional
fn determine_array_type(bounds: &AstNode) -> Option<VariableType> {
    match &bounds.stmt {
        AstStatement::RangeStatement(..) => {
            let size = extract_array_size(bounds);
            Some(VariableType::Array(size))
        }
        AstStatement::VlaRangeStatement => Some(VariableType::Array(i128::default())),
        AstStatement::ExpressionList(nodes) => {
            let mut sizes: Vec<i128> = Vec::new();

            for node in nodes {
                sizes.push(extract_array_size(node));
            }

            Some(VariableType::MultidimensionalArray(sizes))
        }
        _ => None,
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
