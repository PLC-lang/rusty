// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{HardwareBinding, PouIndexEntry, VariableIndexEntry, VariableType};
use crate::index::{ArgumentType, Index, MemberInfo};
use crate::typesystem::{self, *};
use plc_ast::ast::{
    self, ArgumentProperty, Assignment, AstFactory, AstNode, AstStatement, AutoDerefType, CompilationUnit,
    DataType, DataTypeDeclaration, Implementation, Pou, PouType, RangeStatement, TypeNature,
    UserTypeDeclaration, Variable, VariableBlock, VariableBlockType,
};
use plc_ast::literals::AstLiteral;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::SourceLocation;
use plc_util::convention::internal_type_name;

pub fn visit(unit: &CompilationUnit) -> Index {
    let mut index = Index::default();
    //Create user defined datatypes
    for user_type in &unit.user_types {
        visit_data_type(&mut index, user_type);
    }

    //Create defined global variables
    for global_vars in &unit.global_vars {
        visit_global_var_block(&mut index, global_vars);
    }

    //Create types and variables for POUs
    for pou in &unit.units {
        visit_pou(&mut index, pou);
    }

    for implementation in &unit.implementations {
        visit_implementation(&mut index, implementation);
    }
    index
}

pub fn visit_pou(index: &mut Index, pou: &Pou) {
    let mut members = vec![];

    //register the pou's member variables
    let mut member_varargs = None;
    let mut count = 0;
    for block in &pou.variable_blocks {
        let block_type = get_declaration_type_for(block, &pou.pou_type);
        for var in &block.variables {
            let varargs = if let DataTypeDeclaration::DataTypeDefinition {
                data_type: ast::DataType::VarArgs { referenced_type, sized },
                ..
            } = &var.data_type_declaration
            {
                let name = referenced_type
                    .as_ref()
                    .map(|it| &**it)
                    .and_then(DataTypeDeclaration::get_name)
                    .map(|it| it.to_string());
                Some(if *sized { VarArgs::Sized(name) } else { VarArgs::Unsized(name) })
            } else {
                None
            };

            if varargs.is_some() {
                member_varargs.clone_from(&varargs);
            }

            let var_type_name = var.data_type_declaration.get_name().unwrap_or(VOID_TYPE);
            let type_name = if block_type.is_by_ref() {
                //register a pointer type for argument
                register_byref_pointer_type_for(index, var_type_name)
            } else {
                var_type_name.to_string()
            };
            let initial_value = index.get_mut_const_expressions().maybe_add_constant_expression(
                var.initializer.clone(),
                type_name.as_str(),
                Some(pou.name.clone()),
                Some(var.get_name().to_string()),
            );

            let binding = var
                .address
                .as_ref()
                .and_then(|it| HardwareBinding::from_statement(index, it, Some(pou.name.clone())));

            let entry = index.register_member_variable(
                MemberInfo {
                    container_name: &pou.name,
                    variable_name: &var.name,
                    variable_linkage: block_type,
                    variable_type_name: &type_name,
                    is_constant: block.constant,
                    is_var_external: matches!(block.variable_block_type, VariableBlockType::External),
                    binding,
                    varargs,
                },
                initial_value,
                var.location.clone(),
                count,
            );
            members.push(entry);
            count += 1;
        }
    }

    //register a function's return type as a member variable
    let return_type_name = pou.return_type.as_ref().and_then(|it| it.get_name()).unwrap_or(VOID_TYPE);
    if pou.return_type.is_some() {
        let entry = index.register_member_variable(
            MemberInfo {
                container_name: &pou.name,
                variable_name: pou.get_return_name(),
                variable_linkage: ArgumentType::ByVal(VariableType::Return),
                variable_type_name: return_type_name,
                is_constant: false,     //return variables are not constants
                is_var_external: false, // see above
                binding: None,
                varargs: None,
            },
            None,
            pou.name_location.clone(),
            count,
        );
        members.push(entry);
    }

    let has_varargs = member_varargs.is_some();
    let datatype = typesystem::DataType {
        name: pou.name.to_string(),
        initial_value: None,
        information: DataTypeInformation::Struct {
            name: pou.name.to_string(),
            members,
            source: StructSource::Pou(pou.pou_type.clone()),
        },
        nature: TypeNature::Any,
        location: pou.name_location.clone(),
    };

    match &pou.pou_type {
        PouType::Program => {
            index.register_program(&pou.name, pou.name_location.clone(), pou.linkage);
            index.register_pou_type(datatype);
        }
        PouType::FunctionBlock => {
            let global_struct_name = crate::index::get_initializer_name(&pou.name);
            let variable = VariableIndexEntry::create_global(
                &global_struct_name,
                &global_struct_name,
                &pou.name,
                pou.name_location.clone(),
            )
            .set_constant(true)
            .set_linkage(pou.linkage);
            index.register_global_initializer(&global_struct_name, variable);
            index.register_pou(PouIndexEntry::create_function_block_entry(
                &pou.name,
                pou.linkage,
                pou.name_location.clone(),
                pou.super_class.clone().as_deref(),
            ));
            index.register_pou_type(datatype);
        }
        PouType::Class => {
            let global_struct_name = crate::index::get_initializer_name(&pou.name);
            let variable = VariableIndexEntry::create_global(
                &global_struct_name,
                &global_struct_name,
                &pou.name,
                pou.name_location.clone(),
            )
            .set_constant(true)
            .set_linkage(pou.linkage);
            index.register_global_initializer(&global_struct_name, variable);
            index.register_pou(PouIndexEntry::create_class_entry(
                &pou.name,
                pou.linkage,
                pou.name_location.clone(),
                pou.super_class.clone(),
            ));
            index.register_pou_type(datatype);
        }
        PouType::Function | PouType::Init | PouType::ProjectInit => {
            index.register_pou(PouIndexEntry::create_function_entry(
                &pou.name,
                return_type_name,
                &pou.generics,
                pou.linkage,
                has_varargs,
                pou.name_location.clone(),
            ));
            index.register_pou_type(datatype);
        }
        PouType::Method { owner_class } => {
            index.register_pou(PouIndexEntry::create_method_entry(
                &pou.name,
                return_type_name,
                owner_class,
                pou.linkage,
                pou.name_location.clone(),
            ));
            index.register_pou_type(datatype);
        }
        _ => {}
    };
}

