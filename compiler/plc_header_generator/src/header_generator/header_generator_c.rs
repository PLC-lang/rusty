use std::collections::HashMap;

use plc::typesystem::{get_builtin_types, DataType, DINT_TYPE, LWORD_TYPE};
use plc_ast::ast::{
    self, ArgumentProperty, CompilationUnit, Identifier, Implementation, LinkageType, Pou, PouType,
    UserTypeDeclaration, VariableBlock, VariableBlockType,
};
use plc_diagnostics::diagnostics::Diagnostic;
use tera::{from_value, to_value, Context, Tera};

use crate::header_generator::{
    coalesce_field_name_override_with_default, data_type_is_system_generated, determine_array_type,
    extract_enum_declaration_from_elements, extract_string_size,
    file_helper::HeaderFileInformation,
    get_type_from_data_type_decleration, get_user_generated_type_by_name, sanitize_method_name,
    symbol_helper::SymbolHelper,
    template_helper::{
        Function, TemplateData, TemplateHelper, TemplateType, UserType, Variable, VariableType,
    },
    type_helper::{TypeAttribute, TypeHelper},
    ExtendedTypeName, GeneratedHeader,
};

/// The constant value for the string that is appended to types added by the header generation process
const TYPE_APPEND: &str = "_type";

pub struct GeneratedHeaderForC {
    pub file_information: HeaderFileInformation,
    contents: String,
    pub template_data: TemplateData,
}

impl Default for GeneratedHeaderForC {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedHeader for GeneratedHeaderForC {
    fn is_empty(&self) -> bool {
        (self.file_information.directory.is_empty()
            && self.file_information.path.is_empty()
            && self.contents.is_empty())
            || self.template_data.is_empty()
    }

    fn get_contents(&self) -> &str {
        &self.contents
    }

    fn prepare_template_data(&mut self, compilation_unit: &CompilationUnit) {
        let builtin_types = get_builtin_types();

        self.prepare_global_variables(compilation_unit, &builtin_types);
        self.prepare_user_types(compilation_unit, &builtin_types);
        self.prepare_functions(compilation_unit, &builtin_types);
        self.resolve_alias_dependencies();
    }

    fn generate_headers(&mut self) -> Result<(), Diagnostic> {
        // Configure tera
        let mut tera = Tera::default();
        let mut context = Context::new();

        let template = self.get_template(TemplateType::Header);
        tera.add_raw_template(&template.name, &template.content)
            .expect("Unable to add the 'header' template to tera!");
        tera.register_function(
            "format_variable_for_parameter",
            format_variable_for_parameter(self.get_reference_symbol(), self.get_variadic_symbol()),
        );
        tera.register_function("format_variable_for_definition", format_variable_for_definition());
        tera.register_function("format_variable_for_enum_definition", format_variable_for_enum_definition());
        tera.register_function("is_array_with_size", is_array_with_size());
        tera.register_function(
            "format_variable_for_function_comment",
            format_variable_for_function_comment(),
        );

        context.insert("global_variables", &self.template_data.global_variables);
        context.insert("user_defined_types", &self.template_data.user_defined_types);
        context.insert("functions", &self.template_data.functions);
        context.insert("file_name_caps", &self.file_information.formatted_path);

        // Set the outputs
        self.contents = tera.render(&template.name, &context).unwrap();

        Ok(())
    }
}

impl GeneratedHeaderForC {
    pub const fn new() -> Self {
        GeneratedHeaderForC {
            file_information: HeaderFileInformation::new(),
            template_data: TemplateData::new(),
            contents: String::new(),
        }
    }

    /// Populates the self scoped [TemplateData] instance with the user types that should be added to the generated header file
    fn prepare_user_types(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) {
        for user_type in &compilation_unit.user_types {
            if user_type.linkage == LinkageType::External {
                continue;
            }

            self.prepare_user_type(
                user_type,
                builtin_types,
                &compilation_unit.user_types,
                &compilation_unit.pous,
            );
        }
    }

    /// Populates the self scoped [TemplateData] instance with the global variables that should be added to the generated header file
    fn prepare_global_variables(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) {
        self.template_data.global_variables = self.get_variables_from_variable_blocks(
            &compilation_unit.global_vars,
            builtin_types,
            &[VariableBlockType::Global],
            &compilation_unit.user_types,
            (&compilation_unit.pous, None),
            false,
        );
    }

