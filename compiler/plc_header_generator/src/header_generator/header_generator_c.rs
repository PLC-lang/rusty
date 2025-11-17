use std::collections::HashMap;

use plc::typesystem::{get_builtin_types, DataType, LWORD_TYPE};
use plc_ast::ast::{
    self, ArgumentProperty, CompilationUnit, Identifier, Pou, PouType, UserTypeDeclaration, VariableBlock,
    VariableBlockType,
};
use plc_diagnostics::diagnostics::Diagnostic;
use tera::{from_value, to_value, Context, Tera};

use crate::header_generator::{
    coalesce_field_name_override_with_default, data_type_is_system_generated, extract_array_size,
    extract_enum_declaration_from_elements, extract_string_size,
    file_helper::HeaderFileInformation,
    get_type_from_data_type_decleration, get_user_generated_type_by_name,
    symbol_helper::SymbolHelper,
    template_helper::{
        Function, TemplateData, TemplateHelper, TemplateType, UserType, Variable, VariableType,
    },
    type_helper::{TypeAttribute, TypeHelper},
    ExtendedTypeName, GeneratedHeader,
};

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
        self.file_information.directory.is_empty()
            && self.file_information.path.is_empty()
            && self.contents.is_empty()
    }

    fn get_contents(&self) -> &str {
        &self.contents
    }

    fn prepare_template_data(&mut self, compilation_unit: &CompilationUnit) {
        let builtin_types = get_builtin_types();

        self.prepare_global_variables(compilation_unit, &builtin_types);
        self.prepare_user_types(compilation_unit, &builtin_types);
        self.prepare_functions(compilation_unit, &builtin_types);
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

        context.insert("global_variables", &self.template_data.global_variables);
        context.insert("user_defined_types", &self.template_data.user_defined_types);
        context.insert("functions", &self.template_data.functions);

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

    fn prepare_user_types(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) {
        for user_type in &compilation_unit.user_types {
            self.prepare_user_type(user_type, builtin_types, &compilation_unit.user_types);
        }
    }

    fn prepare_global_variables(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) {
        self.template_data.global_variables = self.get_variables_from_variable_blocks(
            &compilation_unit.global_vars,
            builtin_types,
            &[VariableBlockType::Global],
            &compilation_unit.user_types,
        );
    }

    fn prepare_functions(&mut self, compilation_unit: &CompilationUnit, builtin_types: &[DataType]) {
        for pou in &compilation_unit.pous {
            match pou.kind {
                PouType::Function => {
                    let type_info = &self.get_type_name_for_type(
                        &get_type_from_data_type_decleration(&pou.return_type),
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
                        &compilation_unit.user_types,
                    );

                    self.template_data.functions.push(Function {
                        name: pou.name.to_string(),
                        return_type: type_info.get_type_name(),
                        parameters,
                    });
                }
                PouType::FunctionBlock => {
                    self.prepare_function_block(pou, compilation_unit, builtin_types);
                }
                PouType::Program => {
                    let program_name = pou.name.to_string();
                    let data_type = format!("{program_name}_type");

                    self.template_data.global_variables.push(Variable {
                        data_type,
                        name: format!("{program_name}_instance"),
                        variable_type: VariableType::Default,
                    });

                    self.prepare_function_block(pou, compilation_unit, builtin_types);
                }
                _ => continue,
            }
        }
    }

    fn prepare_user_type(
        &mut self,
        user_type: &UserTypeDeclaration,
        builtin_types: &[DataType],
        user_types: &[UserTypeDeclaration],
    ) {
        // We generally want to skip the declaration of user types that are only internally relevant
        if let Some(data_type_name) = &user_type.data_type.get_name() {
            if data_type_name.starts_with("__") {
                return;
            }
        }

        match &user_type.data_type {
            ast::DataType::StructType { name, variables } => {
                let formatted_variables =
                    self.get_transformed_variables_from_variables(variables, builtin_types, false, user_types);

                self.template_data.user_defined_types.structs.push(UserType {
                    name: name.clone().unwrap_or_default(),
                    variables: formatted_variables,
                });
            }
            ast::DataType::EnumType { name, elements, .. } => {
                let enum_declerations = extract_enum_declaration_from_elements(elements);

                self.template_data
                    .user_defined_types
                    .enums
                    .push(UserType { name: name.clone().unwrap_or_default(), variables: enum_declerations });
            }
            ast::DataType::ArrayType { name, bounds, referenced_type, .. } => {
                self.template_data.user_defined_types.aliases.push(Variable {
                    data_type: referenced_type.get_name().unwrap_or_default().to_string(),
                    name: name.clone().unwrap_or_default(),
                    variable_type: VariableType::Array(extract_array_size(bounds)),
                });
            }
            ast::DataType::PointerType { name, referenced_type, .. } => {
                let data_type = format!(
                    "{}{}",
                    referenced_type.get_name().unwrap_or_default(),
                    self.get_reference_symbol()
                );

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
                self.template_data.user_defined_types.aliases.push(Variable {
                    data_type: referenced_type.clone(),
                    name: name.clone().unwrap_or_default(),
                    variable_type: VariableType::Default,
                });
            }
            _ => {
                // The rest of these are not managed here
            }
        }
    }

    fn prepare_function_block(
        &mut self,
        pou: &Pou,
        compilation_unit: &CompilationUnit,
        builtin_types: &[DataType],
    ) {
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
        );

        if pou.super_class.is_some() {
            self.template_data.user_defined_types.structs.push(UserType {
                name: data_type.to_string(),
                variables: self
                    .modify_function_block_variables_for_inheritance(&input_variables, &pou.super_class),
            });
        } else {
            self.template_data
                .user_defined_types
                .structs
                .push(UserType { name: data_type.to_string(), variables: input_variables });
        }

        let mut parameters: Vec<Variable> = Vec::new();
        parameters.push(Variable {
            data_type: format!("{data_type}{}", self.get_reference_symbol()),
            name: String::from("self"),
            variable_type: VariableType::Default,
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
        pou_super_class: &Option<Identifier>,
    ) -> Vec<Variable> {
        let mut modified_input_variables: Vec<Variable> = Vec::new();

        if let Some(super_class) = &pou_super_class {
            for input_variable in input_variables {
                if input_variable.data_type == super_class.name {
                    modified_input_variables.push(Variable {
                        name: input_variable.name.clone(),
                        data_type: format!("{}_type", input_variable.data_type),
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
        }

        modified_input_variables
    }

    /// Transforms the variables in an array of [variable blocks](plc_ast::ast::VariableBlock) into simplified [variables](crate::header_generator::template_helper::Variable)
    /// that can be rendered by the template.
    fn get_variables_from_variable_blocks(
        &mut self,
        variable_blocks: &[VariableBlock],
        builtin_types: &[DataType],
        variable_block_types: &[VariableBlockType],
        user_types: &[UserTypeDeclaration],
    ) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();

        for variable_block in variable_blocks {
            if variable_block_types.contains(&variable_block.kind) {
                let is_reference = variable_block.kind == VariableBlockType::Input(ArgumentProperty::ByRef)
                    || variable_block.kind == VariableBlockType::InOut
                    || variable_block.kind == VariableBlockType::Output;

                variables.append(&mut self.get_transformed_variables_from_variables(
                    &variable_block.variables,
                    builtin_types,
                    is_reference,
                    user_types,
                ));
            }
        }

        variables
    }

    /// Transforms an array of [ast variables](plc_ast::ast::Variable) into simplified [variables](crate::header_generator::template_helper::Variable)
    /// that can be rendered by the template.
    fn get_transformed_variables_from_variables(
        &mut self,
        variable_block_variables: &[plc_ast::ast::Variable],
        builtin_types: &[DataType],
        is_reference: bool,
        user_types: &[UserTypeDeclaration],
    ) -> Vec<Variable> {
        let mut variables: Vec<Variable> = Vec::new();
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
                TypeAttribute::Variadic => {
                    variables.push(Variable {
                        data_type,
                        name: variable.get_name().to_string(),
                        variable_type: VariableType::Variadic,
                    });
                }
                TypeAttribute::Other => {
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
            ast::DataType::StructType { name, .. } => Some(Variable {
                name: coalesce_field_name_override_with_default(name, field_name_override),
                data_type: type_name_override.unwrap().to_string(),
                variable_type: VariableType::Struct,
            }),
            ast::DataType::EnumType { name, .. } => Some(Variable {
                name: coalesce_field_name_override_with_default(name, field_name_override),
                data_type: type_name_override.unwrap().to_string(),
                variable_type: VariableType::Default,
            }),
            ast::DataType::StringType { name, size, is_wide } => Some(Variable {
                name: coalesce_field_name_override_with_default(name, field_name_override),
                data_type: self.get_type_name_for_string(is_wide),
                variable_type: VariableType::Array(extract_string_size(size)),
            }),
            ast::DataType::ArrayType { name, bounds, referenced_type, .. } => {
                let type_info = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.get_name().unwrap().to_string(),
                        is_variadic: false,
                    },
                    builtin_types,
                );

                Some(Variable {
                    name: coalesce_field_name_override_with_default(name, field_name_override),
                    data_type: type_info.get_type_name(),
                    variable_type: VariableType::Array(extract_array_size(bounds)),
                })
            }
            ast::DataType::PointerType { name, referenced_type, .. } => {
                let type_info = self.get_type_name_for_type(
                    &ExtendedTypeName {
                        type_name: referenced_type.get_name().unwrap().to_string(),
                        is_variadic: false,
                    },
                    builtin_types,
                );

                let data_type = format!("{}{}", type_info.get_type_name(), self.get_reference_symbol());

                Some(Variable {
                    name: coalesce_field_name_override_with_default(name, field_name_override),
                    data_type,
                    variable_type: VariableType::Default,
                })
            }
            ast::DataType::SubRangeType { name, referenced_type, .. } => {
                let type_info = self.get_type_name_for_type(
                    &ExtendedTypeName { type_name: referenced_type.to_string(), is_variadic: false },
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
                        Ok(to_value(format!("{} = {}", variable.name, right)).unwrap())
                    }
                    _ => Ok(to_value(format!("{}{}", variable.data_type, variable.name)).unwrap()),
                },
                Err(_) => Err("Unable to format variable for parameter!".into()),
            },
            None => Err("Unable to format variable for parameter!".into()),
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
                    VariableType::Array(_) | VariableType::Struct => {
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