/// returns the declaration type (ByRef or ByVal) for the given VariableBlock (VAR_INPUT, VAR_OUTPUT, VAR_INOUT, etc.)
fn get_declaration_type_for(block: &VariableBlock, pou_type: &PouType) -> ArgumentType {
    if matches!(
        block.variable_block_type,
        VariableBlockType::InOut | VariableBlockType::Input(ArgumentProperty::ByRef)
    ) {
        ArgumentType::ByRef(get_variable_type_from_block(block))
    } else if block.variable_block_type == VariableBlockType::Output {
        // outputs differ depending on pou type
        match pou_type {
            PouType::Function => ArgumentType::ByRef(get_variable_type_from_block(block)),
            _ => ArgumentType::ByVal(get_variable_type_from_block(block)),
        }
    } else {
        ArgumentType::ByVal(get_variable_type_from_block(block))
    }
}

fn visit_implementation(index: &mut Index, implementation: &Implementation) {
    let pou_type = &implementation.pou_type;
    let start_location = implementation
        .statements
        .first()
        .map(|it| it.get_location())
        .as_ref()
        .or(Some(&implementation.location))
        .cloned()
        .unwrap();
    index.register_implementation(
        &implementation.name,
        &implementation.type_name,
        pou_type.get_optional_owner_class().as_ref(),
        pou_type.into(),
        implementation.generic,
        start_location,
    );
    //if we are registing an action, also register a datatype for it
    if pou_type == &PouType::Action {
        let datatype = typesystem::DataType {
            name: implementation.name.to_string(),
            initial_value: None,
            information: DataTypeInformation::Alias {
                name: implementation.name.clone(),
                referenced_type: implementation.type_name.clone(),
            },
            nature: TypeNature::Derived,
            location: implementation.name_location.clone(),
        };

        index.register_pou(PouIndexEntry::create_action_entry(
            implementation.name.as_str(),
            implementation.type_name.as_str(),
            implementation.linkage,
            implementation.name_location.clone(),
        ));
        index.register_pou_type(datatype);
    }
}

/// registers an auto-deref pointer type for the inner_type_name if it does not already exist
fn register_byref_pointer_type_for(index: &mut Index, inner_type_name: &str) -> String {
    //get unique name
    let type_name = internal_type_name("auto_pointer_to_", inner_type_name);

    //check if type was already created
    if index.find_effective_type_by_name(type_name.as_str()).is_none() {
        //generate a pointertype for the variable
        index.register_type(typesystem::DataType {
            name: type_name.clone(),
            initial_value: None,
            information: DataTypeInformation::Pointer {
                name: type_name.clone(),
                inner_type_name: inner_type_name.to_string(),
                auto_deref: Some(AutoDerefType::Default),
            },
            nature: TypeNature::Any,
            location: SourceLocation::internal(),
        });
    }

    type_name
}

