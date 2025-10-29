use plc::typesystem::{get_builtin_types, DataType};
use plc_ast::{
    ast::{
        self, ArgumentProperty, AstNode, AstStatement, CompilationUnit, DataTypeDeclaration, PouType,
        ReferenceAccess, UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
    },
    literals::AstLiteral,
};
use plc_diagnostics::diagnostics::Diagnostic;
use tera::{Context, Tera};

use crate::{
    file_manager::FileManager,
    template_manager::{TemplateManager, TemplateType},
    type_manager::TypeManager,
    GenerateHeaderOptions,
};

pub struct GeneratedHeader {
    pub directory: String,
    pub path: String,
    pub contents: String,

    declared_user_types: Vec<String>,
    template_manager: TemplateManager,
    type_manager: TypeManager,
    file_manager: FileManager,
}

pub enum GenerationSource {
    GlobalVariable,
    UserType,
    Struct,
    FunctionParameter,
}

impl Default for GeneratedHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedHeader {
    pub const fn new() -> Self {
        GeneratedHeader {
            declared_user_types: Vec::new(),
            directory: String::new(),
            path: String::new(),
            contents: String::new(),
            template_manager: TemplateManager::new(),
            type_manager: TypeManager::new(),
            file_manager: FileManager::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.directory.is_empty() && self.path.is_empty() && self.contents.is_empty()
    }

    pub fn generate_headers(
        &mut self,
        generate_header_options: &GenerateHeaderOptions,
        compilation_unit: CompilationUnit,
    ) -> Result<(), Diagnostic> {
        // Setup managers
        self.template_manager.language = generate_header_options.language;
        self.type_manager.language = generate_header_options.language;
        self.file_manager.language = generate_header_options.language;

        // If the directories could not be configured with an acceptable outcome, then we exit without performing generation for this compilation unit
        if !self.file_manager.prepare_file_and_directory(generate_header_options, &compilation_unit)? {
            return Ok(());
        }

        // Configure tera
        let mut tera = Tera::default();
        let mut context = Context::new();

        let template = self
            .template_manager
            .get_template(TemplateType::Header)
            .expect("Unable to find the 'header' template!");
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'header' template to tera!");

        let builtin_types = get_builtin_types();

        // 1 - Add global variables
        let global_variables = self.build_global_variables(&compilation_unit, &builtin_types);
        context.insert("global_variables", &global_variables);

        // 2 - Add functions
        let functions = self.build_functions(&compilation_unit, &builtin_types);
        context.insert("functions", &functions);

        // TODO: 3 - Add function blocks
        let test = "// TODO";
        let function_blocks = format!("{test}: I'm not a function block... that's weird??");
        context.insert("function_blocks", &function_blocks);

        // 4 - Add user-defined data types
        let user_defined_data_types = self.build_user_types(&compilation_unit, &builtin_types);
        context.insert("user_defined_data_types", &user_defined_data_types);

        // Set the outputs
        self.directory = self.file_manager.output_dir.clone();
        self.path = self.file_manager.output_path.clone();
        self.contents = tera.render(&template.name, &context).unwrap();

        Ok(())
    }

    fn build_global_variables(
        &mut self,
        compilation_unit: &CompilationUnit,
        builtin_types: &[DataType],
    ) -> String {
        let global_variables = self.get_variables_from_variable_blocks(
            &compilation_unit.global_vars,
            builtin_types,
            &[VariableBlockType::Global],
            &compilation_unit.user_types,
            &GenerationSource::GlobalVariable,
        );

        format!("extern {};", global_variables.join(";\nextern "))
    }

    fn build_functions(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) -> String {
        let mut functions: Vec<String> = Vec::new();

        for pou in &compilation_unit.pous {
            match pou.kind {
                PouType::Function => {
                    let mut tera = Tera::default();
                    let mut context = Context::new();

                    let template = self
                        .template_manager
                        .get_template(TemplateType::Function)
                        .expect("Unable to find the 'function' template!");
                    tera.add_raw_template(&template.name, &template.content)
                        .expect("Unable to add the 'function' template to tera!");

                    let (_, type_name) = &self.type_manager.get_type_name_for_type(
                        &get_type_from_data_type_decleration(&pou.return_type),
                        builtin_types,
                    );

                    context.insert("data_type", type_name);
                    context.insert("name", &pou.name);

                    let input_variables = self
                        .get_variables_from_variable_blocks(
                            &pou.variable_blocks,
                            builtin_types,
                            &[
                                VariableBlockType::Input(ArgumentProperty::ByRef),
                                VariableBlockType::Input(ArgumentProperty::ByVal),
                                VariableBlockType::InOut,
                                VariableBlockType::Output,
                            ],
                            &compilation_unit.user_types,
                            &GenerationSource::FunctionParameter,
                        )
                        .join(", ");

                    context.insert("variables", &input_variables);

                    let content = tera.render(&template.name, &context).unwrap().trim().to_string();
                    functions.push(content);
                }
                _ => continue,
            }
        }

        format!("{};", functions.join(";\n\n"))
    }

    fn build_user_types(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) -> String {
        let mut user_types: Vec<String> = Vec::new();

        for user_type in &compilation_unit.user_types {
            let formatted_user_type = self.build_user_type(
                user_type,
                builtin_types,
                &compilation_unit.user_types,
                None,
                None,
                &GenerationSource::UserType,
            );

            if let Some(value) = formatted_user_type {
                user_types.push(value);
            }
        }

        format!("{};", user_types.join(";\n\n"))
    }

    fn build_user_type(
        &mut self,
        user_type: &UserTypeDeclaration,
        builtin_types: &[DataType],
        user_types: &[UserTypeDeclaration],
        field_name_override: Option<&String>,
        type_name_override: Option<&String>,
        generation_source: &GenerationSource,
    ) -> Option<String> {
        let mut tera = Tera::default();
        let mut context = Context::new();

        let type_name = String::from(user_type.data_type.get_name().unwrap_or(""));
        if self.declared_user_types.contains(&type_name)
            && matches!(generation_source, GenerationSource::UserType)
        {
            return None;
        }

        match &user_type.data_type {
            ast::DataType::StructType { name, variables } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable | GenerationSource::UserType => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::UserTypeStruct)
                            .expect("Unable to find the 'user type struct' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'user type struct' template to tera!");

                        let formatted_variables = self.get_formatted_variables_from_variables(
                            variables,
                            builtin_types,
                            false,
                            user_types,
                            &GenerationSource::Struct,
                        );
                        context.insert(
                            "name",
                            &coalesce_optional_strings_with_default(name, field_name_override),
                        );
                        context.insert("variables", &format!("{};", formatted_variables.join(";\n\t")));

                        template
                    }
                    GenerationSource::Struct | GenerationSource::FunctionParameter => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::ParamStruct)
                            .expect("Unable to find the 'param struct' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'param struct' template to tera!");

