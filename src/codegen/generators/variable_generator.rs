// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables
use crate::index::Index;
use inkwell::{module::Module, values::GlobalValue};

use crate::{
    codegen::llvm_index::LlvmTypedIndex, compile_error::CompileError, index::VariableIndexEntry,
};

use super::{expression_generator::ExpressionCodeGenerator, llvm::Llvm};

pub fn generate_global_variables<'ctx, 'b>(
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    types_index: &'b LlvmTypedIndex<'ctx>,
) -> Result<LlvmTypedIndex<'ctx>, CompileError> {
    let mut index = LlvmTypedIndex::new();
    let globals = global_index.get_globals();
    for (name, variable) in globals {
        let global_variable =
            generate_global_variable(module, llvm, global_index, types_index, variable)?;
        index.associate_global(name, global_variable)?
    }
    Ok(index)
}

/// convenience function to generates a global variable for the given variable
///
/// - `module` the module to generate the variable into
/// - `llvm` the struct used to generate IR-code
/// - `index` the global symbol table, the global variable will be registerd as a new symbol
/// - `global_variable` the variable to generate
pub fn generate_global_variable<'ctx, 'b>(
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    index: &'b LlvmTypedIndex<'ctx>,
    global_variable: &VariableIndexEntry,
) -> Result<GlobalValue<'ctx>, CompileError> {
    let type_name = global_variable.get_type_name();
    let variable_type = index.get_associated_type(type_name)?;

    let initial_value = if let Some(initializer) = &global_variable.initial_value {
        let expr_generator = ExpressionCodeGenerator::new_context_free(
            llvm,
            global_index,
            index,
            Some(global_index.get_type_information(type_name).unwrap()),
        );
        let (_, value) = expr_generator.generate_expression(initializer)?;
        //Todo cast if necessary
        Some(value)
    } else {
        None
    };
    let initial_value = initial_value.or_else(|| index.find_associated_initial_value(type_name));
    let global_ir_variable = llvm.create_global_variable(
        module,
        global_variable.get_name(),
        variable_type,
        initial_value,
    );
    Ok(global_ir_variable)
}
