// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use rustc_hash::FxHashMap;

use plc_util::convention::internal_type_name;

use crate::{
    ast::{
        flatten_expression_list, Assignment, AstFactory, AstNode, AstStatement, CompilationUnit,
        ConfigVariable, DataType, DataTypeDeclaration, LinkageType, Operator, Pou, UserTypeDeclaration,
        Variable, VariableBlock, VariableBlockType,
    },
    literals::AstLiteral,
    provider::IdProvider,
    try_from,
};
use plc_source::source_location::SourceLocation;

pub fn pre_process(unit: &mut CompilationUnit, mut id_provider: IdProvider) {
    //process all local variables from POUs
    for pou in unit.pous.iter_mut() {
        //Find all generic types in that pou
        let generic_types = preprocess_generic_structs(pou);
        unit.user_types.extend(generic_types);

        process_pou_variables(pou, &mut unit.user_types);
    }

    for interface in unit.interfaces.iter_mut().flat_map(|it| &mut it.methods) {
        process_pou_variables(interface, &mut unit.user_types);
    }

    // XXX: Track which hardware-backing globals (__PI_*, __M_*, __G_*) have already been created.
    // Seeded from existing globals so re-runs of the pipeline (e.g. lowering re-indexes) don't
    // produce duplicates. This is a workaround for the fact that pre_process is called multiple
    // times during the pipeline and should be removed when the lowering phase is refactored.
    let mut known_hw_globals: rustc_hash::FxHashSet<String> = unit
        .global_vars
        .iter()
        .flat_map(|block| &block.variables)
        .filter(|var| {
            var.name.starts_with("__PI_") || var.name.starts_with("__M_") || var.name.starts_with("__G_")
        })
        .map(|var| var.name.clone())
        .collect();

    //process all variables from GVLs
    process_global_variables(unit, &mut id_provider, &mut known_hw_globals);
    process_var_config_variables(unit, &mut known_hw_globals);

    // Same as above but for struct members with hardware addresses (e.g. `c AT %IX1.2 : BOOL`
    // inside a STRUCT). Creates the backing __PI_* global and sets the member's initializer.
    process_struct_hardware_variables(unit, &mut id_provider, &mut known_hw_globals);

    //process all variables in dataTypes
    let mut new_types = vec![];
    for dt in unit.user_types.iter_mut() {
        {
            match &mut dt.data_type {
                DataType::StructType { name, variables, .. } => {
                    let name: &str = name.as_ref().map(|it| it.as_str()).unwrap_or("undefined");
                    variables.iter_mut().filter(|it| should_generate_implicit_type(it)).for_each(|var| {
                        pre_process_variable_data_type(name, var, &mut new_types, dt.linkage)
                    });
                }
                DataType::ArrayType { name, referenced_type, .. }
                | DataType::PointerType { name, referenced_type, .. }
                    if should_generate_implicit(referenced_type) =>
                {
                    let name: &str = name.as_ref().map(|it| it.as_str()).unwrap_or("undefined");

                    let type_name = internal_type_name("", name);
                    let type_ref = DataTypeDeclaration::Reference {
                        referenced_type: type_name.clone(),
                        location: SourceLocation::internal(), //return_type.get_location(),
                    };
                    let datatype = std::mem::replace(referenced_type, Box::new(type_ref));
                    if let DataTypeDeclaration::Definition { mut data_type, location, scope } = *datatype {
                        data_type.set_name(type_name);
                        add_nested_datatypes(name, &mut data_type, &mut new_types, &location, dt.linkage);
                        let data_type = UserTypeDeclaration {
                            data_type: *data_type,
                            initializer: None,
                            location,
                            scope,
                            linkage: dt.linkage,
                        };
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
                        .map(|it| {
                            try_from!(it, Assignment).map_or_else(
                                || (extract_flat_ref_name(it), None, it.get_location()),
                                |Assignment { left, right }| {
                                    (
                                        extract_flat_ref_name(left.as_ref()),
                                        Some(*right.clone()),
                                        it.get_location(),
                                    )
                                },
                            )
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

fn process_pou_variables(pou: &mut Pou, user_types: &mut Vec<UserTypeDeclaration>) {
    let local_variables = pou
        .variable_blocks
        .iter_mut()
        .flat_map(|it| it.variables.iter_mut())
        .filter(|it| should_generate_implicit_type(it));

    for var in local_variables {
        pre_process_variable_data_type(pou.name.as_str(), var, user_types, pou.linkage)
    }

    //Generate implicit type for returns
    preprocess_return_type(pou, user_types);
}

fn process_global_variables(
    unit: &mut CompilationUnit,
    id_provider: &mut IdProvider,
    known_hw_globals: &mut rustc_hash::FxHashSet<String>,
) {
    let mut mangled_globals = Vec::new();

    for (linkage, global_var) in
        unit.global_vars.iter_mut().flat_map(|block| block.variables.iter_mut().map(|it| (block.linkage, it)))
    {
        let ref_ty = global_var.data_type_declaration.get_inner_pointer_ty();

        if should_generate_implicit_type(global_var) {
            pre_process_variable_data_type("global", global_var, &mut unit.user_types, linkage)
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

                if known_hw_globals.insert(name.clone()) {
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
    }

    update_generated_globals(unit, mangled_globals);
}

fn process_var_config_variables(
    unit: &mut CompilationUnit,
    known_hw_globals: &mut rustc_hash::FxHashSet<String>,
) {
    let variables = unit.var_config.iter().filter_map(|ConfigVariable { data_type, address, .. }| {
        let AstStatement::HardwareAccess(hardware) = &address.stmt else {
            unreachable!("Must be parsed as hardware access")
        };

        if hardware.is_template() {
            return None;
        }

        let name = hardware.get_mangled_variable_name();
        if !known_hw_globals.insert(name.clone()) {
            return None; // Already exists, skip
        }

        Some(Variable {
            name,
            data_type_declaration: data_type.get_inner_pointer_ty().unwrap_or(data_type.clone()),
            initializer: None,
            address: None,
            location: address.get_location(),
        })
    });

    update_generated_globals(unit, variables.collect());
}

/// Processes struct members declared with IEC hardware addresses (e.g. `c AT %IX1.2 : BOOL`).
/// For each such member, creates a backing global variable (`__PI_1_2`) and sets the member's
/// initializer to reference it so the lowering emits a `REF=` assignment in the struct's constructor.
fn process_struct_hardware_variables(
    unit: &mut CompilationUnit,
    id_provider: &mut IdProvider,
    known_hw_globals: &mut rustc_hash::FxHashSet<String>,
) {
    let mut mangled_globals = Vec::new();

    for dt in unit.user_types.iter_mut() {
        if let DataType::StructType { variables, .. } = &mut dt.data_type {
            for var in variables.iter_mut() {
                if let Some(ref node) = var.address {
                    if let AstStatement::HardwareAccess(hardware) = &node.stmt {
                        if hardware.is_template() {
                            continue;
                        }

                        let name = hardware.get_mangled_variable_name();
                        let ref_ty = var.data_type_declaration.get_inner_pointer_ty();

                        let mangled_initializer = AstFactory::create_member_reference(
                            AstFactory::create_identifier(
                                &name,
                                SourceLocation::internal(),
                                id_provider.next_id(),
                            ),
                            None,
                            id_provider.next_id(),
                        );

                        var.initializer = Some(mangled_initializer);

                        if known_hw_globals.insert(name.clone()) {
                            let internal_mangled_var = Variable {
                                name,
                                data_type_declaration: ref_ty.unwrap_or(var.data_type_declaration.clone()),
                                initializer: None,
                                address: None,
                                location: node.location.clone(),
                            };
                            mangled_globals.push(internal_mangled_var);
                        }
                    }
                }
            }
        }
    }

    update_generated_globals(unit, mangled_globals);
}

fn update_generated_globals(unit: &mut CompilationUnit, mangled_globals: Vec<Variable>) {
    if mangled_globals.is_empty() {
        return;
    }
    let mut block = if let Some(index) = unit
        .global_vars
        .iter()
        .position(|block| block.kind == VariableBlockType::Global && block.location.is_builtin_internal())
    {
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
            linkage: pou.linkage,
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
    let linkage = pou.linkage;
    if let Some(return_type) = &pou.return_type {
        if should_generate_implicit(return_type) {
            let type_name = format!("__{}_return", &pou.name); // TODO: Naming convention (see plc_util/src/convention.rs)
            let type_ref = DataTypeDeclaration::Reference {
                referenced_type: type_name.clone(),
                location: return_type.get_location(),
            };
            let datatype = pou.return_type.replace(type_ref);
            if let Some(DataTypeDeclaration::Definition { mut data_type, location, scope }) = datatype {
                data_type.set_name(type_name);
                add_nested_datatypes(pou.name.as_str(), &mut data_type, types, &location, linkage);
                let data_type = UserTypeDeclaration {
                    data_type: *data_type,
                    initializer: None,
                    location,
                    scope,
                    linkage,
                };
                types.push(data_type);
            }
        }
    }
}

fn should_generate_implicit(datatype: &DataTypeDeclaration) -> bool {
    match datatype {
        DataTypeDeclaration::Reference { .. } | DataTypeDeclaration::Aggregate { .. } => false,
        DataTypeDeclaration::Definition { data_type, .. }
            if matches!(data_type.as_ref(), DataType::VarArgs { .. }) =>
        {
            false
        }
        DataTypeDeclaration::Definition { .. } => true,
    }
}

fn should_generate_implicit_type(variable: &Variable) -> bool {
    should_generate_implicit(&variable.data_type_declaration)
}

fn pre_process_variable_data_type(
    container_name: &str,
    variable: &mut Variable,
    types: &mut Vec<UserTypeDeclaration>,
    linkage: LinkageType,
) {
    let new_type_name = internal_type_name(&format!("{container_name}_"), &variable.name);
    if let DataTypeDeclaration::Definition { mut data_type, location, scope } =
        variable.replace_data_type_with_reference_to(new_type_name.clone())
    {
        // create index entry
        add_nested_datatypes(new_type_name.as_str(), &mut data_type, types, &location, linkage);
        data_type.set_name(new_type_name);
        types.push(UserTypeDeclaration {
            data_type: *data_type,
            initializer: None,
            location,
            scope,
            linkage,
        });
    }
    //make sure it gets generated
}

fn add_nested_datatypes(
    container_name: &str,
    datatype: &mut DataType,
    types: &mut Vec<UserTypeDeclaration>,
    location: &SourceLocation,
    linkage: LinkageType,
) {
    // TODO: Naming convention (see plc_util/src/convention.rs)
    let new_type_name = format!("{container_name}_");
    // FIXME: When processing pointer-to-pointer types (e.g., alias variables pointing to existing pointers),
    // the inner type is already a DataTypeDeclaration::Reference, so replace_data_type_with_reference_to
    // returns None and nested type processing is skipped. This results in incomplete names like "__global_alias_var_"
    // (with trailing underscore but no suffix) when the inner pointer type should be processed.
    // We need to distinguish between pointer references (which should be processed) and other references
    // (which should return None) to properly handle nested pointer structures.
    if let Some(DataTypeDeclaration::Definition { mut data_type, location: inner_location, scope }) =
        datatype.replace_data_type_with_reference_to(new_type_name.clone(), location)
    {
        data_type.set_name(new_type_name.clone());
        add_nested_datatypes(new_type_name.as_str(), &mut data_type, types, &inner_location, linkage);
        types.push(UserTypeDeclaration {
            data_type: *data_type,
            initializer: None,
            location: location.clone(),
            scope,
            linkage,
        });
    }
}

fn replace_generic_type_name(dt: &mut DataTypeDeclaration, generics: &FxHashMap<String, String>) {
    match dt {
        DataTypeDeclaration::Definition { data_type, .. } => match data_type.as_mut() {
            DataType::ArrayType { referenced_type, .. }
            | DataType::PointerType { referenced_type, .. }
            | DataType::VarArgs { referenced_type: Some(referenced_type), .. } => {
                replace_generic_type_name(referenced_type.as_mut(), generics)
            }
            _ => {}
        },
        DataTypeDeclaration::Reference { referenced_type, .. } => {
            if let Some(type_name) = generics.get(referenced_type) {
                referenced_type.clone_from(type_name);
            }
        }
        DataTypeDeclaration::Aggregate { .. } => {} //todo!(),
    }
}