    /// Populates the self scoped [TemplateData] instance with the functions that should be added to the generated header file
    fn prepare_functions(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) {
        for pou in &compilation_unit.pous {
            if pou.linkage == LinkageType::External {
                continue;
            }

            match &pou.kind {
                PouType::Function => {
                    if let Some(function) = self.get_function(
                        pou,
                        &compilation_unit.user_types,
                        builtin_types,
                        &compilation_unit.pous,
                    ) {
                        self.template_data.functions.push(function);
                    }
                }
                PouType::FunctionBlock => {
                    self.prepare_function_block(pou, compilation_unit, builtin_types, &compilation_unit.pous);
                }
                PouType::Program => {
                    let program_name = pou.name.to_string();
                    let data_type = format!("{program_name}{TYPE_APPEND}");

                    // Adds the global variable instance for this program
                    self.template_data.global_variables.push(Variable {
                        data_type,
                        name: format!("{program_name}_instance"),
                        variable_type: VariableType::Default,
                    });

                    self.prepare_function_block(pou, compilation_unit, builtin_types, &compilation_unit.pous);
                }
                PouType::Method { parent, .. } => {
                    let type_info = &self.get_type_name_for_type(
                        &get_type_from_data_type_decleration(&pou.return_type, false),
                        builtin_types,
                    );

                    let data_type = format!("{parent}{TYPE_APPEND}");

                    let mut parameters: Vec<Variable> = Vec::new();
                    parameters.push(Variable {
                        data_type: format!("{data_type}{}", self.get_reference_symbol()),
                        name: String::from("self"),
                        variable_type: VariableType::Default,
                    });

                    parameters.append(&mut self.get_variables_from_variable_blocks(
                        &pou.variable_blocks,
                        builtin_types,
                        &[
                            VariableBlockType::Input(ArgumentProperty::ByRef),
                            VariableBlockType::Input(ArgumentProperty::ByVal),
                            VariableBlockType::InOut,
                            VariableBlockType::Output,
                        ],
                        &compilation_unit.user_types,
                        (&compilation_unit.pous, Some(&pou.kind)),
                        true,
                    ));

                    self.template_data.functions.push(Function {
                        name: sanitize_method_name(&pou.name),
                        return_type: type_info.get_type_name(),
                        parameters,
                    });
                }
                _ => continue,
            }
        }

        for implementation in &compilation_unit.implementations {
            if implementation.linkage == LinkageType::External {
                continue;
            }

            match &implementation.pou_type {
                PouType::Action => {
                    if let Some(function) = self.get_action(implementation, builtin_types) {
                        self.template_data.functions.push(function);
                    }
                }
                _ => continue,
            }
        }
    }

    /// Get the function definition given a pou
    fn get_function(
        &mut self,
        pou: &Pou,
        user_types: &[UserTypeDeclaration],
        builtin_types: &[DataType],
        pous: &Vec<Pou>,
    ) -> Option<Function> {
        match pou.kind {
            PouType::Function => {
                let type_info = &self.get_type_name_for_type(
                    &get_type_from_data_type_decleration(&pou.return_type, false),
                    builtin_types,
                );

                let parameters = self.get_variables_from_variable_blocks(
                    &pou.variable_blocks,
                    builtin_types,
                    &[
                        VariableBlockType::Input(ArgumentProperty::ByRef),
                        VariableBlockType::Input(ArgumentProperty::ByVal),
                        VariableBlockType::InOut,
                        VariableBlockType::Output,
                    ],
                    user_types,
                    (pous, Some(&pou.kind)),
                    true,
                );

                Some(Function {
                    name: sanitize_method_name(&pou.name),
                    return_type: type_info.get_type_name(),
                    parameters,
                })
            }
            _ => None,
        }
    }

    /// Get the action definition given an implementation
    fn get_action(
        &mut self,
        implementation: &Implementation,
        builtin_types: &[DataType],
    ) -> Option<Function> {
        match implementation.pou_type {
            PouType::Action => {
                // Get void type
                let void_type = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: String::new(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                );

                let function_name = implementation.type_name.to_string();
                let data_type = format!("{function_name}{TYPE_APPEND}");

                // Push the parameters for the function
                let mut parameters: Vec<Variable> = Vec::new();
                parameters.push(Variable {
                    data_type: format!("{data_type}{}", self.get_reference_symbol()),
                    name: String::from("self"),
                    variable_type: VariableType::Default,
                });

                Some(Function {
                    name: sanitize_method_name(&implementation.name),
                    return_type: void_type.get_type_name(),
                    parameters,
                })
            }
            _ => None,
        }
    }

