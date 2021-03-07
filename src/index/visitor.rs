/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::Index;
use super::VariableType;
use super::super::ast::{ POU, PouType, CompilationUnit, VariableBlock, VariableBlockType, DataType, DataTypeDeclaration };

pub fn visit(index: &mut Index, unit: &mut CompilationUnit) {
    for user_type in &unit.types {
        visit_data_type(index, &user_type.data_type);
    }

    for global_vars in &unit.global_vars {
        visit_global_var_block(index, global_vars);
    }

    for pou in &unit.units {
        visit_pou(index, pou);
    }

}

pub fn visit_pou(index: &mut Index, pou: &POU){

    index.register_type(pou.name.as_str().to_string());

    if pou.pou_type == PouType::Program {
        //Associate a global variable for the program 
        index.register_global_variable(pou.name.clone(), pou.name.clone()); 
    }

    let mut count = 0;
    for block in &pou.variable_blocks {
        let block_type = get_variable_type_from_block(block);
        for var in &block.variables {
            index.register_member_variable(
                pou.name.clone(), 
                var.name.clone(), 
                block_type,
                var.data_type.get_name().unwrap().to_string(), 
                count,
            );
            count = count + 1;
        }
    }

    if let Some(return_type) = &pou.return_type {
        index.register_member_variable(
            pou.name.clone(), 
            pou.name.clone(), 
            VariableType::Return, 
            return_type.get_name().unwrap().to_string(), 
            count)
    }

}


fn visit_global_var_block(index :&mut Index, block: &VariableBlock) {
    for var in &block.variables {

        index.register_global_variable(
                            var.name.clone(),
                            var.data_type.get_name().unwrap().to_string()
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


fn visit_data_type(index: &mut Index, data_type: &DataType) {
    //names should not be empty
    match data_type {
        DataType::StructType { name, variables } => 
        {
            let struct_name = name.as_ref().unwrap();
            index.register_type(name.as_ref().map(|it| it.to_string()).unwrap());
            let mut count = 0;
            for var in variables {

                if let DataTypeDeclaration::DataTypeDefinition{ data_type} = &var.data_type {
                    //first we need to handle the inner type
                    visit_data_type(index, &data_type)
                }

                index.register_member_variable(
                    struct_name.clone(), 
                    var.name.clone(), 
                    VariableType::Local,
                    var.data_type.get_name().unwrap().to_string(), 
                    count,
                );
                count = count + 1;
            }

        },

        DataType::EnumType { name, elements, .. } =>  {
            index.register_type( name.as_ref().map(|it| it.to_string()).unwrap());
            elements.iter().for_each(|v| index.register_global_variable(v.to_string(), "INT".to_string()));
        },

        DataType::SubRangeType { name, .. } => 
            index.register_type (name.as_ref().map(|it| it.to_string()).unwrap()),
        DataType::ArrayType { name, .. } => 
            index.register_type (name.as_ref().map(|it| it.to_string()).unwrap()),
    }
}
