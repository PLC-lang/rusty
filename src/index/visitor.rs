// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::VariableType;
use crate::ast::{
    self, AstStatement, CompilationUnit, DataType, DataTypeDeclaration, Implementation, Pou,
    PouType, SourceRange, UserTypeDeclaration, VariableBlock, VariableBlockType,
};
use crate::compile_error::CompileError;
use crate::index::{Index, MemberInfo};
use crate::lexer::IdProvider;
use crate::typesystem::{self, *};

pub fn visit(unit: &CompilationUnit, mut id_provider: IdProvider) -> Index {
    let mut index = Index::new();

    //Create the typesystem
    let builtins = get_builtin_types();
    for data_type in builtins {
        index.register_type(
            data_type.get_name(),
            data_type.initial_value,
            data_type.clone_type_information(),
        );
    }

    //Create user defined datatypes
    for user_type in &unit.types {
        visit_data_type(&mut index, &mut id_provider, user_type);
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
    let interface_name = format!("{}_interface", &pou.name);

    if pou.pou_type == PouType::Program {
        //Associate a global variable for the program
        let instance_name = format!("{}_instance", &pou.name);
        index.register_global_variable_with_name(
            &pou.name,
            &instance_name,
            &pou.name,
            None,
            false, //program's instance variable is no constant
            pou.location.clone(),
        );
    }

    let mut member_names = vec![];

    //register the pou's member variables
    let mut count = 0;
    let mut varargs = None;
    for block in &pou.variable_blocks {
        let block_type = get_variable_type_from_block(block);
        for var in &block.variables {
            if let DataTypeDeclaration::DataTypeDefinition {
                data_type: ast::DataType::VarArgs { referenced_type },
                ..
            } = &var.data_type
            {
                let name = referenced_type
                    .as_ref()
                    .map(|it| &**it)
                    .map(DataTypeDeclaration::get_name)
                    .flatten()
                    .map(|it| it.to_string());
                varargs = Some(name);
                continue;
            }
            member_names.push(var.name.clone());

            let var_type_name = var.data_type.get_name().expect("named datatype");
            let type_name = if block_type == VariableType::InOut {
                //register a pointer type for the var_in_out
                register_inout_pointer_type_for(index, var_type_name)
            } else {
                var_type_name.to_string()
            };
            let initial_value = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    var.initializer.clone(),
                    type_name.as_str(),
                    Some(pou.name.clone()),
                );

            index.register_member_variable(
                &MemberInfo {
                    container_name: &pou.name,
                    variable_name: &var.name,
                    variable_linkage: block_type,
                    variable_type_name: &type_name,
                    is_constant: block.constant,
                },
                initial_value,
                var.location.clone(),
                count,
            );
            count += 1;
        }
    }

    //register a function's return type as a member variable
    if let Some(return_type) = &pou.return_type {
        member_names.push(pou.get_return_name().into());
        let source_location = SourceRange::new(pou.location.get_end()..pou.location.get_end());
        index.register_member_variable(
            &MemberInfo {
                container_name: &pou.name,
                variable_name: pou.get_return_name(),
                variable_linkage: VariableType::Return,
                variable_type_name: return_type.get_name().unwrap_or_default(),
                is_constant: false, //return variables are not constants
            },
            None,
            source_location,
            count,
        )
    }

    index.register_type(
        &pou.name,
        None,
        DataTypeInformation::Struct {
            name: interface_name,
            member_names,
            varargs,
            source: StructSource::Pou(pou.pou_type.clone()),
            generics: pou.generics.clone(),
        },
    );
}

fn visit_implementation(index: &mut Index, implementation: &Implementation) {
    let pou_type = &implementation.pou_type;
    index.register_implementation(
        &implementation.name,
        &implementation.type_name,
        pou_type.get_optional_owner_class().as_ref(),
        pou_type.into(),
    );
    //if we are registing an action, also register a datatype for it
    if pou_type == &PouType::Action {
        index.register_type(
            &implementation.name,
            None,
            DataTypeInformation::Alias {
                name: implementation.name.clone(),
                referenced_type: implementation.type_name.clone(),
            },
        );
    }
}

