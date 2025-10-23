use std::{fs, path::{PathBuf}};
use plc::typesystem::{get_builtin_types, DataType};
use plc_ast::ast::{self, ArgumentProperty, CompilationUnit, DataTypeDeclaration, PouType, UserTypeDeclaration, Variable, VariableBlock, VariableBlockType};
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::FileMarker;
use tera::{Context, Tera};

use crate::{type_map::get_type_name_for_type_by_language, GenerateHeaderOptions, GenerateLanguage};

pub fn generate_headers(generate_header_options: &GenerateHeaderOptions, compilation_unit: CompilationUnit) -> Result<(), Diagnostic> {
    let (header_path, header_dir, header_file_name) = match generate_header_options.language {
        GenerateLanguage::C => {
            let file_path = match compilation_unit.file {
                FileMarker::File(file_path) => PathBuf::from(file_path),
                _ => PathBuf::from(String::new())
            };

            let mut output_path = if generate_header_options.output_path.as_os_str().is_empty() {
                if file_path.parent().is_some() { PathBuf::from(file_path.parent().unwrap()) } else { PathBuf::from(String::new()) }
            }
            else {
                generate_header_options.output_path.clone()
            };

            if output_path.as_os_str().is_empty() {
                return Err(Diagnostic::new("Unable to determine an output path!"));
            }

            let output_dir = output_path.clone();
            let output_name = if generate_header_options.prefix.is_empty() {
                let file_name = get_file_name_from_path_buf_without_extension(file_path);
                if file_name.is_some() {
                    format!("{}.h", file_name.unwrap())
                }
                else {
                    String::new()
                }
            }
            else {
                format!("{}.h", generate_header_options.prefix)
            };

            if output_name.is_empty() {
                // TODO: I'm not sure how I feel about swallowing this...
                // ... this usually means this compilation unit is not associated with a file,
                // but I don't know if that will always be the case
                return Ok(());
            }

            output_path.push(&output_name);

            (output_path, output_dir, output_name)
        },
        language => {
            return Err(Diagnostic::new(format!("{language:?} language not yet supported!")));
        }
    };

    // Create the directories to the output path
    fs::create_dir_all(&header_dir)?;

    // Copy the template file to the output path
    let template_file = include_str!("templates/header_template.h");
    fs::write(&header_path, template_file)?;

    // Use tera to render the header
    let header_path = header_path.to_str().unwrap();
    let header_dir = format!("{}/**/*", header_dir.to_str().unwrap());

    let tera = Tera::new(&header_dir).unwrap();
    let mut context = Context::new();
    let builtin_types = get_builtin_types();

    // Add global variables
    let global_variables = build_global_variables(&compilation_unit, generate_header_options.language, &builtin_types);
    context.insert("global_variables", &global_variables);

    // Add user-defined data types
    let user_defined_data_types = build_user_types(&compilation_unit, generate_header_options.language, &builtin_types);
    context.insert("user_defined_data_types", &user_defined_data_types);

    // Add functions
    let functions = build_functions(&compilation_unit, generate_header_options.language, &builtin_types);
    context.insert("functions", &functions);

    // TODO: Add function blocks
    let test = "// TODO";
    let function_blocks = format!("{test}: I'm not a function block... that's weird??");
    context.insert("function_blocks", &function_blocks);

    // Write the file
    let content = tera.render(&header_file_name, &context).unwrap();
    fs::write(header_path, content)?;

    Ok(())
}

fn build_global_variables(compilation_unit: &CompilationUnit, language: GenerateLanguage, builtin_types: &[DataType]) -> String {
    let global_variables = get_variables_from_variable_blocks(
            language,
            &compilation_unit.global_vars,
            builtin_types,
            &[VariableBlockType::Global])
        .join(";\nextern ");

    format!("extern {global_variables};")
}