fn visit_global_var_block(index: &mut Index, block: &VariableBlock) {
    let linkage = block.linkage;
    for var in &block.variables {
        let target_type = var.data_type_declaration.get_name().unwrap_or_default();
        let initializer = index.get_mut_const_expressions().maybe_add_constant_expression(
            var.initializer.clone(),
            target_type,
            None,
            Some(var.get_name().into()),
        );
        let variable = VariableIndexEntry::create_global(
            &var.name,
            &var.name,
            var.data_type_declaration.get_name().expect("named variable datatype"),
            var.location.clone(),
        )
        .set_initial_value(initializer)
        .set_constant(block.constant)
        .set_linkage(linkage)
        .set_hardware_binding(
            var.address.as_ref().and_then(|it| HardwareBinding::from_statement(index, it, None)),
        );
        index.register_global_variable(&var.name, variable);
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    match block.variable_block_type {
        VariableBlockType::Local => VariableType::Local,
        VariableBlockType::Temp => VariableType::Temp,
        VariableBlockType::Input(_) => VariableType::Input,
        VariableBlockType::Output => VariableType::Output,
        VariableBlockType::Global => VariableType::Global,
        VariableBlockType::InOut => VariableType::InOut,
        VariableBlockType::External => VariableType::External,
    }
}

fn visit_data_type(index: &mut Index, type_declaration: &UserTypeDeclaration) {
    let data_type = &type_declaration.data_type;
    let scope = &type_declaration.scope;
    //names should not be empty
    match data_type {
        DataType::StructType { name: Some(name), variables } => {
            visit_struct(name, variables, index, scope, type_declaration, StructSource::OriginalDeclaration);
        }

        DataType::EnumType { name: Some(name), elements, numeric_type, .. } => {
            let mut variants = Vec::new();

            for ele in ast::flatten_expression_list(elements) {
                let variant = ast::get_enum_element_name(ele);
                if let AstStatement::Assignment(Assignment { right, .. }) = ele.get_stmt() {
                    let init = index.get_mut_const_expressions().add_constant_expression(
                        right.as_ref().clone(),
                        numeric_type.clone(),
                        scope.clone(),
                        None,
                    );

                    variants.push(index.register_enum_variant(name, &variant, Some(init), ele.get_location()))
                } else {
                    unreachable!("the preprocessor should have provided explicit assignments for enum values")
                }
            }

            let information = DataTypeInformation::Enum {
                name: name.to_owned(),
                variants,
                referenced_type: numeric_type.clone(),
            };

            let init = index.get_mut_const_expressions().maybe_add_constant_expression(
                type_declaration.initializer.clone(),
                name,
                scope.clone(),
                None,
            );

            index.register_type(typesystem::DataType {
                name: name.to_owned(),
                initial_value: init,
                information,
                nature: TypeNature::Int,
                location: type_declaration.location.clone(),
            });
        }

        DataType::SubRangeType { name: Some(name), referenced_type, bounds } => {
            let information = if let Some(AstStatement::RangeStatement(RangeStatement { start, end })) =
                bounds.as_ref().map(|it| it.get_stmt())
            {
                DataTypeInformation::SubRange {
                    name: name.into(),
                    referenced_type: referenced_type.into(),
                    sub_range: (*start.clone()..*end.clone()),
                }
            } else {
                DataTypeInformation::Alias { name: name.into(), referenced_type: referenced_type.into() }
            };

            let init = index.get_mut_const_expressions().maybe_add_constant_expression(
                type_declaration.initializer.clone(),
                name,
                scope.clone(),
                None,
            );
            index.register_type(typesystem::DataType {
                name: name.to_string(),
                initial_value: init,
                information,
                nature: TypeNature::Int,
                location: type_declaration.location.clone(),
            });
        }
        DataType::ArrayType { name: Some(name), referenced_type, bounds, is_variable_length }
            if *is_variable_length =>
        {
            visit_variable_length_array(bounds, referenced_type, name, index, type_declaration);
        }
        DataType::ArrayType { name: Some(name), bounds, referenced_type, .. } => {
            visit_array(bounds, index, scope, referenced_type, name, type_declaration);
        }
        DataType::PointerType { name: Some(name), referenced_type, auto_deref: kind } => {
            let inner_type_name = referenced_type.get_name().expect("named datatype");
            let information = DataTypeInformation::Pointer {
                name: name.clone(),
                inner_type_name: inner_type_name.into(),
                auto_deref: *kind,
            };

            let init = index.get_mut_const_expressions().maybe_add_constant_expression(
                type_declaration.initializer.clone(),
                name,
                scope.clone(),
                None,
            );
            index.register_type(typesystem::DataType {
                name: name.to_string(),
                initial_value: init,
                information,
                nature: TypeNature::Any,
                location: type_declaration.location.clone(),
            });
        }
        DataType::StringType { name: Some(name), size, is_wide, .. } => {
            let type_name = name;
            let encoding = if *is_wide { StringEncoding::Utf16 } else { StringEncoding::Utf8 };

            let size = match size {
                Some(AstNode { stmt: AstStatement::Literal(AstLiteral::Integer(value)), .. }) => {
                    TypeSize::from_literal((value + 1) as i64)
                }
                Some(statement) => {
                    // construct a "x + 1" expression because we need one additional character for \0 terminator
                    let len_plus_1 = AstFactory::create_binary_expression(
                        statement.clone(),
                        ast::Operator::Plus,
                        AstNode::new_literal(
                            AstLiteral::new_integer(1),
                            statement.get_id(),
                            statement.get_location(),
                        ),
                        statement.get_id(),
                    );

                    TypeSize::from_expression(index.get_mut_const_expressions().add_constant_expression(
                        len_plus_1,
                        type_name.clone(),
                        scope.clone(),
                        None,
                    ))
                }
                None => TypeSize::from_literal((DEFAULT_STRING_LEN + 1).into()),
            };
            let information = DataTypeInformation::String { size, encoding };
            let init = index.get_mut_const_expressions().maybe_add_constant_expression(
                type_declaration.initializer.clone(),
                type_name,
                scope.clone(),
                None,
            );
            index.register_type(typesystem::DataType {
                name: name.to_string(),
                initial_value: init,
                information,
                nature: TypeNature::String,
                location: type_declaration.location.clone(),
            });

            if init.is_some() {
                // register a global variable with the initial value to memcopy from
                let global_init_name = crate::index::get_initializer_name(name);
                let initializer_global = VariableIndexEntry::create_global(
                    global_init_name.as_str(),
                    global_init_name.as_str(),
                    name,
                    type_declaration.location.clone(),
                )
                .set_constant(true)
                .set_initial_value(init);
                index.register_global_initializer(global_init_name.as_str(), initializer_global);
            }
        }
        DataType::VarArgs { .. } => {} //Varargs are not indexed,
        DataType::GenericType { name, generic_symbol, nature } => {
            let information = DataTypeInformation::Generic {
                name: name.clone(),
                generic_symbol: generic_symbol.clone(),
                nature: *nature,
            };
            index.register_type(typesystem::DataType {
                name: name.to_string(),
                initial_value: None,
                information,
                nature: TypeNature::Any,
                location: type_declaration.location.clone(),
            });
        }

        _ => { /* unnamed datatypes are ignored */ }
    };
}

/// Internally we create a fat pointer struct for VLAs, which consists of a pointer to the passed array plus
/// its dimensions, such that `ARRAY[*, *, *] OF INT` becomes
/// ```ignore
/// STRUCT
///     ptr       : REF_TO ARRAY[*] OF INT;
///     dimensions: ARRAY[0..2, 0..1] OF DINT;
///                       ^^^^       --> dimension index
///                             ^^^^ --> start- & end-offset
/// END_STRUCT
/// ```
fn visit_variable_length_array(
    bounds: &AstNode,
    referenced_type: &DataTypeDeclaration,
    name: &str,
    index: &mut Index,
    type_declaration: &UserTypeDeclaration,
) {
    let ndims = match bounds.get_stmt() {
        AstStatement::VlaRangeStatement => 1,
        AstStatement::ExpressionList(expressions) => expressions.len(),
        _ => unreachable!("not a bounds statement"),
    };

    let referenced_type = referenced_type.get_name().expect("named datatype").to_string();
    let struct_name = name.to_owned();

    let dummy_array_name = format!("__arr_vla_{ndims}_{referenced_type}").to_lowercase();
    let member_array_name = format!("__ptr_to_{dummy_array_name}");
    let member_dimensions_name = format!("__bounds_{dummy_array_name}");

    // check the index if a dummy-array type matching the given VLA (eg. 1 dimension, type INT) already exists.
    // if we find a type, we can use references to the internal types. otherwise, register the array in the index
    // and declare internal member types.
    let (vla_arr_type_declaration, dim_arr_type_declaration) =
        if index.get_effective_type_by_name(&dummy_array_name).is_ok() {
            (
                DataTypeDeclaration::DataTypeReference {
                    referenced_type: member_array_name,
                    location: SourceLocation::internal(),
                },
                DataTypeDeclaration::DataTypeReference {
                    referenced_type: member_dimensions_name,
                    location: SourceLocation::internal(),
                },
            )
        } else {
            // register dummy array type so it can later be annotated as a type hint
            index.register_type(typesystem::DataType {
                name: dummy_array_name.clone(),
                initial_value: None,
                information: DataTypeInformation::Array {
                    name: dummy_array_name.clone(),
                    inner_type_name: referenced_type.clone(),
                    // dummy dimensions that will never actually be used
                    dimensions: (0..ndims)
                        .map(|_| Dimension {
                            start_offset: TypeSize::Undetermined,
                            end_offset: TypeSize::Undetermined,
                        })
                        .collect::<Vec<_>>(),
                },
                nature: TypeNature::__VLA,
                location: SourceLocation::internal(),
            });

            // define internal vla members
            (
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::PointerType {
                        name: Some(member_array_name),
                        referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                            referenced_type: dummy_array_name,
                            location: SourceLocation::internal(),
                        }),
                        auto_deref: None,
                    },
                    location: SourceLocation::internal(),
                    scope: None,
                },
                DataTypeDeclaration::DataTypeDefinition {
                    data_type: DataType::ArrayType {
                        name: Some(member_dimensions_name),
                        bounds: AstNode::new(
                            AstStatement::ExpressionList(
                                (0..ndims)
                                    .map(|_| {
                                        AstFactory::create_range_statement(
                                            AstNode::new_literal(
                                                AstLiteral::new_integer(0),
                                                0,
                                                SourceLocation::internal(),
                                            ),
                                            AstNode::new_literal(
                                                AstLiteral::new_integer(1),
                                                0,
                                                SourceLocation::internal(),
                                            ),
                                            0,
                                        )
                                    })
                                    .collect::<_>(),
                            ),
                            0,
                            SourceLocation::internal(),
                        ),
                        referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                            referenced_type: DINT_TYPE.to_string(),
                            location: SourceLocation::internal(),
                        }),
                        is_variable_length: false,
                    },
                    location: SourceLocation::internal(),
                    scope: None,
                },
            )
        };

    // Create variable index entries for VLA struct members
    let variables = vec![
        // Pointer
        Variable {
            name: format!("struct_vla_{referenced_type}_{ndims}").to_lowercase(),
            data_type_declaration: vla_arr_type_declaration,
            initializer: None,
            address: None,
            location: SourceLocation::internal(),
        },
        // Dimensions Array
        Variable {
            name: "dimensions".to_string(),
            data_type_declaration: dim_arr_type_declaration,
            initializer: None,
            address: None,
            location: SourceLocation::internal(),
        },
    ];

    let struct_ty = DataType::StructType { name: Some(struct_name.clone()), variables: variables.clone() };
    let type_dec = UserTypeDeclaration {
        data_type: struct_ty,
        initializer: None,
        location: type_declaration.location.clone(),
        scope: type_declaration.scope.clone(),
    };

    // visit the internally created struct type to also index its members
    visit_struct(
        &struct_name,
        &variables,
        index,
        &type_declaration.scope,
        &type_dec,
        StructSource::Internal(InternalType::VariableLengthArray { inner_type_name: referenced_type, ndims }),
    )
}