fn register_inout_pointer_type_for(index: &mut Index, inner_type_name: &str) -> String {
    //get unique name
    let type_name = format!("pointer_to_{}", inner_type_name);

    //generate a pointertype for the variable
    index.register_type(
        &type_name,
        None,
        DataTypeInformation::Pointer {
            name: type_name.clone(),
            inner_type_name: inner_type_name.to_string(),
            auto_deref: true,
        },
    );

    type_name
}

fn visit_global_var_block(index: &mut Index, block: &VariableBlock) {
    for var in &block.variables {
        let target_type = var.data_type.get_name().unwrap_or_default();
        let initializer = index
            .get_mut_const_expressions()
            .maybe_add_constant_expression(var.initializer.clone(), target_type, None);
        index.register_global_variable(
            &var.name,
            var.data_type.get_name().expect("named variable datatype"),
            initializer,
            block.constant,
            var.location.clone(),
        );
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    match block.variable_block_type {
        VariableBlockType::Local => VariableType::Local,
        VariableBlockType::Temp => VariableType::Temp,
        VariableBlockType::Input => VariableType::Input,
        VariableBlockType::Output => VariableType::Output,
        VariableBlockType::Global => VariableType::Global,
        VariableBlockType::InOut => VariableType::InOut,
    }
}

fn visit_data_type(
    index: &mut Index,
    id_provider: &mut IdProvider,
    type_declaration: &UserTypeDeclaration,
) {
    let data_type = &type_declaration.data_type;
    let scope = &type_declaration.scope;
    //names should not be empty
    match data_type {
        DataType::StructType {
            name: Some(name),
            variables,
        } => {
            let struct_name = name.as_str();

            let member_names: Vec<String> =
                variables.iter().map(|it| it.name.to_string()).collect();

            let type_name = name.clone();
            let information = DataTypeInformation::Struct {
                name: type_name.clone(),
                member_names,
                varargs: None,
                source: StructSource::OriginalDeclaration,
                generics: vec![],
            };

            let init = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    type_declaration.initializer.clone(),
                    type_name.as_str(),
                    scope.clone(),
                );
            index.register_type(name, init, information);
            for (count, var) in variables.iter().enumerate() {
                if let DataTypeDeclaration::DataTypeDefinition {
                    data_type, scope, ..
                } = &var.data_type
                {
                    //first we need to handle the inner type
                    visit_data_type(
                        index,
                        id_provider,
                        &UserTypeDeclaration {
                            data_type: data_type.clone(),
                            initializer: None,
                            location: SourceRange::undefined(),
                            scope: scope.clone(),
                        },
                    )
                }

                let member_type = var.data_type.get_name().expect("named variable datatype");
                let init = index
                    .get_mut_const_expressions()
                    .maybe_add_constant_expression(
                        var.initializer.clone(),
                        member_type,
                        scope.clone(),
                    );
                index.register_member_variable(
                    &MemberInfo {
                        container_name: struct_name,
                        variable_name: &var.name,
                        variable_linkage: VariableType::Local,
                        variable_type_name: member_type,
                        is_constant: false, //struct members are not constants //TODO thats probably not true (you can define a struct in an CONST-block?!)
                    },
                    init,
                    var.location.clone(),
                    count as u32,
                );
            }
        }

        DataType::EnumType {
            name: Some(name),
            elements,
        } => {
            let enum_name = name.as_str();
            let information = DataTypeInformation::Enum {
                name: enum_name.to_string(),
                elements: elements.clone(),
            };

            let init = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    type_declaration.initializer.clone(),
                    enum_name,
                    scope.clone(),
                );
            index.register_type(enum_name, init, information);

            elements.iter().enumerate().for_each(|(i, v)| {
                let enum_literal = ast::AstStatement::LiteralInteger {
                    value: i as i128,
                    location: SourceRange::undefined(),
                    id: id_provider.next_id(),
                };
                let init = index.get_mut_const_expressions().add_constant_expression(
                    enum_literal,
                    typesystem::INT_TYPE.to_string(),
                    scope.clone(),
                );

                index.register_enum_element(v, enum_name, Some(init), SourceRange::undefined())
            }); //TODO : Enum locations
        }

        DataType::SubRangeType {
            name: Some(name),
            referenced_type,
            bounds,
        } => {
            let information = if let Some(AstStatement::RangeStatement { start, end, .. }) = bounds
            {
                DataTypeInformation::SubRange {
                    name: name.into(),
                    referenced_type: referenced_type.into(),
                    sub_range: (*start.clone()..*end.clone()),
                }
            } else {
                DataTypeInformation::Alias {
                    name: name.into(),
                    referenced_type: referenced_type.into(),
                }
            };

            let init = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    type_declaration.initializer.clone(),
                    name,
                    scope.clone(),
                );
            index.register_type(name, init, information)
        }
        DataType::ArrayType {
            name: Some(name),
            referenced_type,
            bounds,
        } => {
            let dimensions: Result<Vec<Dimension>, CompileError> = bounds
                .get_as_list()
                .iter()
                .map(|it| {
                    if let AstStatement::RangeStatement { start, end, .. } = it {
                        let constants = index.get_mut_const_expressions();
                        Ok(Dimension {
                            start_offset: TypeSize::from_expression(
                                constants.add_constant_expression(
                                    *start.clone(),
                                    typesystem::INT_TYPE.to_string(),
                                    scope.clone(),
                                ),
                            ),
                            end_offset: TypeSize::from_expression(
                                constants.add_constant_expression(
                                    *end.clone(),
                                    typesystem::INT_TYPE.to_string(),
                                    scope.clone(),
                                ),
                            ),
                        })
                    } else {
                        Err(CompileError::codegen_error(
                            "Invalid array definition: RangeStatement expected".into(),
                            it.get_location(),
                        ))
                    }
                })
                .collect();
            let dimensions = dimensions.unwrap(); //TODO hmm we need to talk about all this unwrapping :-/
            let referenced_type_name = referenced_type.get_name().expect("named datatype");
            let information = DataTypeInformation::Array {
                name: name.clone(),
                inner_type_name: referenced_type_name.to_string(),
                dimensions,
            };

            let init = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    type_declaration.initializer.clone(),
                    name,
                    scope.clone(),
                );
            index.register_type(name, init, information)
        }
        DataType::PointerType {
            name: Some(name),
            referenced_type,
            ..
        } => {
            let inner_type_name = referenced_type.get_name().expect("named datatype");
            let information = DataTypeInformation::Pointer {
                name: name.clone(),
                inner_type_name: inner_type_name.into(),
                auto_deref: false,
            };

            let init = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    type_declaration.initializer.clone(),
                    name,
                    scope.clone(),
                );
            index.register_type(name, init, information)
        }
        DataType::StringType {
            name: Some(name),
            size,
            is_wide,
            ..
        } => {
            let type_name = name;
            let encoding = if *is_wide {
                StringEncoding::Utf16
            } else {
                StringEncoding::Utf8
            };

            let size = match size {
                Some(AstStatement::LiteralInteger { value, .. }) => {
                    TypeSize::from_literal((value + 1) as u32)
                }
                Some(statement) => {
                    // construct a "x + 1" expression because we need one additional character for \0 terminator
                    let len_plus_1 = AstStatement::BinaryExpression {
                        id: statement.get_id(),
                        left: Box::new(statement.clone()),
                        operator: ast::Operator::Plus,
                        right: Box::new(AstStatement::LiteralInteger {
                            id: statement.get_id(),
                            location: statement.get_location(),
                            value: 1,
                        }),
                    };

                    TypeSize::from_expression(
                        index.get_mut_const_expressions().add_constant_expression(
                            len_plus_1,
                            type_name.clone(),
                            scope.clone(),
                        ),
                    )
                }
                None => TypeSize::from_literal(DEFAULT_STRING_LEN + 1),
            };
            let information = DataTypeInformation::String { size, encoding };
            let init = index
                .get_mut_const_expressions()
                .maybe_add_constant_expression(
                    type_declaration.initializer.clone(),
                    type_name,
                    scope.clone(),
                );
            index.register_type(name, init, information)
        }
        DataType::VarArgs { .. } => {} //Varargs are not indexed
        _ => { /* unnamed datatypes are ignored */ }
    };
}
