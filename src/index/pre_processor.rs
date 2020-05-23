use std::iter;
use super::super::ast::{ POU, PouType, Variable, CompilationUnit, VariableBlock, VariableBlockType, DataType, DataType::DataTypeReference};


fn pre_process(unit: &mut CompilationUnit) {

    let all_variables = unit.global_vars.into_iter()
                .map(|gv : &VariableBlock| &gv.variables)
                .flatten();


}

fn pre_process_variable(container_name: &str, blocks: &Vec<VariableBlock>) {


}