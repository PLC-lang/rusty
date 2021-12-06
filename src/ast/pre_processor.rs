// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::{ast::DataTypeDeclaration, lexer::IdProvider};

use super::{
    super::ast::{CompilationUnit, UserTypeDeclaration, Variable},
    flatten_expression_list, AstStatement, DataType, Operator, Pou, SourceRange,
};
use std::{collections::HashMap, vec};

pub fn pre_process(unit: &mut CompilationUnit, mut id_provider: IdProvider) {
    //process all local variables from POUs
    for mut pou in unit.units.iter_mut() {
        //Find all generic types in that pou
        let generic_types = preprocess_generic_structs(&mut pou);
        unit.types.extend(generic_types);

        let all_variables = pou
            .variable_blocks
            .iter_mut()
            .flat_map(|it| it.variables.iter_mut())
            .filter(|it| should_generate_implicit_type(it));

        for var in all_variables {
            pre_process_variable_data_type(pou.name.as_str(), var, &mut unit.types)
        }

        //Generate implicit type for returns
        preprocess_return_type(&mut pou, &mut unit.types);
    }

    //process all variables from GVLs
    let all_variables = unit
        .global_vars
        .iter_mut()
        .flat_map(|gv| gv.variables.iter_mut())
        .filter(|it| should_generate_implicit_type(it));

    for var in all_variables {
        pre_process_variable_data_type("global", var, &mut unit.types)
    }

    //process all variables in dataTypes
    let mut new_types = vec![];
    for dt in unit.types.iter_mut() {
        {
            match &mut dt.data_type {
                DataType::StructType {
                    name, variables, ..
                } => {
                    let name: &str = name.as_ref().map(|it| it.as_str()).unwrap_or("undefined");
                    variables
                        .iter_mut()
                        .filter(|it| should_generate_implicit_type(it))
                        .for_each(|var| pre_process_variable_data_type(name, var, &mut new_types));
                }
                DataType::ArrayType {
                    name,
                    referenced_type,
                    ..
                }
                | DataType::PointerType {
                    name,
                    referenced_type,
                    ..
                } if should_generate_implicit(referenced_type) => {
                    let name: &str = name.as_ref().map(|it| it.as_str()).unwrap_or("undefined");

                    let type_name = format!("__{}", name);
                    let type_ref = DataTypeDeclaration::DataTypeReference {
                        referenced_type: type_name.clone(),
                        location: SourceRange::undefined(), //return_type.get_location(),
                    };
                    let datatype = std::mem::replace(referenced_type, Box::new(type_ref));
                    if let DataTypeDeclaration::DataTypeDefinition {
                        mut data_type,
                        location,
                        scope,
                    } = *datatype
                    {
                        data_type.set_name(type_name);
                        add_nested_datatypes(name, &mut data_type, &mut new_types, &location);
                        let data_type = UserTypeDeclaration {
                            data_type,
                            initializer: None,
                            location,
                            scope,
                        };
                        new_types.push(data_type);
                    }
                }
                DataType::EnumType { elements, .. }
                    if matches!(elements, AstStatement::EmptyStatement { .. }) =>
                {
                    //avoid empty statements, just use an empty expression list to make it easier to work with
                    let _ = std::mem::replace(
                        elements,
                        AstStatement::ExpressionList {
                            expressions: vec![],
                            id: id_provider.next_id(),
                        },
                    );
                }
                DataType::EnumType {
                    elements: original_elements,
                    name: Some(enum_name),
                    ..
                } if !matches!(original_elements, AstStatement::EmptyStatement { .. }) => {
                    

                    let mut last_name: Option<String> = None;
                    let elements = flatten_expression_list(original_elements)
                        .iter()
                        .map(|it| match it {
                            AstStatement::Reference { name, location, .. } => {
                                (name.clone(), None, location.clone())
                            }
                            AstStatement::Assignment { left, right, .. } => {
                                let name =
                                    if let AstStatement::Reference { name, .. } = left.as_ref() {
                                        name.clone()
                                    } else {
                                        "<undefined>".to_string()
                                    };
                                (name, Some(*right.clone()), it.get_location())
                            }
                            _ => ("<undefined>".to_string(), None, SourceRange::undefined()),
                        })
                        .map(|(name, v, location)| {
                            let enum_literal = v.unwrap_or_else(|| {
                                if let Some(last_element) = last_name.as_ref() {
                                    // generate a `enum#last + 1` statement
                                    AstStatement::BinaryExpression {
                                        id: id_provider.next_id(),
                                        operator: Operator::Plus,
                                        left: Box::new(AstStatement::CastStatement {
                                            target: Box::new(AstStatement::Reference {
                                                id: id_provider.next_id(),
                                                location: location.clone(),
                                                name: last_element.clone(),
                                            }),
                                            id: id_provider.next_id(),
                                            location: location.clone(),
                                            type_name: enum_name.to_string(),
                                        }),
                                        right: Box::new(AstStatement::LiteralInteger {
                                            value: 1,
                                            location: location.clone(),
                                            id: id_provider.next_id(),
                                        }),
                                    }
                                } else {
                                    AstStatement::LiteralInteger {
                                        value: 0,
                                        location: location.clone(),
                                        id: id_provider.next_id(),
                                    }
                                }
                            });
                            last_name = Some(name.clone());
                            AstStatement::Assignment {
                                id: id_provider.next_id(),
                                left: Box::new(AstStatement::Reference {
                                    id: id_provider.next_id(),
                                    name,
                                    location,
                                }),
                                right: Box::new(enum_literal),
                            }
                        })
                        .collect::<Vec<AstStatement>>();
                    if !elements.is_empty() {
                        let expression = AstStatement::ExpressionList {
                            expressions: elements,
                            id: id_provider.next_id(),
                        };
                        let _ = std::mem::replace(original_elements, expression);
                    }
                }

                _ => {}
            }
        }
    }
    unit.types.append(&mut new_types);
}

