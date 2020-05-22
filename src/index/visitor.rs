
use super::Index;
use super::VariableType;
use super::super::ast::{ POU, PouType, CompilationUnit, VariableBlock, VariableBlockType, DataType, DataType::DataTypeReference};

pub fn visit(index: &mut Index, unit: &CompilationUnit) {
    for global_vars in &unit.global_vars {
        visit_global_var_block(index, global_vars);
    }

    for pou in &unit.units {
        visit_pou(index, pou);
    }
}

fn visit_pou(index: &mut Index, pou: &POU){

    index.register_type(pou.name.as_str().to_string());

    if pou.pou_type == PouType::Program {
        //Associate a global variable for the program 
        index.register_global_variable(pou.name.clone(), pou.name.clone()); 
    }

    let mut count = 0;
    for block in &pou.variable_blocks {
        let block_type = get_variable_type_from_block(block);
        for var in &block.variables {
            index.register_local_variable(
                pou.name.clone(), 
                var.name.clone(), 
                block_type,
                get_type_name(&var.data_type).to_string(), 
                count,
            );
            count = count + 1;
        }
    }

    if let Some(return_type) = &pou.return_type {
        index.register_local_variable(
            pou.name.clone(), 
            pou.name.clone(), 
            VariableType::Return, 
            get_type_name(return_type).to_string(),
            count)
    }

}


fn visit_global_var_block(index :&mut Index, block: &VariableBlock) {
    for var in &block.variables {
        index.register_global_variable(
                            var.name.clone(), 
                            get_type_name(&var.data_type).to_string()
                        );
    }
}

fn get_type_name(data_type: &DataType) -> &str {
    match data_type{
        DataTypeReference { type_name } => type_name,
        _ => &""
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    match block.variable_block_type {
        VariableBlockType::Local => VariableType::Local,
        VariableBlockType::Input => VariableType::Input,
        VariableBlockType::Global => VariableType::Global,
    }
}