    /// Populates the self scoped [TemplateData] instance with the user type that should be added to the generated header file
    fn prepare_user_type(
        &mut self,
        user_type: &UserTypeDeclaration,
        builtin_types: &[DataType],
        user_types: &[UserTypeDeclaration],
        pous: &Vec<Pou>,
    ) {
        if !self.user_type_can_be_transformed_for_header(user_type, builtin_types) {
            return;
        }

        match &user_type.data_type {
            ast::DataType::StructType { name, variables } => {
                let formatted_variables = self.get_transformed_variables_from_variables(
                    variables,
                    builtin_types,
                    false,
                    user_types,
                    pous,
                );

                self.template_data.user_defined_types.structs.push(UserType {
                    name: name.clone().unwrap_or_default(),
                    variables: formatted_variables,
                    data_type: None,
                });
            }
            ast::DataType::EnumType { name, elements, numeric_type } => {
                // The assumption here is that the name has already been parsed and populated
                let name = name.clone().unwrap_or_default();
                let enum_declerations = extract_enum_declaration_from_elements(&name, elements);

                // The assumption here is that the numeric type has already been parsed and populated
                let type_information = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: numeric_type.to_string(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                );

                self.template_data.user_defined_types.enums.push(UserType {
                    name,
                    variables: enum_declerations,
                    data_type: Some(type_information.get_type_name()),
                });
            }
            ast::DataType::ArrayType { name, bounds, referenced_type, .. } => {
                // The assumption here is that the referenced type has already been parsed and populated
                let type_information = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.get_name().unwrap_or_default().to_string(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                );

                if let Some(variable_type) = determine_array_type(bounds) {
                    self.template_data.user_defined_types.aliases.push(Variable {
                        data_type: type_information.get_type_name(),
                        name: name.clone().unwrap_or_default(),
                        variable_type,
                    });
                } else {
                    log::warn!(
                        "Array type cannot be determined for array with name: {} but none found!",
                        name.clone().unwrap_or_default()
                    )
                }
            }
            ast::DataType::PointerType { name, referenced_type, .. } => {
                // The assumption here is that the referenced type has already been parsed and populated
                let type_information = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.get_name().unwrap_or_default().to_string(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                );

                let data_type =
                    format!("{}{}", type_information.get_type_name(), self.get_reference_symbol());

                self.template_data.user_defined_types.aliases.push(Variable {
                    data_type,
                    name: name.clone().unwrap_or_default(),
                    variable_type: VariableType::Default,
                });
            }
            ast::DataType::StringType { name, is_wide, size } => {
                self.template_data.user_defined_types.aliases.push(Variable {
                    data_type: self.get_type_name_for_string(is_wide),
                    name: name.clone().unwrap_or_default(),
                    variable_type: VariableType::Array(extract_string_size(size)),
                });
            }
            ast::DataType::SubRangeType { name, referenced_type, .. } => {
                let type_information = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.to_string(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                );

                self.template_data.user_defined_types.aliases.push(Variable {
                    data_type: type_information.get_type_name(),
                    name: name.clone().unwrap_or_default(),
                    variable_type: VariableType::Default,
                });
            }
            _ => {
                // The rest of these are not managed here
            }
        }
    }

    /// Verifies whether a user type is transformable for declaration in the header
    fn user_type_can_be_transformed_for_header(
        &mut self,
        user_type: &UserTypeDeclaration,
        _: &[DataType],
    ) -> bool {
        if let Some(data_type_name) = &user_type.data_type.get_name() {
            if data_type_name.starts_with("__") {
                match &user_type.data_type {
                    // Support for multi-dimensional arrays
                    ast::DataType::ArrayType { .. } => return true,
                    _ => return false,
                }
            }
        }

        true
    }

    /// Populates the self scoped [TemplateData] instance with the necessary structs and functions created by a function block implementation
    fn prepare_function_block(
        &mut self,
        pou: &Pou,
        compilation_unit: &CompilationUnit,
        builtin_types: &[DataType],
        pous: &Vec<Pou>,
    ) {
        let type_info = &self.get_type_name_for_type(
            &get_type_from_data_type_decleration(&pou.return_type, false),
            builtin_types,
        );

        let function_name = pou.name.to_string();
        let data_type = format!("{function_name}{TYPE_APPEND}");

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
            (pous, Some(&pou.kind)),
            true,
        );

        if let Some(super_class) = &pou.super_class {
            self.template_data.user_defined_types.structs.push(UserType {
                name: data_type.to_string(),
                variables: self
                    .modify_function_block_variables_for_inheritance(&input_variables, super_class),
                data_type: None,
            });
        } else {
            self.template_data.user_defined_types.structs.push(UserType {
                name: data_type.to_string(),
                variables: input_variables,
                data_type: None,
            });
        }

