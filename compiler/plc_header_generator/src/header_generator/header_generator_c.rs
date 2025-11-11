use plc::typesystem::{get_builtin_types, DataType, LWORD_TYPE};
use plc_ast::ast::{
    self, ArgumentProperty, AstNode, AstStatement, CompilationUnit, Pou, PouType, UserTypeDeclaration,
    Variable, VariableBlock, VariableBlockType,
};
use plc_diagnostics::diagnostics::Diagnostic;
use tera::{Context, Tera};

use crate::header_generator::{
    coalesce_optional_strings_with_default, extract_array_size, extract_enum_field_name_from_statement,
    extract_enum_field_value_from_statement, extract_string_size,
    file_helper::HeaderFileInformation,
    get_type_from_data_type_decleration, get_user_generated_type_by_name,
    symbol_helper::SymbolHelper,
    template_helper::{Template, TemplateHelper, TemplateType},
    type_helper::{TypeAttribute, TypeHelper},
    ExtendedTypeName, GeneratedHeader, GenerationSource,
};

pub struct GeneratedHeaderForC {
    pub file_information: HeaderFileInformation,
    contents: String,
    declared_user_types: Vec<String>,
}

impl Default for GeneratedHeaderForC {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedHeader for GeneratedHeaderForC {
    fn is_empty(&self) -> bool {
        self.file_information.directory.is_empty()
            && self.file_information.path.is_empty()
            && self.contents.is_empty()
    }

    fn get_contents(&self) -> &str {
        &self.contents
    }

    fn get_directory(&self) -> &str {
        &self.file_information.directory
    }

    fn get_path(&self) -> &str {
        &self.file_information.path
    }

    fn generate_headers(&mut self, compilation_unit: &CompilationUnit) -> Result<(), Diagnostic> {
        // Configure tera
        let mut tera = Tera::default();
        let mut context = Context::new();

        let template = self.get_template(TemplateType::Header);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'header' template to tera!");

        let builtin_types = get_builtin_types();

        // 1 - Add global variables
        let global_variables = self.build_global_variables(compilation_unit, &builtin_types);
        context.insert("global_variables", &global_variables);

        // 2 - Add pous (functions, function blocks and programs)
        let functions = self.build_pous(compilation_unit, &builtin_types);
        context.insert("functions", &functions);

        // 3 - Add user-defined data types
        let user_defined_data_types = self.build_user_types(compilation_unit, &builtin_types);
        context.insert("user_defined_data_types", &user_defined_data_types);

        // Set the outputs
        self.contents = tera.render(&template.name, &context).unwrap();

        Ok(())
    }
}

impl GeneratedHeaderForC {
    pub const fn new() -> Self {
        GeneratedHeaderForC {
            file_information: HeaderFileInformation::new(),
            contents: String::new(),
            declared_user_types: Vec::new(),
        }
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

        self.format_global_variables(&global_variables)
    }

    fn build_pous(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) -> String {
        let mut pous: Vec<String> = Vec::new();
        let mut tera = Tera::default();
        let mut context = Context::new();

        for pou in &compilation_unit.pous {
            match pou.kind {
                PouType::Function => {
                    let type_info = &self.get_type_name_for_type(
                        &get_type_from_data_type_decleration(&pou.return_type),
                        builtin_types,
                    );

                    let input_variables = self.get_variables_from_variable_blocks(
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
                    );
                    let input_variables = self.format_function_parameters(&input_variables);

                    let content = self.build_function(&pou.name, &type_info.name, &input_variables);
                    pous.push(content);
                }
                PouType::FunctionBlock => {
                    pous.append(&mut self.build_function_block(
                        &mut tera,
                        &mut context,
                        pou,
                        compilation_unit,
                        builtin_types,
                    ));
                }
                PouType::Program => {
                    let program_name = pou.name.to_string();
                    let data_type = format!("{program_name}_type");

                    let template = self.prepare_and_get_tera_variable_template(
                        &mut tera,
                        &mut context,
                        &data_type,
                        &format!("{program_name}_instance"),
                        &String::new(),
                    );

                    let content = tera.render(&template.name, &context).unwrap().trim().to_string();
                    pous.push(self.format_global_variables(&[content]));

                    pous.append(&mut self.build_function_block(
                        &mut tera,
                        &mut context,
                        pou,
                        compilation_unit,
                        builtin_types,
                    ));
                }
                _ => continue,
            }
        }

        self.format_functions(&pous)
    }

    fn build_function(&mut self, name: &str, data_type: &str, input_variables: &str) -> String {
        let mut tera = Tera::default();
        let mut context = Context::new();
        let template = self.get_template(TemplateType::Function);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'function' template to tera!");

        context.insert("data_type", data_type);
        context.insert("name", name);
        context.insert("variables", input_variables);

        tera.render(&template.name, &context).unwrap().trim().to_string()
    }

