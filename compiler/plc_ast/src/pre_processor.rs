// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use rustc_hash::FxHashMap;

use plc_util::convention::internal_type_name;

use crate::{
    ast::{
        flatten_expression_list, Assignment, AstFactory, AstNode, AstStatement, CompilationUnit,
        ConfigVariable, DataType, DataTypeDeclaration, Operator, Pou, UserTypeDeclaration, Variable,
        VariableBlock, VariableBlockType,
    },
    literals::AstLiteral,
    provider::IdProvider,
};
use plc_source::source_location::SourceLocation;

pub fn pre_process(unit: &mut CompilationUnit, mut id_provider: IdProvider) {
    //process all local variables from POUs
    for pou in unit.units.iter_mut() {
        //Find all generic types in that pou
        let generic_types = preprocess_generic_structs(pou);
        unit.user_types.extend(generic_types);

        let local_variables = pou
            .variable_blocks
            .iter_mut()
            .flat_map(|it| it.variables.iter_mut())
            .filter(|it| should_generate_implicit_type(it));

        for var in local_variables {
            pre_process_variable_data_type(pou.name.as_str(), var, &mut unit.user_types)
        }

        //Generate implicit type for returns
        preprocess_return_type(pou, &mut unit.user_types);
    }

    //process all variables from GVLs
    process_global_variables(unit, &mut id_provider);
    process_var_config_variables(unit);

    //process all variables in dataTypes
    let mut new_types = vec![];
    for dt in unit.user_types.iter_mut() {
        {
            match &mut dt.data_type {
                DataType::StructType { name, variables, .. } => {
                    let name: &str = name.as_ref().map(|it| it.as_str()).unwrap_or("undefined");
                    variables
                        .iter_mut()
                        .filter(|it| should_generate_implicit_type(it))
                        .for_each(|var| pre_process_variable_data_type(name, var, &mut new_types));
                }
                DataType::ArrayType { name, referenced_type, .. }
                | DataType::PointerType { name, referenced_type, .. }
                    if should_generate_implicit(referenced_type) =>
                {
                    let name: &str = name.as_ref().map(|it| it.as_str()).unwrap_or("undefined");

                    let type_name = internal_type_name("", name);
                    let type_ref = DataTypeDeclaration::DataTypeReference {
                        referenced_type: type_name.clone(),
                        location: SourceLocation::internal(), //return_type.get_location(),
                    };
                    let datatype = std::mem::replace(referenced_type, Box::new(type_ref));
                    if let DataTypeDeclaration::DataTypeDefinition { mut data_type, location, scope } =
                        *datatype
                    {
                        data_type.set_name(type_name);
                        add_nested_datatypes(name, &mut data_type, &mut new_types, &location);
                        let data_type = UserTypeDeclaration { data_type, initializer: None, location, scope };
                        new_types.push(data_type);
                    }
                }
                DataType::EnumType { elements, .. }
                    if matches!(elements.stmt, AstStatement::EmptyStatement { .. }) =>
                {
                    //avoid empty statements, just use an empty expression list to make it easier to work with
                    let _ = std::mem::replace(&mut elements.stmt, AstStatement::ExpressionList(vec![]));
                }
                DataType::EnumType { elements: original_elements, name: Some(enum_name), .. }
                    if !matches!(original_elements.stmt, AstStatement::EmptyStatement { .. }) =>
                {
                    let mut last_name: Option<String> = None;

                    fn extract_flat_ref_name(statement: &AstNode) -> &str {
                        statement.get_flat_reference_name().expect("expected assignment")
                    }

                    let initialized_enum_elements = flatten_expression_list(original_elements)
                        .iter()
                        .map(|it| match &it.stmt {
                            AstStatement::Assignment(Assignment { left, right }) => {
                                //<element-name, initializer, location>
                                (
                                    extract_flat_ref_name(left.as_ref()),
                                    Some(*right.clone()),
                                    it.get_location(),
                                )
                            }
                            _ => (extract_flat_ref_name(it), None, it.get_location()),
                        })
                        .map(|(element_name, initializer, location)| {
                            let enum_literal = initializer.unwrap_or_else(|| {
                                build_enum_initializer(&last_name, &location, &mut id_provider, enum_name)
                            });
                            last_name = Some(element_name.to_string());
                            AstFactory::create_assignment(
                                AstFactory::create_member_reference(
                                    AstFactory::create_identifier(
                                        element_name,
                                        &location,
                                        id_provider.next_id(),
                                    ),
                                    None,
                                    id_provider.next_id(),
                                ),
                                enum_literal,
                                id_provider.next_id(),
                            )
                        })
                        .collect::<Vec<AstNode>>();
                    // if the enum is empty, we dont change anything
                    if !initialized_enum_elements.is_empty() {
                        // we can safely unwrap because we checked the vec
                        let start_loc =
                            initialized_enum_elements.first().expect("non empty vec").get_location();
                        let end_loc =
                            initialized_enum_elements.iter().last().expect("non empty vec").get_location();
                        //swap the expression list with our new Assignments
                        let expression = AstFactory::create_expression_list(
                            initialized_enum_elements,
                            start_loc.span(&end_loc),
                            id_provider.next_id(),
                        );
                        let _ = std::mem::replace(original_elements, expression);
                    }
                }
                _ => {}
            }
        }
    }
    unit.user_types.append(&mut new_types);
}