fn visit_array(
    bounds: &AstNode,
    index: &mut Index,
    scope: &Option<String>,
    referenced_type: &DataTypeDeclaration,
    name: &String,
    type_declaration: &UserTypeDeclaration,
) {
    let dimensions: Result<Vec<Dimension>, Diagnostic> = bounds
        .get_as_list()
        .iter()
        .map(|it| match it.get_stmt() {
            AstStatement::RangeStatement(RangeStatement { start, end }) => {
                let constants = index.get_mut_const_expressions();
                Ok(Dimension {
                    start_offset: TypeSize::from_expression(constants.add_constant_expression(
                        *start.clone(),
                        typesystem::DINT_TYPE.to_string(),
                        scope.clone(),
                        None,
                    )),
                    end_offset: TypeSize::from_expression(constants.add_constant_expression(
                        *end.clone(),
                        typesystem::DINT_TYPE.to_string(),
                        scope.clone(),
                        None,
                    )),
                })
            }

            _ => Err(Diagnostic::codegen_error(
                "Invalid array definition: RangeStatement expected",
                it.get_location(),
            )),
        })
        .collect();

    // TODO(mhasel, volsa): This unwrap will panic with `ARRAY[0..5, 5] OF DINT;`
    let dimensions = dimensions.unwrap();

    //TODO hmm we need to talk about all this unwrapping :-/
    let referenced_type_name = referenced_type.get_name().expect("named datatype");
    let information = DataTypeInformation::Array {
        name: name.clone(),
        inner_type_name: referenced_type_name.to_string(),
        dimensions,
    };

    let init1 = index.get_mut_const_expressions().maybe_add_constant_expression(
        type_declaration.initializer.clone(),
        name,
        scope.clone(),
        None,
    );
    // TODO unfortunately we cannot share const-expressions between multiple
    // index-entries
    let init2 = index.get_mut_const_expressions().maybe_add_constant_expression(
        type_declaration.initializer.clone(),
        name,
        scope.clone(),
        None,
    );

    index.register_type(typesystem::DataType {
        name: name.to_string(),
        initial_value: init1,
        information,
        nature: TypeNature::Any,
        location: type_declaration.location.clone(),
    });
    let global_init_name = crate::index::get_initializer_name(name);
    if init2.is_some() {
        let variable = VariableIndexEntry::create_global(
            global_init_name.as_str(),
            global_init_name.as_str(),
            name,
            type_declaration.location.clone(),
        )
        .set_constant(true)
        .set_initial_value(init2);
        index.register_global_initializer(&global_init_name, variable);
    }
}

