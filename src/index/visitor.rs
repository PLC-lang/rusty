/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::ast::Implementation;
use crate::ast::{self, UserTypeDeclaration};
use super::Index;
use super::VariableType;
use crate::typesystem::*;
use super::super::ast::{ POU, PouType, CompilationUnit, VariableBlock, VariableBlockType, DataType, DataTypeDeclaration, get_array_dimensions, evaluate_constant_int};

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

pub fn visit_pou(index: &mut Index, pou: &POU){

    let interface_name = format!("{}_interface", &pou.name);

    if pou.pou_type == PouType::Program {
        //Associate a global variable for the program 
    let instance_name = format!("{}_instance", &pou.name);
        index.register_global_variable_with_name(&pou.name, &instance_name, &pou.name, None, pou.location.clone()); 
    }

    let mut member_names = vec![];

    let mut count = 0;
    for block in &pou.variable_blocks {
        let block_type = get_variable_type_from_block(block);
        for var in &block.variables {
            member_names.push(var.name.clone());
            index.register_member_variable(
                &pou.name, 
                &var.name, 
                block_type,
                var.data_type.get_name().unwrap(), 
                var.initializer.clone(),
                var.location.clone(),
                count,
            );
            count = count + 1;
        }
    }

    if let Some(return_type) = &pou.return_type {
        member_names.push(pou.name.clone());
        let source_location = pou.location.end .. pou.location.end;
        index.register_member_variable(
            &pou.name, 
            &pou.name, 
            VariableType::Return, 
            return_type.get_name().unwrap().into(), 
            None,
            source_location,
            count)
    }

    index.register_type(&pou.name, None, DataTypeInformation::Struct {
        name : interface_name, 
        member_names,
    });

}

fn visit_implementation(index :&mut Index, implementation : &Implementation) {
    index.register_implementation(&implementation.name, &implementation.type_name);
    //if we are registing an action, also register a datatype for it
    if implementation.pou_type == PouType::Action {
        index.register_type(&implementation.name, None, 
            DataTypeInformation::Alias {
                name : implementation.name.clone(),
                referenced_type : implementation.type_name.clone(),
            }
        );
    }
}


fn visit_global_var_block(index :&mut Index, block: &VariableBlock) {
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
    }
}


fn visit_data_type(index: &mut Index, type_declatation: &UserTypeDeclaration) {
    let data_type = &type_declatation.data_type;
    //names should not be empty
    match data_type {
        DataType::StructType { name, variables } => 
        {
            let struct_name = name.as_ref().unwrap(); 

            let member_names :Vec<String> = variables.iter().map(|it| it.name.to_string()).collect();
            
            let information = DataTypeInformation::Struct {
                        name: name.clone().unwrap(),
                        member_names
                    };
            index.register_type(name.as_ref().unwrap(), type_declatation.initializer.clone(), information);
            let mut count = 0;
            for var in variables {

                if let DataTypeDeclaration::DataTypeDefinition{data_type} = &var.data_type {
                    //first we need to handle the inner type
                    visit_data_type(index, &UserTypeDeclaration{data_type : data_type.clone(), initializer : None})
                }

                index.register_member_variable(
                    &struct_name, 
                    &var.name,
                    VariableType::Local,
                    var.data_type.get_name().unwrap(), 
                    var.initializer.clone(),
                    var.location.clone(),
                    count,
                );
                count = count + 1;
            }

        },

        DataType::EnumType { name, elements } =>  {
                let information = DataTypeInformation::Integer {
                    name: "DINT".into(),
                    signed: true,
                    size: 32,
                };
            index.register_type( name.as_ref().unwrap(), type_declatation.initializer.clone(), information);
            elements.iter().enumerate().for_each(|(i,v)| index.register_global_variable(v, "DINT", Some(ast::Statement::LiteralInteger{value:i.to_string(), location: 0..0}), 0..0)); //TODO : Enum locations
        },

        DataType::SubRangeType { name, referenced_type,  .. } => {
                let information = DataTypeInformation::Alias {
                    name: name.as_ref().unwrap().into(),
                    referenced_type : referenced_type.into(),
                };
            index.register_type (name.as_ref().unwrap(),type_declatation.initializer.clone(), information)
        },
        DataType::ArrayType { name, referenced_type, bounds } => {
                let dimensions = get_array_dimensions(&bounds).unwrap();
                let referenced_type_name = referenced_type.get_name().unwrap();
                let information = DataTypeInformation::Array {
                        name : name.as_ref().unwrap().clone(),
                        inner_type_name: referenced_type_name.to_string(),
                        dimensions,

                };
            index.register_type (name.as_ref().unwrap(),type_declatation.initializer.clone(), information)
        },
        DataType::StringType { name, size, ..} => {

                let size = if let Some(statement) = size {
                    evaluate_constant_int(&statement).unwrap() as u32
                } else {
                    crate::typesystem::DEFAULT_STRING_LEN  // DEFAULT STRING LEN
                } + 1;
                let information = DataTypeInformation::String {
                    size
                };
                index.register_type (name.as_ref().unwrap(),type_declatation.initializer.clone(), information)
        },
                
    };
}
