// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::VariableType;
use crate::ast::{
    self, evaluate_constant_int, get_array_dimensions, CompilationUnit, DataType,
    DataTypeDeclaration, Implementation, Pou, PouType, Statement, UserTypeDeclaration,
    VariableBlock, VariableBlockType,
};
use crate::index::{Index, MemberInfo};
use crate::typesystem::*;

pub fn visit(unit: &CompilationUnit) -> Index {
    let mut index = Index::new();

    //Create the typesystem
    let builtins = get_builtin_types();
    for data_type in builtins {
        index.types.insert(data_type.get_name().into(), data_type);
    }

    //Create user defined datatypes
    for user_type in &unit.types {
        visit_data_type(&mut index, &user_type);
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
            pou.location.clone(),
        );
    }

    let mut member_names = vec![];

    let mut count = 0;
    for block in &pou.variable_blocks {
        let block_type = get_variable_type_from_block(block);
        for var in &block.variables {
            member_names.push(var.name.clone());
            index.register_member_variable(
                &MemberInfo {
                    container_name: &pou.name,
                    variable_name: &var.name,
                    variable_linkage: block_type,
                    variable_type_name: var.data_type.get_name().unwrap(),
                },
                var.initializer.clone(),
                var.location.clone(),
                count,
            );
            count += 1;
        }
    }

    if let Some(return_type) = &pou.return_type {
        member_names.push(pou.name.clone());
        let source_location = pou.location.end..pou.location.end;
        index.register_member_variable(
            &MemberInfo {
                container_name: &pou.name,
                variable_name: &pou.name,
                variable_linkage: VariableType::Return,
                variable_type_name: return_type.get_name().unwrap(),
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
        },
    );
}

fn visit_implementation(index: &mut Index, implementation: &Implementation) {
    index.register_implementation(&implementation.name, &implementation.type_name);
    //if we are registing an action, also register a datatype for it
    if implementation.pou_type == PouType::Action {
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

fn visit_global_var_block(index: &mut Index, block: &VariableBlock) {
    for var in &block.variables {
        index.register_global_variable(
            &var.name,
            var.data_type.get_name().unwrap(),
            var.initializer.clone(),
            var.location.clone(),
        );
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    match block.variable_block_type {
        VariableBlockType::Local => VariableType::Local,
        VariableBlockType::Input => VariableType::Input,
        VariableBlockType::Output => VariableType::Output,
        VariableBlockType::Global => VariableType::Global,
        VariableBlockType::InOut => VariableType::InOut,
    }
}

fn visit_data_type(index: &mut Index, type_declatation: &UserTypeDeclaration) {
    let data_type = &type_declatation.data_type;
    //names should not be empty
    match data_type {
        DataType::StructType { name, variables } => {
            let struct_name = name.as_ref().unwrap();

            let member_names: Vec<String> =
                variables.iter().map(|it| it.name.to_string()).collect();

            let information = DataTypeInformation::Struct {
                name: name.clone().unwrap(),
                member_names,
            };
            index.register_type(
                name.as_ref().unwrap(),
                type_declatation.initializer.clone(),
                information,
            );
            for (count, var) in variables.iter().enumerate() {
                if let DataTypeDeclaration::DataTypeDefinition { data_type } = &var.data_type {
                    //first we need to handle the inner type
                    visit_data_type(
                        index,
                        &UserTypeDeclaration {
                            data_type: data_type.clone(),
                            initializer: None,
                        },
                    )
                }

                index.register_member_variable(
                    &MemberInfo {
                        container_name: &struct_name,
                        variable_name: &var.name,
                        variable_linkage: VariableType::Local,
                        variable_type_name: var.data_type.get_name().unwrap(),
                    },
                    var.initializer.clone(),
                    var.location.clone(),
                    count as u32,
                );
            }
        }

        DataType::EnumType { name, elements } => {
            let information = DataTypeInformation::Integer {
                name: "DINT".into(),
                signed: true,
                size: 32,
            };
            index.register_type(
                name.as_ref().unwrap(),
                type_declatation.initializer.clone(),
                information,
            );
            elements.iter().enumerate().for_each(|(i, v)| {
                index.register_global_variable(
                    v,
                    "DINT",
                    Some(ast::Statement::LiteralInteger {
                        value: i.to_string(),
                        location: 0..0,
                    }),
                    0..0,
                )
            }); //TODO : Enum locations
        }

        DataType::SubRangeType {
            name,
            referenced_type,
            bounds,
        } => {
            let information = if let Some(Statement::RangeStatement { start, end }) = bounds {
                DataTypeInformation::SubRange {
                    name: name.as_ref().unwrap().into(),
                    referenced_type: referenced_type.into(),
                    sub_range: (*start.clone()..*end.clone()),
                }
            } else {
                DataTypeInformation::Alias {
                    name: name.as_ref().unwrap().into(),
                    referenced_type: referenced_type.into(),
                }
            };
            index.register_type(
                name.as_ref().unwrap(),
                type_declatation.initializer.clone(),
                information,
            )
        }
        DataType::ArrayType {
            name,
            referenced_type,
            bounds,
        } => {
            let dimensions = get_array_dimensions(&bounds).unwrap();
            let referenced_type_name = referenced_type.get_name().unwrap();
            let information = DataTypeInformation::Array {
                name: name.as_ref().unwrap().clone(),
                inner_type_name: referenced_type_name.to_string(),
                dimensions,
            };
            index.register_type(
                name.as_ref().unwrap(),
                type_declatation.initializer.clone(),
                information,
            )
        }
        DataType::StringType { name, size, .. } => {
            let size = if let Some(statement) = size {
                evaluate_constant_int(&statement).unwrap() as u32
            } else {
                crate::typesystem::DEFAULT_STRING_LEN // DEFAULT STRING LEN
            } + 1;
            let information = DataTypeInformation::String { size };
            index.register_type(
                name.as_ref().unwrap(),
                type_declatation.initializer.clone(),
                information,
            )
        }
    };
}