fn build_functions(compilation_unit: &CompilationUnit, language: GenerateLanguage, builtin_types: &[DataType]) -> String {
    let mut functions: Vec<String> = Vec::new();

    for pou in &compilation_unit.pous {
        match pou.kind {
            PouType::Function => {
                let mut tera = Tera::default();
                let function_template_content = include_str!("templates/function_template.h");
                // TODO: This sucks - do something about it
                let _ = tera.add_raw_template("function_template.h", function_template_content);
                let mut context = Context::new();

                context.insert("data_type", &get_type_name_for_type_by_language(
                    language,
                    &get_type_from_data_type_decleration(&pou.return_type),
                    builtin_types));
                context.insert("name", &pou.name);

                let input_variables = get_variables_from_variable_blocks(
                    language,
                    &pou.variable_blocks,
                    builtin_types,
                    &[
                        VariableBlockType::Input(ArgumentProperty::ByRef),
                        VariableBlockType::Input(ArgumentProperty::ByVal),
                        VariableBlockType::InOut,
                        VariableBlockType::Output
                    ])
                .join(", ");

                context.insert("variables", &input_variables);

                let content = tera.render("function_template.h", &context).unwrap();
                functions.push(content);
            }
            _ => continue
        }
    }

    functions.join("\n")
}

fn build_user_types(compilation_unit: &CompilationUnit, language: GenerateLanguage, builtin_types: &[DataType]) -> String {
    let mut user_types: Vec<String> = Vec::new();

    for user_type in &compilation_unit.user_types {
        user_types.push(get_type_from_user_type_decleration(language, user_type, builtin_types));
    }

    user_types.join("\n")
}

fn get_type_from_user_type_decleration(language: GenerateLanguage, user_type: &UserTypeDeclaration, builtin_types: &[DataType]) -> String {
    match &user_type.data_type {
        ast::DataType::StructType { name, variables} => {
            let mut tera = Tera::default();
            let struct_template_content = include_str!("templates/struct_template.h");
            // TODO: This sucks - do something about it
            let _ = tera.add_raw_template("struct_template.h", struct_template_content);
            let mut context = Context::new();

            let formatted_variables = get_formatted_variables_from_variables(language, variables, builtin_types, false);
            context.insert("name", &name.clone().unwrap_or(String::new()),);
            context.insert("variables", &format!("{};", formatted_variables.join(";\n\t")));

            tera.render("struct_template.h", &context).unwrap()
        },
        _ => String::new()
    }
}

fn get_type_from_data_type_decleration(data_type_declaration: &Option<DataTypeDeclaration>) -> String {
    match data_type_declaration {
        Some(DataTypeDeclaration::Reference { referenced_type, .. }) => {
            referenced_type.clone()
        }
        _ => String::new()
    }
}

fn get_variables_from_variable_blocks(language: GenerateLanguage, variable_blocks: &[VariableBlock],
    builtin_types: &[DataType], variable_block_types: &[VariableBlockType]) -> Vec<String> {
    let mut variables: Vec<String> = Vec::new();

    for variable_block in variable_blocks {
        if variable_block_types.contains(&variable_block.kind) {
            let is_reference = variable_block.kind == VariableBlockType::Input(ArgumentProperty::ByRef)
                || variable_block.kind == VariableBlockType::InOut || variable_block.kind == VariableBlockType::Output;

            variables.append(&mut get_formatted_variables_from_variables(language, &variable_block.variables, builtin_types, is_reference));
        }
    }

    variables
}

fn get_formatted_variables_from_variables(language: GenerateLanguage, variable_block_variables: &[Variable], builtin_types: &[DataType], is_reference: bool) -> Vec<String> {
    let mut variables: Vec<String> = Vec::new();

    let reference_sign = if is_reference { "*" } else { "" };

    for variable in variable_block_variables {
        let variable_str = format!("{}{reference_sign} {}",
            get_type_name_for_type_by_language(language, &get_type_from_data_type_decleration(&Some(variable.data_type_declaration.clone())), builtin_types),
            variable.get_name());
        variables.push(variable_str.to_string());
    }

    variables
}

fn get_file_name_from_path_buf_without_extension(file_path: PathBuf) -> Option<String> {
    if file_path.file_name().is_some()
    {
        let file_name = file_path.file_name().unwrap().to_str();
        file_name?;

        let file_name = file_name.unwrap().split('.').next().unwrap_or("");

        if file_name.is_empty() {
            return None;
        }

        Some(String::from(file_name))
    }
    else {
        None
    }
}