        // Get void type
        let void_type = self.get_type_name_for_type(
            &ExtendedTypeName { type_name: String::new(), is_variadic: false, is_sized_variadic: false },
            builtin_types,
        );

        // Push the parameters for the function
        let mut parameters: Vec<Variable> = Vec::new();
        parameters.push(Variable {
            data_type: format!("{data_type}{}", self.get_reference_symbol()),
            name: String::from("self"),
            variable_type: VariableType::Default,
        });

        // TODO: May need to be removed after the initialization changes are made to the compiler
        self.template_data.functions.push(Function {
            return_type: void_type.get_type_name(),
            name: format!("__{function_name}__init"),
            parameters: parameters.clone(),
        });

        self.template_data.functions.push(Function {
            return_type: type_info.get_type_name(),
            name: function_name,
            parameters,
        });
    }

    /// Modifies the variables from a function block to account for differences in the naming convention of the super class.
    fn modify_function_block_variables_for_inheritance(
        &self,
        input_variables: &[Variable],
        super_class: &Identifier,
    ) -> Vec<Variable> {
        let mut modified_input_variables: Vec<Variable> = Vec::new();

        for input_variable in input_variables {
            if input_variable.data_type == super_class.name {
                modified_input_variables.push(Variable {
                    name: input_variable.name.clone(),
                    data_type: format!("{}{TYPE_APPEND}", input_variable.data_type),
                    variable_type: input_variable.variable_type.clone(),
                });
            } else {
                modified_input_variables.push(Variable {
                    name: input_variable.name.clone(),
                    data_type: input_variable.data_type.clone(),
                    variable_type: input_variable.variable_type.clone(),
                });
            }
        }

        modified_input_variables
    }

    /// Transforms the variables in an array of [variable blocks](plc_ast::ast::VariableBlock) into simplified [variables](crate::header_generator::template_helper::Variable)
    /// that can be rendered by the template.
    ///
    /// ---
    ///
    /// Might return an empty [Vec<Variable>] if no variables are found within the variable blocks.
    fn get_variables_from_variable_blocks(
        &mut self,
        variable_blocks: &[VariableBlock],
        builtin_types: &[DataType],
        variable_block_types: &[VariableBlockType],
        user_types: &[UserTypeDeclaration],
        (pous, pou_type): (&Vec<Pou>, Option<&PouType>),
        include_external: bool,
    ) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();

        for variable_block in variable_blocks {
            if variable_block_types.contains(&variable_block.kind)
                && (include_external || variable_block.linkage != LinkageType::External)
            {
                let is_reference = if let Some(pou_type) = pou_type {
                    variable_block.kind == VariableBlockType::Input(ArgumentProperty::ByRef)
                        || variable_block.kind == VariableBlockType::InOut
                        || (variable_block.kind == VariableBlockType::Output && !pou_type.is_stateful())
                } else {
                    variable_block.kind == VariableBlockType::Input(ArgumentProperty::ByRef)
                        || variable_block.kind == VariableBlockType::InOut
                };

                variables.append(&mut self.get_transformed_variables_from_variables(
                    &variable_block.variables,
                    builtin_types,
                    is_reference,
                    user_types,
                    pous,
                ));
            }
        }