    fn build_function_block(
        &mut self,
        tera: &mut Tera,
        context: &mut Context,
        pou: &Pou,
        compilation_unit: &CompilationUnit,
        builtin_types: &[DataType],
    ) -> Vec<String> {
        let mut functions: Vec<String> = Vec::new();
        let type_info = &self
            .get_type_name_for_type(&get_type_from_data_type_decleration(&pou.return_type), builtin_types);

        let function_name = pou.name.to_string();
        let data_type = format!("{function_name}_type");

        // Create the template for the function block user type
        let input_variables = self.get_variables_from_variable_blocks(
            &pou.variable_blocks,
            builtin_types,
            &[
                VariableBlockType::Input(ArgumentProperty::ByRef),
                VariableBlockType::Input(ArgumentProperty::ByVal),
                VariableBlockType::InOut,
                VariableBlockType::Output,
                VariableBlockType::Local,
            ],
            &compilation_unit.user_types,
            &GenerationSource::Struct,
        );

        let template =
            self.prepare_and_get_tera_user_type_struct_template(tera, context, &data_type, &input_variables);

        let content = tera.render(&template.name, context).unwrap().trim().to_string();
        functions.push(content);

        // Create the template for the function block function
        let template = self.prepare_and_get_tera_param_struct_template(
            tera,
            context,
            Some(&String::from("self")),
            Some(&data_type),
        );
        let input_variables = tera.render(&template.name, context).unwrap().trim().to_string();

        let content = self.build_function(&function_name, &type_info.name, &input_variables);
        functions.push(content);

        functions
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

        self.format_user_types(&user_types)
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

        // We generally want to skip the declaration of user types that are only internally relevant
        if let Some(data_type_name) = &user_type.data_type.get_name() {
            if data_type_name.starts_with("__") {
                if let Some(field_name) = field_name_override {
                    if field_name.starts_with("__") {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }

        match &user_type.data_type {
            ast::DataType::StructType { name, variables } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable | GenerationSource::UserType => {
                        let formatted_variables = self.get_formatted_variables_from_variables(
                            variables,
                            builtin_types,
                            false,
                            user_types,
                            &GenerationSource::Struct,
                        );

                        self.prepare_and_get_tera_user_type_struct_template(
                            &mut tera,
                            &mut context,
                            &coalesce_optional_strings_with_default(name, field_name_override),
                            &formatted_variables,
                        )
                    }
                    GenerationSource::Struct | GenerationSource::FunctionParameter => self
                        .prepare_and_get_tera_param_struct_template(
                            &mut tera,
                            &mut context,
                            field_name_override,
                            type_name_override,
                        ),
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::EnumType { name, elements, .. } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable | GenerationSource::UserType => {
                        let template = self.get_template(TemplateType::UserTypeEnum);
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
                        let template = self.get_template(TemplateType::ParamEnum);
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
                    | GenerationSource::UserType => self.prepare_and_get_tera_user_type_array_template(
                        &mut tera,
                        &mut context,
                        &self.get_type_name_for_string(is_wide),
                        &coalesce_optional_strings_with_default(name, field_name_override),
                        &extract_string_size(size),
                    ),
                    GenerationSource::FunctionParameter => self.prepare_and_get_tera_param_array_template(
                        &mut tera,
                        &mut context,
                        &self.get_type_name_for_string(is_wide),
                        field_name_override
                            .expect("Field name expected for generation source type: 'Parameter'!"),
                    ),
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::ArrayType { name, bounds, referenced_type, .. } => {
                let template = match generation_source {
                    GenerationSource::GlobalVariable
                    | GenerationSource::Struct
                    | GenerationSource::UserType => {
                        let type_info = self.get_type_name_for_type(
                            &ExtendedTypeName {
                                type_name: referenced_type.get_name().unwrap().to_string(),
                                is_variadic: false,
                            },
                            builtin_types,
                        );

                        self.prepare_and_get_tera_user_type_array_template(
                            &mut tera,
                            &mut context,
                            &type_info.name,
                            &coalesce_optional_strings_with_default(name, field_name_override),
                            &extract_array_size(bounds),
                        )
                    }
                    GenerationSource::FunctionParameter => {
                        let type_info = self.get_type_name_for_type(
                            &ExtendedTypeName {
                                type_name: referenced_type.get_name().unwrap().to_string(),
                                is_variadic: false,
                            },
                            builtin_types,
                        );

                        self.prepare_and_get_tera_param_array_template(
                            &mut tera,
                            &mut context,
                            &type_info.name,
                            field_name_override
                                .expect("Field name expected for generation source type: 'Parameter'!"),
                        )
                    }
                };

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::PointerType { name, referenced_type, .. } => {
                let type_info = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.get_name().unwrap().to_string(),
                        is_variadic: false,
                    },
                    builtin_types,
                );

                let template = self.prepare_and_get_tera_variable_template(
                    &mut tera,
                    &mut context,
                    &type_info.name,
                    &coalesce_optional_strings_with_default(name, field_name_override),
                    &self.get_reference_symbol(),
                );

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::SubRangeType { name, referenced_type, .. } => {
                let type_info = self.get_type_name_for_type(
                    &ExtendedTypeName { type_name: referenced_type.to_string(), is_variadic: false },
                    builtin_types,
                );

                let template = self.prepare_and_get_tera_variable_template(
                    &mut tera,
                    &mut context,
                    &type_info.name,
                    &coalesce_optional_strings_with_default(name, field_name_override),
                    &String::new(),
                );

                Some(tera.render(&template.name, &context).unwrap().trim().to_string())
            }
            ast::DataType::VarArgs { .. } => Some(self.get_variadic_symbol()),
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

                            formatted_enum_declarations.push(self.format_variable_declaration(left, right));
                        }
                        _ => continue,
                    }
                }
                self.format_enum_fields(&formatted_enum_declarations)
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

        let mut reference_symbol = if is_reference { self.get_reference_symbol() } else { String::new() };

        for variable in variable_block_variables {
            // Handle the special __vtable case
            let type_info = if variable.get_name() == "__vtable" {
                reference_symbol = self.get_reference_symbol();

                self.get_type_name_for_type(
                    &ExtendedTypeName { type_name: LWORD_TYPE.into(), is_variadic: false },
                    builtin_types,
                )
            } else {
                self.get_type_name_for_type(
                    &get_type_from_data_type_decleration(&Some(variable.data_type_declaration.clone())),
                    builtin_types,
                )
            };

            match type_info.attribute {
                TypeAttribute::UserGenerated => {
                    let option_user_type = get_user_generated_type_by_name(&type_info.name, user_types);

                    if let Some(user_type) = option_user_type {
                        let formatted_user_type = self.build_user_type(
                            user_type,
                            builtin_types,
                            user_types,
                            Some(&String::from(variable.get_name())),
                            Some(&type_info.name),
                            generation_source,
                        );

                        if let Some(value) = formatted_user_type {
                            if !self.user_type_can_be_declared_outside_of_a_function(user_type) {
                                self.declared_user_types.push(type_info.name.clone());
                            }
                            variables.push(value);
                        }
                    } else {
                        let template = self.prepare_and_get_tera_variable_template(
                            &mut tera,
                            &mut context,
                            &type_info.name,
                            &variable.get_name().to_string(),
                            &reference_symbol,
                        );

                        variables.push(tera.render(&template.name, &context).unwrap().trim().to_string());
                    }
                }
                TypeAttribute::Variadic => {
                    variables.push(self.get_variadic_symbol());
                }
                TypeAttribute::Other => {
                    let template = self.prepare_and_get_tera_variable_template(
                        &mut tera,
                        &mut context,
                        &type_info.name,
                        &variable.get_name().to_string(),
                        &reference_symbol,
                    );

                    variables.push(tera.render(&template.name, &context).unwrap().trim().to_string());
                }
            }
        }

        variables
    }

    // -- Template Preparation -- \\
    fn prepare_and_get_tera_user_type_struct_template(
        &mut self,
        tera: &mut Tera,
        context: &mut Context,
        name: &String,
        variables: &[String],
    ) -> Template {
        let template = self.get_template(TemplateType::UserTypeStruct);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'user type struct' template to tera!");

        context.insert("name", name);
        context.insert("variables", &self.format_struct_fields(variables));

        template
    }

    fn prepare_and_get_tera_param_struct_template(
        &mut self,
        tera: &mut Tera,
        context: &mut Context,
        name: Option<&String>,
        data_type: Option<&String>,
    ) -> Template {
        let template = self.get_template(TemplateType::ParamStruct);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'param struct' template to tera!");

        context.insert("name", &name.expect("Field name expected for 'param struct' template!"));
        context.insert("data_type", &data_type.expect("Data Type expected for 'param struct' template!"));

        template
    }

    fn prepare_and_get_tera_variable_template(
        &mut self,
        tera: &mut Tera,
        context: &mut Context,
        data_type: &String,
        name: &String,
        reference_symbol: &String,
    ) -> Template {
        let template = self.get_template(TemplateType::Variable);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'variable' template to tera!");

        context.insert("data_type", data_type);
        context.insert("name", name);
        context.insert("reference_symbol", reference_symbol);

        template
    }

    fn prepare_and_get_tera_param_array_template(
        &mut self,
        tera: &mut Tera,
        context: &mut Context,
        data_type: &String,
        name: &String,
    ) -> Template {
        let template = self.get_template(TemplateType::ParamArray);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'param array' template to tera!");

        context.insert("data_type", data_type);
        context.insert("name", name);

        template
    }

    fn prepare_and_get_tera_user_type_array_template(
        &mut self,
        tera: &mut Tera,
        context: &mut Context,
        data_type: &String,
        name: &String,
        size: &String,
    ) -> Template {
        let template = self.get_template(TemplateType::UserTypeArray);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'user type array' template to tera!");

        context.insert("data_type", data_type);
        context.insert("name", name);
        context.insert("size", size);

        template
    }
}
