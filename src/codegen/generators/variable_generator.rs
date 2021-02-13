/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables

use inkwell::{module::Module, values::GlobalValue};


use crate::{
    ast::Variable,
    compile_error::CompileError,
    index::{Index},
};

use super::{expression_generator::ExpressionCodeGenerator, llvm::LLVM};

pub fn generate_global_variable<'ctx, 'b>(
    module: &Module<'ctx>,
    llvm : &'b LLVM<'ctx>,
    index: &'b mut Index<'ctx>,
    variable: &Variable,
) -> Result<GlobalValue<'ctx>, CompileError> {
    let type_name = variable.data_type.get_name().unwrap(); //TODO
    let variable_type_description = index.get_type(type_name)?;
    let variable_type = variable_type_description.get_type().unwrap();

    let initial_value = if let Some(initializer) = &variable.initializer {
        let expr_generator = ExpressionCodeGenerator::new_context_free(llvm, index, None);
        let (_, value) = expr_generator.generate_expression(&initializer)?;
        //Todo cast if necessary
        Some(value)
    } else {
        None
    };
    let initial_value = initial_value.or(variable_type_description.get_initial_value());
    let global_variable = llvm.create_global_variable(module, &variable.name, variable_type, initial_value);
    index.associate_global_variable(&variable.name, global_variable.as_pointer_value());
    Ok(global_variable)
}