        variables
    }

    /// Transforms an array of [ast variables](plc_ast::ast::Variable) into simplified [variables](crate::header_generator::template_helper::Variable)
    /// that can be rendered by the template.
    ///
    /// ---
    ///
    /// Might return an empty [Vec<Variable>] if no variables are supplied.
    fn get_transformed_variables_from_variables(
        &mut self,
        variable_block_variables: &[plc_ast::ast::Variable],
        builtin_types: &[DataType],
        is_reference: bool,
        user_types: &[UserTypeDeclaration],
        pous: &Vec<Pou>,
    ) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
        let mut reference_symbol = if is_reference { self.get_reference_symbol() } else { String::new() };

        for variable in variable_block_variables {
            // Handle the special __vtable case
            let type_info = if variable.get_name() == "__vtable" {
                reference_symbol = self.get_reference_symbol();

                self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: LWORD_TYPE.into(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                )
            } else {
                self.get_type_name_for_type(
                    &get_type_from_data_type_decleration(
                        &Some(variable.data_type_declaration.clone()),
                        false,
                    ),
                    builtin_types,
                )
            };

            let data_type = format!("{}{reference_symbol}", type_info.get_type_name());

            match type_info.attribute {
                TypeAttribute::UserGenerated => {
                    let type_name = type_info.get_type_name();
                    let option_user_type = get_user_generated_type_by_name(&type_name, user_types);

                    if let Some(user_type) = option_user_type {
                        let user_type_variable = self.get_user_type_variable(
                            user_type,
                            builtin_types,
                            Some(&String::from(variable.get_name())),
                            Some(&type_name),
                        );

                        if let Some(value) = user_type_variable {
                            // Function pointers
                            let data_type_with_no_reference_symbol = value.data_type.replace("*", "");
                            let option_pou =
                                pous.iter().find(|pou| pou.name == data_type_with_no_reference_symbol);

                            if let Some(pou) = option_pou {
                                if let Some(function) =
                                    self.get_function(pou, user_types, builtin_types, pous)
                                {
                                    let prototype_name = format!("{}_ptr", function.name);
                                    let function_pointer_name = format!(
                                        "({}{})({})",
                                        self.get_reference_symbol(),
                                        prototype_name,
                                        self.get_formatted_function_parameter_data_types(&function)
                                    );

                                    if !self
                                        .template_data
                                        .user_defined_types
                                        .aliases
                                        .iter()
                                        .any(|f| f.name == function_pointer_name)
                                    {
                                        self.template_data.user_defined_types.aliases.push(Variable {
                                            data_type: function.return_type,
                                            name: function_pointer_name,
                                            variable_type: VariableType::Default,
                                        });
                                    }

                                    variables.push(Variable {
                                        data_type: prototype_name,
                                        name: value.name,
                                        variable_type: VariableType::Default,
                                    });

                                    continue;
                                }
                            }

                            // This is an alias
                            if type_name != value.data_type && !data_type_is_system_generated(&type_name) {
                                variables.push(Variable {
                                    data_type: type_name,
                                    name: value.name,
                                    variable_type: value.variable_type,
                                })
                            } else {
                                variables.push(value);
                            }
                        }
                    } else {
                        variables.push(Variable {
                            data_type,
                            name: variable.get_name().to_string(),
                            variable_type: VariableType::Default,
                        });
                    }
                }
                TypeAttribute::Variadic(is_sized) => {
                    if !is_sized {
                        variables.push(Variable {
                            data_type,
                            name: variable.get_name().to_string(),
                            variable_type: VariableType::Variadic,
                        });
                    } else {
                        // Push an integer for the size of the variadic
                        let dint_type_info = self.get_type_name_for_type(
                            &ExtendedTypeName {
                                type_name: DINT_TYPE.into(),
                                is_variadic: false,
                                is_sized_variadic: false,
                            },
                            builtin_types,
                        );

                        let size_variable_name = format!("{}_count", variable.get_name());
                        variables.push(Variable {
                            data_type: dint_type_info.get_type_name(),
                            name: size_variable_name,
                            variable_type: VariableType::Default,
                        });

                        // Push a reference for the variadic's type
                        let underlying_type_info = self.get_type_name_for_type(
                            &get_type_from_data_type_decleration(
                                &Some(variable.data_type_declaration.clone()),
                                true,
                            ),
                            builtin_types,
                        );

                        let reference_to_data_type = match underlying_type_info.attribute {
                            TypeAttribute::Array(_) => {
                                format!(
                                    "{}{}",
                                    underlying_type_info.get_type_name(),
                                    self.get_reference_symbol()
                                )
                            }
                            _ => underlying_type_info.get_type_name(),
                        };

                        variables.push(Variable {
                            data_type: reference_to_data_type,
                            name: variable.get_name().to_string(),
                            variable_type: VariableType::Array(0),
                        });
                    }
                }
                TypeAttribute::Array(size) => {
                    variables.push(Variable {
                        data_type,
                        name: variable.get_name().to_string(),
                        variable_type: VariableType::Array(size),
                    });
                }
                TypeAttribute::Default => {
                    variables.push(Variable {
                        data_type,
                        name: variable.get_name().to_string(),
                        variable_type: VariableType::Default,
                    });
                }
            }
        }

        variables
    }

    /// Returns a list of data types from the function parameters as a formatted list
    ///
    /// ---
    ///
    /// Example:
    ///
    /// Given the parameters: int32_t someInt, char* someString
    ///
    /// The return will be: int32_t, char*
    fn get_formatted_function_parameter_data_types(&self, function: &Function) -> String {
        let mut parameters: Vec<String> = Vec::new();

        for parameter in &function.parameters {
            match parameter.variable_type {
                VariableType::Array(_) | VariableType::Struct => {
                    parameters.push(format!("{}{}", parameter.data_type, self.get_reference_symbol()));
                }
                _ => parameters.push(parameter.data_type.to_string()),
            }
        }

        parameters.join(", ")
    }

    /// Returns an [Variable] based on a given [UserTypeDeclaration]
    ///
    /// ---
    ///
    /// Might return [None] if the data type name of the [UserTypeDeclaration] is system generated.
    /// This is because these system generated user types are not relevant to the header.
    fn get_user_type_variable(
        &mut self,
        user_type: &UserTypeDeclaration,
        builtin_types: &[DataType],
        field_name_override: Option<&String>,
        type_name_override: Option<&String>,
    ) -> Option<Variable> {
        // We generally want to skip the declaration of user types that are only internally relevant
        if let Some(data_type_name) = user_type.data_type.get_name() {
            if data_type_is_system_generated(data_type_name) {
                if let Some(field_name) = field_name_override {
                    if data_type_is_system_generated(field_name) {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }

        match &user_type.data_type {
            ast::DataType::StructType { name, .. } => {
                let name = coalesce_field_name_override_with_default(name, field_name_override);
                if let Some(type_name_override) = type_name_override {
                    Some(Variable {
                        name,
                        data_type: type_name_override.to_string(),
                        variable_type: VariableType::Struct,
                    })
                } else {
                    log::warn!("type_name_override expected for struct with name: {name} but none supplied!");
                    None
                }
            }
            ast::DataType::EnumType { name, .. } => {
                let name = coalesce_field_name_override_with_default(name, field_name_override);
                if let Some(type_name_override) = type_name_override {
                    Some(Variable {
                        name,
                        data_type: type_name_override.to_string(),
                        variable_type: VariableType::Default,
                    })
                } else {
                    log::warn!("type_name_override expected for enum with name: {name} but none supplied!");
                    None
                }
            }
            ast::DataType::StringType { name, size, is_wide } => Some(Variable {
                name: coalesce_field_name_override_with_default(name, field_name_override),
                data_type: self.get_type_name_for_string(is_wide),
                variable_type: VariableType::Array(extract_string_size(size)),
            }),
            ast::DataType::ArrayType { name, bounds, referenced_type, .. } => {
                // The assumption here is that the referenced type has been given a name already by the parser
                // As a sanity check we will return "None" and log an error if no name for this referenced type is found
                let name = coalesce_field_name_override_with_default(name, field_name_override);
                if let Some(referenced_type_name) = referenced_type.get_name() {
                    let type_info = self.get_type_name_for_type(
                        &ExtendedTypeName {
                            type_name: referenced_type_name.to_string(),
                            is_variadic: false,
                            is_sized_variadic: false,
                        },
                        builtin_types,
                    );

                    if let Some(variable_type) = determine_array_type(bounds) {
                        Some(Variable { name, data_type: type_info.get_type_name(), variable_type })
                    } else {
                        log::warn!(
                            "Array type cannot be determined for array with name: {name} but none found!"
                        );
                        None
                    }
                } else {
                    log::warn!("Referenced type expected for array with name: {name} but none found!");
                    None
                }
            }
            ast::DataType::PointerType { name, referenced_type, .. } => {
                // The assumption here is that the referenced type has been given a name already by the parser
                // As a sanity check we will return "None" and log an error if no name for this referenced type is found
                let name = coalesce_field_name_override_with_default(name, field_name_override);
                if let Some(referenced_type_name) = referenced_type.get_name() {
                    let type_info = self.get_type_name_for_type(
                        &ExtendedTypeName {
                            type_name: referenced_type_name.to_string(),
                            is_variadic: false,
                            is_sized_variadic: false,
                        },
                        builtin_types,
                    );

                    let data_type = format!("{}{}", type_info.get_type_name(), self.get_reference_symbol());

                    Some(Variable { name, data_type, variable_type: VariableType::Default })
                } else {
                    log::warn!("Referenced type expected for pointer with name: {name} but none found!");
                    None
                }
            }
            ast::DataType::SubRangeType { name, referenced_type, .. } => {
                let type_info = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.to_string(),
                        is_variadic: false,
                        is_sized_variadic: false,
                    },
                    builtin_types,
                );

                Some(Variable {
                    name: coalesce_field_name_override_with_default(name, field_name_override),
                    data_type: type_info.get_type_name(),
                    variable_type: VariableType::Default,
                })
            }
            ast::DataType::VarArgs { .. } => Some(Variable {
                name: String::new(),
                data_type: String::new(),
                variable_type: VariableType::Variadic,
            }),
            ast::DataType::GenericType { .. } => {
                // Currently out of scope
                None
            }
        }
    }

    /// Cleans up the typedef declarations that are not used
    ///
    /// ---
    ///
    /// Sometimes additional generated typedefs slip through that are not useful to the implementation of the interface.
    /// This method will remove those generated typedefs if they are not used by anything in the interface
    fn cleanup_typedef_declarations_if_not_used(&mut self) {
        let mut names: Vec<String> = Vec::new();

        for variable in &self.template_data.user_defined_types.aliases {
            let mut reference_found = false;

            // We only want to evaluate generated types
            if !variable.name.starts_with("__") {
                reference_found = true;
            }

            // Check all aliases
            if !reference_found
                && check_for_usage_of_current_variable_in_variable_list(
                    variable,
                    &self.template_data.user_defined_types.aliases,
                )
            {
                reference_found = true;
            }

            // Check all struct variables
            if !reference_found {
                for user_struct in &self.template_data.user_defined_types.structs {
                    if check_for_usage_of_current_variable_in_variable_list(variable, &user_struct.variables)
                    {
                        reference_found = true;
                        break;
                    }
                }
            }

            // Check all global variables
            if !reference_found
                && check_for_usage_of_current_variable_in_variable_list(
                    variable,
                    &self.template_data.global_variables,
                )
            {
                reference_found = true;
            }

            // Check all function parameters
            if !reference_found {
                for function in &self.template_data.functions {
                    if check_for_usage_of_current_variable_in_variable_list(variable, &function.parameters) {
                        reference_found = true;
                        break;
                    }
                }
            }

            if !reference_found {
                names.push(variable.name.clone());
            }
        }

        for name in names {
            if let Some(index) =
                self.template_data.user_defined_types.aliases.iter().position(|alias| alias.name == name)
            {
                self.template_data.user_defined_types.aliases.swap_remove(index);
            }
        }
    }

    /// Ensure that aliases are declared in the correct order to ensure that no unknown type error occurs
    fn resolve_alias_dependencies(&mut self) {
        let mut aliases = self.template_data.user_defined_types.aliases.clone();
        let mut indices: Vec<usize> = Vec::new();

        resolve_variable_dependency_order_in_variable_list(
            &mut indices,
            &mut aliases,
            &self.template_data.user_defined_types.aliases,
        );

        // We re-use the now empty aliases here
        for index in indices.iter().copied().rev() {
            aliases.push(self.template_data.user_defined_types.aliases[index].clone());
        }

        // Set aliases to the now correctly ordered aliases
        self.template_data.user_defined_types.aliases = aliases;

        self.cleanup_typedef_declarations_if_not_used();
    }
}

