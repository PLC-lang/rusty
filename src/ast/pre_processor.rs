// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::ast::DataTypeDeclaration;

use super::{
    super::ast::{CompilationUnit, DataType, UserTypeDeclaration, Variable},
    Pou, SourceRange,
};
use std::{collections::HashMap, vec};

pub fn pre_process(unit: &mut CompilationUnit) {
    //process all local variables from POUs
    for mut pou in unit.units.iter_mut() {
        //Find all generic types in that pou
        let mut generic_types = HashMap::new();
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
            unit.types.push(data_type);
            generic_types.insert(binding.name.clone(), new_name);
        }
        //Find all variables that reference a generic type
        //Replace the reference with the generic type's name
        for var in pou
            .variable_blocks
            .iter_mut()
            .flat_map(|it| it.variables.iter_mut())
        {
            replace_generic_type_name(&mut var.data_type, &generic_types);
        }
        //Replace the return type's reference if needed
        if let Some(datatype) = pou.return_type.as_mut() {
            replace_generic_type_name(datatype, &generic_types);
        }

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
                _ => {}
            }
        }
    }
    unit.types.append(&mut new_types);
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