fn preprocess_generic_structs(pou: &mut Pou) -> Vec<UserTypeDeclaration> {
    let mut generic_types = HashMap::new();
    let mut types = vec![];
    for binding in &pou.generics {
        let new_name = format!("__{}__{}", pou.name, binding.name);
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
    for var in pou
        .variable_blocks
        .iter_mut()
        .flat_map(|it| it.variables.iter_mut())
    {
        replace_generic_type_name(&mut var.data_type, &generic_types);
    }
    if let Some(datatype) = pou.return_type.as_mut() {
        replace_generic_type_name(datatype, &generic_types);
    };
    types
}

fn preprocess_return_type(pou: &mut Pou, types: &mut Vec<UserTypeDeclaration>) {
    if let Some(return_type) = &pou.return_type {
        if should_generate_implicit(return_type) {
            let type_name = format!("__{}_return", &pou.name);
            let type_ref = DataTypeDeclaration::DataTypeReference {
                referenced_type: type_name.clone(),
                location: return_type.get_location(),
            };
            let datatype = std::mem::replace(&mut pou.return_type, Some(type_ref));
            if let Some(DataTypeDeclaration::DataTypeDefinition {
                mut data_type,
                location,
                scope,
            }) = datatype
            {
                data_type.set_name(type_name);
                add_nested_datatypes(pou.name.as_str(), &mut data_type, types, &location);
                let data_type = UserTypeDeclaration {
                    data_type,
                    initializer: None,
                    location,
                    scope,
                };
                types.push(data_type);
            }
        }
    }
}

fn should_generate_implicit(datatype: &DataTypeDeclaration) -> bool {
    match datatype {
        DataTypeDeclaration::DataTypeReference { .. } => false,
        DataTypeDeclaration::DataTypeDefinition {
            data_type: DataType::VarArgs { .. },
            ..
        } => false,
        DataTypeDeclaration::DataTypeDefinition { .. } => true,
    }
}

fn should_generate_implicit_type(variable: &Variable) -> bool {
    should_generate_implicit(&variable.data_type)
}

fn pre_process_variable_data_type(
    container_name: &str,
    variable: &mut Variable,
    types: &mut Vec<UserTypeDeclaration>,
) {
    let new_type_name = format!("__{}_{}", container_name, variable.name);
    if let DataTypeDeclaration::DataTypeDefinition {
        mut data_type,
        location,
        scope,
    } = variable.replace_data_type_with_reference_to(new_type_name.clone())
    {
        // create index entry
        add_nested_datatypes(new_type_name.as_str(), &mut data_type, types, &location);
        data_type.set_name(new_type_name);
        types.push(UserTypeDeclaration {
            data_type,
            initializer: None,
            location,
            scope,
        });
    }
    //make sure it gets generated
}

fn add_nested_datatypes(
    container_name: &str,
    datatype: &mut DataType,
    types: &mut Vec<UserTypeDeclaration>,
    location: &SourceRange,
) {
    let new_type_name = format!("{}_", container_name);
    if let Some(DataTypeDeclaration::DataTypeDefinition {
        mut data_type,
        location: inner_location,
        scope,
    }) = datatype.replace_data_type_with_reference_to(new_type_name.clone(), location)
    {
        data_type.set_name(new_type_name.clone());
        add_nested_datatypes(
            new_type_name.as_str(),
            &mut data_type,
            types,
            &inner_location,
        );
        types.push(UserTypeDeclaration {
            data_type,
            initializer: None,
            location: location.clone(),
            scope,
        });
    }
}

fn replace_generic_type_name(dt: &mut DataTypeDeclaration, generics: &HashMap<String, String>) {
    match dt {
        DataTypeDeclaration::DataTypeDefinition { data_type, .. } => match data_type {
            DataType::ArrayType {
                referenced_type, ..
            }
            | DataType::PointerType {
                referenced_type, ..
            } => replace_generic_type_name(referenced_type.as_mut(), generics),
            _ => {}
        },
        DataTypeDeclaration::DataTypeReference {
            referenced_type, ..
        } => {
            if let Some(type_name) = generics.get(referenced_type) {
                *referenced_type = type_name.clone();
            }
        }
    }
}