fn process_global_variables(unit: &mut CompilationUnit, id_provider: &mut IdProvider) {
    let mut mangled_globals = Vec::new();

    for global_var in unit.global_vars.iter_mut().flat_map(|block| block.variables.iter_mut()) {
        let ref_ty = global_var.data_type_declaration.get_inner_pointer_ty();

        if should_generate_implicit_type(global_var) {
            pre_process_variable_data_type("global", global_var, &mut unit.user_types)
        }

        // In any case, we have to inject initializers into aliased hardware access variables
        if let Some(ref node) = global_var.address {
            if let AstStatement::HardwareAccess(hardware) = &node.stmt {
                let name = hardware.get_mangled_variable_name();

                // %I*: DWORD; should not be declared at this stage, it is just skipped
                if hardware.is_template() {
                    continue;
                }

                let mangled_initializer = AstFactory::create_member_reference(
                    AstFactory::create_identifier(&name, SourceLocation::internal(), id_provider.next_id()),
                    None,
                    id_provider.next_id(),
                );

                global_var.initializer = Some(mangled_initializer);

                let internal_mangled_var = Variable {
                    name,
                    data_type_declaration: ref_ty.unwrap_or(global_var.data_type_declaration.clone()),
                    initializer: None,
                    address: None,
                    location: node.location.clone(),
                };
                mangled_globals.push(internal_mangled_var);
            }
        }
    }

    update_generated_globals(unit, mangled_globals);
}

fn process_var_config_variables(unit: &mut CompilationUnit) {
    let variables = unit.var_config.iter().filter_map(|ConfigVariable { data_type, address, .. }| {
        let AstStatement::HardwareAccess(hardware) = &address.stmt else {
            unreachable!("Must be parsed as hardware access")
        };

        if hardware.is_template() {
            return None;
        }

        let name = hardware.get_mangled_variable_name();

        Some(Variable {
            name,
            data_type_declaration: data_type.get_inner_pointer_ty().unwrap_or(data_type.clone()),
            initializer: None,
            address: None,
            // XXX: using the VAR_CONFIG location here clashes with pre-existing globals using the same address
            location: SourceLocation::internal(),
            // location: location.clone(),
        })
    });

    update_generated_globals(unit, variables.collect())
}

fn update_generated_globals(unit: &mut CompilationUnit, mangled_globals: Vec<Variable>) {
    let mut block = if let Some(index) = unit.global_vars.iter().position(|block| {
        block.variable_block_type == VariableBlockType::Global && block.location.is_internal()
    }) {
        unit.global_vars.remove(index)
    } else {
        VariableBlock::default().with_block_type(VariableBlockType::Global)
    };
    for var in mangled_globals {
        if !block.variables.contains(&var) {
            block.variables.push(var);
        }
    }
    unit.global_vars.push(block);
}

fn build_enum_initializer(
    last_name: &Option<String>,
    location: &SourceLocation,
    id_provider: &mut IdProvider,
    enum_name: &mut str,
) -> AstNode {
    if let Some(last_element) = last_name.as_ref() {
        // generate a `enum#last + 1` statement
        let enum_ref = AstFactory::create_identifier(last_element, location, id_provider.next_id());
        let type_element = AstFactory::create_member_reference(
            AstFactory::create_identifier(enum_name, location, id_provider.next_id()),
            None,
            id_provider.next_id(),
        );
        AstFactory::create_binary_expression(
            AstFactory::create_cast_statement(type_element, enum_ref, location, id_provider.next_id()),
            Operator::Plus,
            AstNode::new_literal(AstLiteral::new_integer(1), id_provider.next_id(), location.clone()),
            id_provider.next_id(),
        )
    } else {
        AstNode::new_literal(AstLiteral::new_integer(0), id_provider.next_id(), location.clone())
    }
}

