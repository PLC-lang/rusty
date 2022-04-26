// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables
use crate::{
    ast::SourceRange,
    diagnostics::{Diagnostic, ErrNo},
    index::Index,
    resolver::AstAnnotations,
};
use inkwell::{module::Module, values::GlobalValue};

use crate::{codegen::llvm_index::LlvmTypedIndex, index::VariableIndexEntry};

use super::{
    data_type_generator::get_default_for,
    expression_generator::ExpressionCodeGenerator,
    llvm::{GlobalValueExt, Llvm},
};

pub fn generate_global_variables<'ctx, 'b>(
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    annotations: &'b AstAnnotations,
    types_index: &'b LlvmTypedIndex<'ctx>,
) -> Result<LlvmTypedIndex<'ctx>, Diagnostic> {
    let mut index = LlvmTypedIndex::default();
    let globals = global_index.get_globals();
    let initializers = global_index.get_global_initializers();
    let enums = global_index.get_global_qualified_enums();
    for (name, variable) in globals
        .into_iter()
        .chain(initializers.into_iter())
        .chain(enums.into_iter())
    {
        let global_variable = generate_global_variable(
            module,
            llvm,
            global_index,
            annotations,
            types_index,
            variable,
        )
        .map_err(|err| match err.get_type() {
            ErrNo::codegen__missing_function | ErrNo::reference__unresolved => {
                Diagnostic::cannot_generate_initializer(name.as_str(), SourceRange::undefined())
            }
            _ => err,
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
    annotations: &'b AstAnnotations,
    index: &'b LlvmTypedIndex<'ctx>,
    global_variable: &VariableIndexEntry,
) -> Result<GlobalValue<'ctx>, Diagnostic> {
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

    let mut global_ir_variable =
        llvm.create_global_variable(module, global_variable.get_name(), variable_type);
    if global_variable.is_external() {
        global_ir_variable = global_ir_variable.make_external();
    } else {
        let initial_value = initial_value
            // 2nd try: find an associated default value for the declared type
            .or_else(|| index.find_associated_initial_value(type_name))
            // 3rd try: get the compiler's default for the given type (zero-initializer)
            .or_else(|| index.find_associated_type(type_name).map(get_default_for));
        global_ir_variable.set_initial_value(initial_value, variable_type);
        if global_variable.is_constant() {
            global_ir_variable = global_ir_variable.make_constant();
            if initial_value.is_none() {
                return Err(Diagnostic::codegen_error(
                    "Cannot generate uninitialized constant",
                    global_variable.source_location.clone(),
                ));
            }
        }
    }
    Ok(global_ir_variable)
}