/// Formats a variable for definition within an enum block.
///
/// It handles the case where a enum may contiain a value on the right side for definition, or none.
///
/// ---
///
/// This function is used by the templating engine [tera](https://keats.github.io/tera/).
fn format_variable_for_enum_definition() -> impl tera::Function {
    Box::new(move |args: &HashMap<String, serde_json::Value>| -> tera::Result<serde_json::Value> {
        match args.get("variable") {
            Some(value) => match from_value::<Variable>(value.clone()) {
                Ok(variable) => match variable.variable_type {
                    VariableType::Declaration(right) => {
                        // Fetch the parent from naming convention
                        Ok(to_value(format!("{} (({}){})", variable.name, variable.data_type, right))
                            .unwrap())
                    }
                    _ => Err("Unable to format enum variable for parameter!".into()),
                },
                Err(_) => Err("Unable to format enum variable for parameter!".into()),
            },
            None => Err("Unable to format enum variable for parameter!".into()),
        }
    })
}

/// Formats a variable for standalone definition.
///
/// ---
///
/// This function is used by the templating engine [tera](https://keats.github.io/tera/).
fn format_variable_for_definition() -> impl tera::Function {
    Box::new(move |args: &HashMap<String, serde_json::Value>| -> tera::Result<serde_json::Value> {
        match args.get("variable") {
            Some(value) => match from_value::<Variable>(value.clone()) {
                Ok(variable) => match variable.variable_type {
                    VariableType::Array(size) => {
                        Ok(to_value(format!("{} {}[{}]", variable.data_type, variable.name, size)).unwrap())
                    }
                    VariableType::MultidimensionalArray(sizes) => {
                        let mut size_str = String::new();

                        for size in sizes {
                            size_str += &format!("[{size}]");
                        }

                        Ok(to_value(format!("{} {}{}", variable.data_type, variable.name, size_str)).unwrap())
                    }
                    VariableType::Declaration(right) => {
                        Ok(to_value(format!("{} {} = {}", variable.data_type, variable.name, right)).unwrap())
                    }
                    _ => Ok(to_value(format!("{} {}", variable.data_type, variable.name)).unwrap()),
                },
                Err(_) => Err("Unable to format variable for parameter!".into()),
            },
            None => Err("Unable to format variable for parameter!".into()),
        }
    })
}

