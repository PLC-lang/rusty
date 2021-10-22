// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables
use crate::{ast::SourceRange, index::Index, resolver::AnnotationMap};
use inkwell::{module::Module, values::GlobalValue};

use crate::{
    codegen::llvm_index::LlvmTypedIndex, compile_error::CompileError, index::VariableIndexEntry,
};

use super::{expression_generator::ExpressionCodeGenerator, llvm::Llvm};

pub fn generate_global_variables<'ctx, 'b>(
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    annotations: &'b AnnotationMap,
    types_index: &'b LlvmTypedIndex<'ctx>,
) -> Result<LlvmTypedIndex<'ctx>, CompileError> {
    let mut index = LlvmTypedIndex::new();
    let globals = global_index.get_globals();
    let enums = global_index.get_global_qualified_enums();
    for (name, variable) in globals.into_iter().chain(enums.into_iter()) {
        let global_variable = generate_global_variable(
            module,
            llvm,
            global_index,
            annotations,
            types_index,
            variable,
        )
        .map_err(|err| {
            if let CompileError::MissingFunctionError { .. } = err {
                CompileError::cannot_generate_initializer(name.as_str(), SourceRange::undefined())
            } else {
                err
            }
        })?;
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
    annotations: &'b AnnotationMap,
    index: &'b LlvmTypedIndex<'ctx>,
    global_variable: &VariableIndexEntry,
) -> Result<GlobalValue<'ctx>, CompileError> {
    let type_name = global_variable.get_type_name();
    let variable_type = index.get_associated_type(type_name)?;

    let initial_value = if let Some(initializer) = global_index
        .get_const_expressions()
        .maybe_get_constant_statement(&global_variable.initial_value)
    {
        let expr_generator =
            ExpressionCodeGenerator::new_context_free(llvm, global_index, annotations, index);

        //see if this value was compile-time evaluated ...
        if let Some(value) = index.find_constant_value(global_variable.get_qualified_name()) {
            Some(value)
        } else {
            let value = expr_generator.generate_expression(initializer)?;
            Some(value)
        }
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
