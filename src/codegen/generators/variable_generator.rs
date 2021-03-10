/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables

use inkwell::{module::Module, values::GlobalValue};

use crate::{
    ast::Variable,
    compile_error::CompileError,
    index::{Index},
};

use super::{expression_generator::ExpressionCodeGenerator, llvm::LLVM};

/// convenience function to generates a global variable for the given variable
///
/// - `module` the module to generate the variable into
/// - `llvm` the struct used to generate IR-code
/// - `index` the global symbol table, the global variable will be registerd as a new symbol
/// - `global_variable` the variable to generate
pub fn generate_global_variable<'ctx, 'b>(
    module: &Module<'ctx>,
    llvm : &'b LLVM<'ctx>,
    index: &'b mut Index<'ctx>,
    global_variable: &Variable,
) -> Result<GlobalValue<'ctx>, CompileError> {
    let type_name = global_variable.data_type.get_name().unwrap(); //TODO
    let variable_type_index_entry = index.get_type(type_name)?;
    let variable_type = variable_type_index_entry.get_type().unwrap();

    let initial_value = if let Some(initializer) = &global_variable.initializer {
        let expr_generator = ExpressionCodeGenerator::new_context_free(llvm, index, Some(variable_type_index_entry.get_type_information().unwrap().clone()));
        let (_, value) = expr_generator.generate_expression(&initializer)?;
        //Todo cast if necessary
        Some(value)
    } else {
        None
    };
    let initial_value = initial_value.or(variable_type_index_entry.get_initial_value());
    let global_ir_variable = llvm.create_global_variable(module, &global_variable.name, variable_type, initial_value);
    index.associate_global_variable(&global_variable.name, global_ir_variable.as_pointer_value());
    Ok(global_ir_variable)
}