/// Formats a variable for definition as a parameter.
///
/// i.e. When defined within a function's brackets "()"
///
/// ---
///
/// This function is used by the templating engine [tera](https://keats.github.io/tera/).
fn format_variable_for_parameter(reference_symbol: String, variadic_symbol: String) -> impl tera::Function {
    Box::new(move |args: &HashMap<String, serde_json::Value>| -> tera::Result<serde_json::Value> {
        match args.get("variable") {
            Some(value) => match from_value::<Variable>(value.clone()) {
                Ok(variable) => match variable.variable_type {
                    VariableType::Array(_)
                    | VariableType::MultidimensionalArray(_)
                    | VariableType::Struct => {
                        Ok(to_value(format!("{}{} {}", variable.data_type, reference_symbol, variable.name))
                            .unwrap())
                    }
                    VariableType::Variadic => Ok(to_value(variadic_symbol.to_string()).unwrap()),
                    _ => Ok(to_value(format!("{} {}", variable.data_type, variable.name)).unwrap()),
                },
                Err(_) => Err("Unable to format variable for parameter!".into()),
            },
            None => Err("Unable to format variable for parameter!".into()),
        }
    })
}

/// Returns whether or not this variable is an array that has a non-zero size
///
/// ---
///
/// This function is used by the templating engine [tera](https://keats.github.io/tera/).
fn is_array_with_size() -> impl tera::Function {
    Box::new(move |args: &HashMap<String, serde_json::Value>| -> tera::Result<serde_json::Value> {
        match args.get("variable") {
            Some(value) => match from_value::<Variable>(value.clone()) {
                Ok(variable) => match variable.variable_type {
                    VariableType::Array(size) => Ok(to_value(size > 0).unwrap()),
                    VariableType::MultidimensionalArray(sizes) => {
                        if !sizes.is_empty() {
                            return Ok(to_value(sizes[0] > 0).unwrap());
                        }

                        Ok(to_value(false).unwrap())
                    }
                    _ => Ok(to_value(false).unwrap()),
                },
                Err(_) => Err("Unable to determine if this is an array with a size!".into()),
            },
            None => Err("Unable to determine if this is an array with a size!".into()),
        }
    })
}