fn preprocess_generic_structs(pou: &mut Pou) -> Vec<UserTypeDeclaration> {
    let mut generic_types = FxHashMap::default();
    let mut types = vec![];
    for binding in &pou.generics {
        let new_name = format!("__{}__{}", pou.name, binding.name); // TODO: Naming convention (see plc_util/src/convention.rs)

        //Generate a type for the generic
        let data_type = UserTypeDeclaration {
            data_type: DataType::GenericType {
                name: new_name.clone(),
                generic_symbol: binding.name.clone(),
                nature: binding.nature,
            },
            initializer: None,
            scope: Some(pou.name.clone()),
            location: pou.location.clone(),
        };
        types.push(data_type);
        generic_types.insert(binding.name.clone(), new_name);
    }
    for var in pou.variable_blocks.iter_mut().flat_map(|it| it.variables.iter_mut()) {
        replace_generic_type_name(&mut var.data_type_declaration, &generic_types);
    }
    if let Some(datatype) = pou.return_type.as_mut() {
        replace_generic_type_name(datatype, &generic_types);
    };
    types
}

fn preprocess_return_type(pou: &mut Pou, types: &mut Vec<UserTypeDeclaration>) {
    if let Some(return_type) = &pou.return_type {
        if should_generate_implicit(return_type) {
            let type_name = format!("__{}_return", &pou.name); // TODO: Naming convention (see plc_util/src/convention.rs)
            let type_ref = DataTypeDeclaration::DataTypeReference {
                referenced_type: type_name.clone(),
                location: return_type.get_location(),
            };
            let datatype = std::mem::replace(&mut pou.return_type, Some(type_ref));
            if let Some(DataTypeDeclaration::DataTypeDefinition { mut data_type, location, scope }) = datatype
            {
                data_type.set_name(type_name);
                add_nested_datatypes(pou.name.as_str(), &mut data_type, types, &location);
                let data_type = UserTypeDeclaration { data_type, initializer: None, location, scope };
                types.push(data_type);
            }
        }
    }
}

fn should_generate_implicit(datatype: &DataTypeDeclaration) -> bool {
    match datatype {
        DataTypeDeclaration::DataTypeReference { .. } => false,
        DataTypeDeclaration::DataTypeDefinition { data_type: DataType::VarArgs { .. }, .. } => false,
        DataTypeDeclaration::DataTypeDefinition { .. } => true,
    }
}

fn should_generate_implicit_type(variable: &Variable) -> bool {
    should_generate_implicit(&variable.data_type_declaration)
}

fn pre_process_variable_data_type(
    container_name: &str,
    variable: &mut Variable,
    types: &mut Vec<UserTypeDeclaration>,
) {
    let new_type_name = internal_type_name(&format!("{container_name}_"), &variable.name);
    if let DataTypeDeclaration::DataTypeDefinition { mut data_type, location, scope } =
        variable.replace_data_type_with_reference_to(new_type_name.clone())
    {
        // create index entry
        add_nested_datatypes(new_type_name.as_str(), &mut data_type, types, &location);
        data_type.set_name(new_type_name);
        types.push(UserTypeDeclaration { data_type, initializer: None, location, scope });
    }
    //make sure it gets generated
}

fn add_nested_datatypes(
    container_name: &str,
    datatype: &mut DataType,
    types: &mut Vec<UserTypeDeclaration>,
    location: &SourceLocation,
) {
    let new_type_name = format!("{container_name}_"); // TODO: Naming convention (see plc_util/src/convention.rs)
    if let Some(DataTypeDeclaration::DataTypeDefinition { mut data_type, location: inner_location, scope }) =
        datatype.replace_data_type_with_reference_to(new_type_name.clone(), location)
    {
        data_type.set_name(new_type_name.clone());
        add_nested_datatypes(new_type_name.as_str(), &mut data_type, types, &inner_location);
        types.push(UserTypeDeclaration { data_type, initializer: None, location: location.clone(), scope });
    }
}

fn replace_generic_type_name(dt: &mut DataTypeDeclaration, generics: &FxHashMap<String, String>) {
    match dt {
        DataTypeDeclaration::DataTypeDefinition { data_type, .. } => match data_type {
            DataType::ArrayType { referenced_type, .. }
            | DataType::PointerType { referenced_type, .. }
            | DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                replace_generic_type_name(referenced_type.as_mut(), generics)
            }
            _ => {}
        },
        DataTypeDeclaration::DataTypeReference { referenced_type, .. } => {
            if let Some(type_name) = generics.get(referenced_type) {
                referenced_type.clone_from(type_name);
            }
        }
    }
}
