
use super::Index;
use super::VariableType;
use super::PouKind;
use super::super::ast::{ POU, CompilationUnit, VariableBlock, Type , PouType, VariableBlockType };

pub fn visit(index: &mut Index, unit: &CompilationUnit) {
    for global_vars in &unit.global_vars {
        visit_global_var_block(index, global_vars);
    }

    for pou in &unit.units {
        visit_pou(index, pou);
    }
}

fn visit_pou(index: &mut Index, pou: &POU){
    let pou_type = match pou.pou_type {
        PouType::Program => PouKind::Program,
        PouType::Function => PouKind::Function,
        PouType::FunctionBlock => PouKind::FunctionBlock,
    };

    index.register_pou(pou.name.as_str().to_string(), pou_type);

    for block in &pou.variable_blocks {
        let block_type = get_variable_type_from_block(block);
        for var in &block.variables {
            index.register_local_variable(
                pou.name.clone(), 
                var.name.clone(), 
                block_type, 
                get_type_name(&var.data_type));
        }
    }

    if let Some(return_type) = pou.return_type {
        index.register_local_variable(pou.name.clone(), pou.name.clone(), VariableType::Return, get_type_name(&return_type))
    }

}


fn visit_global_var_block(index :&mut Index, block: &VariableBlock) {
    for var in &block.variables {
        let block_type = get_variable_type_from_block(block);
        index.register_global_variable(
                            var.name.clone(), 
                            block_type,
                            get_type_name(&var.data_type)
                        );
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    match block.variable_block_type {
        VariableBlockType::Local => VariableType::Local,
        VariableBlockType::Input => VariableType::Input,
        VariableBlockType::Global => VariableType::Global,
    }
}

fn get_type_name(data_type: &Type) -> String {
    let type_name = match data_type {
        Type::Primitive( prim_type ) => {
            format!("{:?}", prim_type)
        },
        Type::Custom =>
            unimplemented!("Custom datatypes cannot be indexed yet")
    };

    type_name
}