                        context.insert(
                            "name",
                            &field_name_override
                                .expect("Field name expected for generation source type: 'Parameter'!"),
                        );
                        context.insert(
                            "data_type",
                            &type_name_override
                                .expect("Data Type expected for generation source type: 'Parameter'!"),
                        );

                        template
                    }
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::EnumType { name, elements, .. } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable | GenerationSource::UserType => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::UserTypeEnum)
                            .expect("Unable to find the 'user type enum' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'user type enum' template to tera!");

                        let formatted_enum_declerations =
                            self.extract_enum_declaration_from_elements(elements);
                        context.insert(
                            "name",
                            &coalesce_optional_strings_with_default(name, field_name_override),
                        );
                        context.insert("variables", &formatted_enum_declerations);

                        template
                    }
                    GenerationSource::Struct | GenerationSource::FunctionParameter => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::ParamEnum)
                            .expect("Unable to find the 'param enum' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'param enum' template to tera!");

                        context.insert(
                            "name",
                            &field_name_override
                                .expect("Field name expected for generation source type: 'Parameter'!"),
                        );
                        context.insert(
                            "data_type",
                            &type_name_override
                                .expect("Data Type expected for generation source type: 'Parameter'!"),
                        );

                        template
                    }
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::StringType { name, size, is_wide } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable
                    | GenerationSource::Struct
                    | GenerationSource::UserType => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::UserTypeArray)
                            .expect("Unable to find the 'user type array' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'user type array' template to tera!");

                        let string_size = extract_string_size(size);
                        context.insert(
                            "name",
                            &coalesce_optional_strings_with_default(name, field_name_override),
                        );
                        context.insert("data_type", &self.type_manager.get_type_name_for_string(is_wide));
                        context.insert("size", &string_size);

                        template
                    }
                    GenerationSource::FunctionParameter => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::ParamArray)
                            .expect("Unable to find the 'param array' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'param array' template to tera!");

                        context.insert(
                            "name",
                            &field_name_override
                                .expect("Field name expected for generation source type: 'Parameter'!"),
                        );
                        context.insert("data_type", &self.type_manager.get_type_name_for_string(is_wide));

                        template
                    }
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::ArrayType { name, bounds, referenced_type, .. } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable
                    | GenerationSource::Struct
                    | GenerationSource::UserType => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::UserTypeArray)
                            .expect("Unable to find the 'user type array' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'user type array' template to tera!");

                        let string_size = extract_array_size(bounds);
                        context.insert(
                            "name",
                            &coalesce_optional_strings_with_default(name, field_name_override),
                        );

                        let (_, type_name) = self
                            .type_manager
                            .get_type_name_for_type(referenced_type.get_name().unwrap(), builtin_types);
                        context.insert("data_type", &type_name);
                        context.insert("size", &string_size);

                        template
                    }
                    GenerationSource::FunctionParameter => {
                        let template = self
                            .template_manager
                            .get_template(TemplateType::ParamArray)
                            .expect("Unable to find the 'param array' template!");
                        tera.add_raw_template(&template.name, &template.content)
                            .expect("Unable to add the 'param array' template to tera!");

                        context.insert(
                            "name",
                            &field_name_override
                                .expect("Field name expected for generation source type: 'Parameter'!"),
                        );

                        let (_, type_name) = self
                            .type_manager
                            .get_type_name_for_type(referenced_type.get_name().unwrap(), builtin_types);
                        context.insert("data_type", &type_name);

                        template
                    }
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::PointerType { .. } => {
                // TODO: Implement Pointers
                None
            }
            ast::DataType::SubRangeType { .. } => {
                // TODO: Implement Sub Range -- This is just an integer
                None
            }
            ast::DataType::VarArgs { .. } => {
                // TODO: Implement Var Args -- This is limited to functions https://www.geeksforgeeks.org/c/printf-in-c/
                None
            }
            ast::DataType::GenericType { .. } => {
                // Currently out of scope
                None
            }
        }
    }

    fn extract_enum_declaration_from_elements(&self, node: &AstNode) -> String {
        match &node.stmt {
            AstStatement::ExpressionList(exp_nodes) => {
                let mut formatted_enum_declarations: Vec<String> = Vec::new();
                for exp_node in exp_nodes {
                    match &exp_node.stmt {
                        AstStatement::Assignment(assignment) => {
                            let left = extract_enum_field_name_from_statement(&assignment.left.stmt);
                            let right = extract_enum_field_value_from_statement(&assignment.right.stmt);
                            let right = if right.is_empty() { String::new() } else { format!(" = {right}") };

                            formatted_enum_declarations.push(format!("{left}{right}"));
                        }
                        _ => continue,
                    }
                }
                formatted_enum_declarations.join(",\n\t")
            }
            _ => String::new(),
        }
    }

    fn get_variables_from_variable_blocks(
        &mut self,
        variable_blocks: &[VariableBlock],
        builtin_types: &[DataType],
        variable_block_types: &[VariableBlockType],
        user_types: &[UserTypeDeclaration],
        generation_source: &GenerationSource,
    ) -> Vec<String> {
        let mut variables: Vec<String> = Vec::new();

        for variable_block in variable_blocks {
            if variable_block_types.contains(&variable_block.kind) {
                let is_reference = variable_block.kind == VariableBlockType::Input(ArgumentProperty::ByRef)
                    || variable_block.kind == VariableBlockType::InOut
                    || variable_block.kind == VariableBlockType::Output;

                variables.append(&mut self.get_formatted_variables_from_variables(
                    &variable_block.variables,
                    builtin_types,
                    is_reference,
                    user_types,
                    generation_source,
                ));
            }
        }

        variables
    }

    fn get_formatted_variables_from_variables(
        &mut self,
        variable_block_variables: &[Variable],
        builtin_types: &[DataType],
        is_reference: bool,
        user_types: &[UserTypeDeclaration],
        generation_source: &GenerationSource,
    ) -> Vec<String> {
        let mut tera = Tera::default();
        let mut context = Context::new();
        let mut variables: Vec<String> = Vec::new();

        let reference_symbol = if is_reference { "*" } else { "" };

        for variable in variable_block_variables {
            let (is_user_generated, type_name) = self.type_manager.get_type_name_for_type(
                &get_type_from_data_type_decleration(&Some(variable.data_type_declaration.clone())),
                builtin_types,
            );

            if is_user_generated {
                let option_user_type = get_user_generated_type_by_name(&type_name, user_types);

                if let Some(user_type) = option_user_type {
                    let formatted_user_type = self.build_user_type(
                        user_type,
                        builtin_types,
                        user_types,
                        Some(&String::from(variable.get_name())),
                        Some(&type_name),
                        generation_source,
                    );

                    if let Some(value) = formatted_user_type {
                        if !self.type_manager.user_type_can_be_declared_outside_of_a_function(user_type) {
                            self.declared_user_types.push(type_name.clone());
                        }
                        variables.push(value);
                    }
                }
            } else {
                let template = self
                    .template_manager
                    .get_template(TemplateType::Variable)
                    .expect("Unable to find the 'variable' template!");
                tera.add_raw_template(&template.name, &template.content)
                    .expect("Unable to add the 'variable' template to tera!");

                context.insert("data_type", &type_name);
                context.insert("name", variable.get_name());
                context.insert("reference_symbol", reference_symbol);

                variables.push(tera.render(&template.name, &context).unwrap().trim().to_string());
            }
        }

        variables
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

fn get_type_from_data_type_decleration(data_type_declaration: &Option<DataTypeDeclaration>) -> String {
    match data_type_declaration {
        Some(DataTypeDeclaration::Reference { referenced_type, .. }) => referenced_type.clone(),
        _ => String::new(),
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

fn extract_string_size(size: &Option<AstNode>) -> String {
    if size.is_none() {
        return String::new();
    }

    let size = size.clone().unwrap();

    match size.stmt {
        // TODO: Verify this is necessary
        // +1 character for the string-termination-marker
        AstStatement::Literal(AstLiteral::Integer(value)) => format!("{}", value + 1),
        _ => String::new(),
    }
}

fn extract_array_size(bounds: &AstNode) -> String {
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

            format!("{}", end_value - start_value + 1)
        }
        _ => String::new(),
    }
}
