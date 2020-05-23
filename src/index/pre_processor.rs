use super::super::ast::{ Variable, CompilationUnit,DataType,  DataTypeDeclaration};


fn pre_process(unit: &mut CompilationUnit) {
    let mut all_variables :Vec<&mut Variable> = unit.global_vars.iter_mut()
                .flat_map(|gv| gv.variables.iter_mut())
                .filter(|x| match x.data_type {
                    DataTypeDeclaration::DataTypeReference {..} => false,
                    DataTypeDeclaration::DataTypeDefinition {..} => true,
                }).collect();
             
    for var in all_variables {
     pre_process_variable_data_type("global", var, &mut unit.types)   
    }



}

fn pre_process_variable_data_type(container_name: &str, variable: &mut Variable, types: &mut Vec<DataType>) {
    let new_type_name = format!("__{}_{}", container_name, variable.name);
    if let DataTypeDeclaration::DataTypeDefinition {mut data_type}  = variable.replace_data_type_with_reference_to(new_type_name.clone()) {
        // create index entry
        data_type.set_name(new_type_name);
        types.push(data_type);
            
    }
    //make sure it gets generated

}