fn visit_struct(
    name: &str,
    variables: &[Variable],
    index: &mut Index,
    scope: &Option<String>,
    type_declaration: &UserTypeDeclaration,
    source: StructSource,
) {
    let members = variables
        .iter()
        .enumerate()
        .map(|(count, var)| {
            if let DataTypeDeclaration::DataTypeDefinition { data_type, scope, .. } =
                &var.data_type_declaration
            {
                //first we need to handle the inner type
                visit_data_type(
                    index,
                    &UserTypeDeclaration {
                        data_type: data_type.clone(),
                        initializer: None,
                        location: SourceLocation::internal(),
                        scope: scope.clone(),
                    },
                )
            }

            let member_type = var.data_type_declaration.get_name().expect("named variable datatype");
            let init = index.get_mut_const_expressions().maybe_add_constant_expression(
                var.initializer.clone(),
                member_type,
                scope.clone(),
                None,
            );

            let binding =
                var.address.as_ref().and_then(|it| HardwareBinding::from_statement(index, it, scope.clone()));

            index.register_member_variable(
                MemberInfo {
                    container_name: name,
                    variable_name: &var.name,
                    variable_linkage: ArgumentType::ByVal(VariableType::Input), // struct members act like VAR_INPUT in terms of visibility
                    variable_type_name: member_type,
                    is_constant: false, //struct members are not constants //TODO thats probably not true (you can define a struct in an CONST-block?!)
                    is_var_external: false, // structs cannot have VAR_EXTERNAL blocks
                    binding,
                    varargs: None,
                },
                init,
                var.location.clone(),
                count as u32,
            )
        })
        .collect::<Vec<_>>();

    let nature = source.get_type_nature();
    let information = DataTypeInformation::Struct { name: name.to_owned(), members, source };

    let init = index.get_mut_const_expressions().maybe_add_constant_expression(
        type_declaration.initializer.clone(),
        name,
        scope.clone(),
        Some(name.into()),
    );
    index.register_type(typesystem::DataType {
        name: name.to_string(),
        initial_value: init,
        information,
        nature,
        location: type_declaration.location.clone(),
    });
    //Generate an initializer for the struct
    let global_struct_name = crate::index::get_initializer_name(name);
    let variable = VariableIndexEntry::create_global(
        &global_struct_name,
        &global_struct_name,
        name,
        type_declaration.location.clone(),
    )
    .set_initial_value(init)
    .set_constant(true);
    index.register_global_initializer(&global_struct_name, variable);
}