/// Formats a variable as a comment for a function
///
/// i.e. // someVar: maximum of 200 entries
///
/// ---
///
/// This function is used by the templating engine [tera](https://keats.github.io/tera/).
fn format_variable_for_function_comment() -> impl tera::Function {
    Box::new(move |args: &HashMap<String, serde_json::Value>| -> tera::Result<serde_json::Value> {
        match args.get("variable") {
            Some(value) => match from_value::<Variable>(value.clone()) {
                Ok(variable) => match variable.variable_type {
                    VariableType::Array(size) => Ok(to_value(format!(
                        "// {}: maximum of {} {}(s)",
                        variable.name, size, variable.data_type
                    ))
                    .unwrap()),
                    VariableType::MultidimensionalArray(sizes) => {
                        if !sizes.is_empty() {
                            return Ok(to_value(format!(
                                "// {}: maximum of {} {}(s)",
                                variable.name, sizes[0], variable.data_type
                            ))
                            .unwrap());
                        }

                        Ok(to_value("").unwrap())
                    }
                    _ => Ok(to_value("").unwrap()),
                },
                Err(_) => Err("Unable to format variable for function comment!".into()),
            },
            None => Err("Unable to format variable for function comment!".into()),
        }
    })
}

/// Checks for the usage of the current variable in a given variable list
fn check_for_usage_of_current_variable_in_variable_list(
    current_variable: &Variable,
    variables: &Vec<Variable>,
) -> bool {
    for variable in variables {
        if variable.data_type == current_variable.name {
            return true;
        }
    }

    false
}

fn resolve_variable_dependency_order_in_variable_list(
    indices: &mut Vec<usize>,
    variables: &mut Vec<Variable>,
    full_variables: &Vec<Variable>,
) {
    let mut top_level_names: Vec<String> = Vec::new();

    for variable in variables.iter() {
        if !check_for_usage_of_current_variable_in_variable_list(variable, variables) {
            top_level_names.push(variable.name.clone());

            if let Some(full_index) =
                full_variables.iter().position(|full_variable| full_variable.name == variable.name)
            {
                if !indices.contains(&full_index) {
                    indices.push(full_index);
                }
            }
        }
    }

    for name in top_level_names {
        if let Some(index) = variables.iter().position(|variable| variable.name == name) {
            variables.swap_remove(index);
        }
    }

    if !variables.is_empty() {
        resolve_variable_dependency_order_in_variable_list(indices, variables, full_variables);
    }
}
