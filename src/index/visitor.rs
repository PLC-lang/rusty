
use super::Index;
use super::VariableType;
use super::PouKind;
use super::super::ast::{ POU, CompilationUnit, VariableBlock, Type , PouType };

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
            index.register_variable(
                var.name.as_str().to_string(),
                block_type,
                false,
                0,
                0,
                get_type_name(&var.data_type)
            );
        }
    }

}

fn visit_global_var_block(index :&mut Index, block: &VariableBlock) {
    for var in &block.variables {
        index.register_variable(
                            var.name.as_str().to_string(), 
                            VariableType::Global,
                            false,  //const not supported yet
                            0,  //arrays not supported yet
                            0,  //arrays not supported yet
                            get_type_name(&var.data_type)
                        );
    }
}

fn get_variable_type_from_block(block: &VariableBlock) -> VariableType {
    VariableType::Input
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