// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// offers operations to generate global variables
use crate::{
    codegen::{debug::Debug, llvm_index::LlvmTypedIndex, llvm_typesystem::cast_if_needed},
    index::{Index, PouIndexEntry, VariableIndexEntry},
    resolver::{AnnotationMap, AstAnnotations, Dependency},
    OnlineChange,
};
use inkwell::{module::Module, values::GlobalValue};
use plc_ast::ast::LinkageType;
use plc_diagnostics::diagnostics::Diagnostic;
use section_mangler::SectionMangler;

use super::{
    data_type_generator::get_default_for,
    expression_generator::ExpressionCodeGenerator,
    llvm::{GlobalValueExt, Llvm},
    section_names,
};
use crate::codegen::debug::DebugBuilderEnum;
use crate::index::FxIndexSet;

pub struct VariableGenerator<'ctx, 'b> {
    module: &'b Module<'ctx>,
    llvm: &'b Llvm<'ctx>,
    global_index: &'b Index,
    annotations: &'b AstAnnotations,
    types_index: &'b LlvmTypedIndex<'ctx>,
    debug: &'b mut DebugBuilderEnum<'ctx>,
    online_change: &'b OnlineChange,
}

impl<'ctx, 'b> VariableGenerator<'ctx, 'b> {
    pub fn new(
        module: &'b Module<'ctx>,
        llvm: &'b Llvm<'ctx>,
        global_index: &'b Index,
        annotations: &'b AstAnnotations,
        types_index: &'b LlvmTypedIndex<'ctx>,
        debug: &'b mut DebugBuilderEnum<'ctx>,
        online_change: &'b OnlineChange,
    ) -> Self {
        VariableGenerator { module, llvm, global_index, annotations, types_index, debug, online_change }
    }

    pub fn generate_global_variables(
        &mut self,
        dependencies: &FxIndexSet<Dependency>,
        location: &'b str,
    ) -> Result<LlvmTypedIndex<'ctx>, Diagnostic> {
        let mut index = LlvmTypedIndex::default();

        let mut globals = vec![];
        dependencies.iter().for_each(|dep| {
            if let Some(dep) = match dep {
                Dependency::Datatype(name) => {
                    //Attempt to find a pou with that name
                    if let Some(PouIndexEntry::Program { instance_variable, .. }) =
                        self.global_index.find_pou(name).as_ref()
                    {
                        Some((name.as_str(), instance_variable.as_ref()))
                    } else {
                        None
                    }
                }
                Dependency::Variable(name) => {
                    self.global_index.find_fully_qualified_variable(name).map(|it| (name.as_str(), it))
                }
                Dependency::Call(_) => None,
            } {
                globals.push(dep);
            }
        });

        for (name, variable) in &globals {
            let linkage =
                if !variable.is_in_unit(location) { LinkageType::External } else { variable.get_linkage() };
            let global_variable = self.generate_global_variable(variable, linkage).map_err(|err| {
                match err.get_error_code() {
                    //If we encounter a missing function or an invalid reference, we wrap it in a more generic issue
                    "E072" | "E048" => {
                        Diagnostic::new(format!("Cannot generate literal initializer for `{name}`."))
                            .with_error_code("E041")
                            .with_sub_diagnostic(err)
                    }
                    _ => err,
                }
            })?;
            index.associate_global(name, global_variable)?;

            if !self.global_index.is_enum_variant(variable.get_qualified_name())
            {
                // generate debug info for non-enum-variant variables
                self.debug.create_global_variable(
                    variable.get_qualified_name(),
                    &variable.data_type_name,
                    global_variable,
                    &variable.source_location,
                );
            }
        }

        Ok(index)
    }

    /// convenience function to generates a global variable for the given variable
    ///
    /// - `module` the module to generate the variable into
    /// - `llvm` the struct used to generate IR-code
    /// - `index` the global symbol table, the global variable will be registerd as a new symbol
    /// - `global_variable` the variable to generate
    fn generate_global_variable(
        &self,
        global_variable: &VariableIndexEntry,
        linkage: LinkageType,
    ) -> Result<GlobalValue<'ctx>, Diagnostic> {
        let type_name = global_variable.get_type_name();
        let variable_type = self.types_index.get_associated_type(type_name)?;

        let name = if self.global_index.get_type_information_or_void(type_name).is_enum() {
            global_variable.get_qualified_name()
        } else {
            global_variable.get_name()
        };

        let mut global_ir_variable = self.llvm.create_global_variable(self.module, name, variable_type);
        if linkage.is_external_or_included() {
            global_ir_variable = global_ir_variable.make_external();
        } else {
            let initial_value = if global_variable
                .initial_value
                .and_then(|it| self.global_index.get_const_expressions().find_const_expression(&it))
                .is_some_and(|it| it.is_address_unresolvable())
            {
                None
            } else if let Some(initializer) = self
                .global_index
                .get_const_expressions()
                .maybe_get_constant_statement(&global_variable.initial_value)
            {
                let expr_generator = ExpressionCodeGenerator::new_context_free(
                    self.llvm,
                    self.global_index,
                    self.annotations,
                    self.types_index,
                );

                //see if this value was compile-time evaluated ...
                let value = expr_generator.generate_expression(initializer)?;
                let target_type = self.global_index.get_effective_type_or_void_by_name(type_name);
                let value_type = self.annotations.get_type_or_void(initializer, self.global_index);
                Some(cast_if_needed!(expr_generator, target_type, value_type, value, None)?)
            } else {
                None
            };
            let initial_value = initial_value
                // 2nd try: find an associated default value for the declared type
                .or_else(|| self.types_index.find_associated_initial_value(type_name))
                // 3rd try: get the compiler's default for the given type (zero-initializer)
                .or_else(|| self.types_index.find_associated_type(type_name).map(get_default_for));
            global_ir_variable.set_initial_value(initial_value, variable_type);
            if global_variable.is_constant() && initial_value.is_none() {
                return Err(Diagnostic::codegen_error(
                    "Cannot generate uninitialized constant",
                    &global_variable.source_location,
                ));
            }
        }

        if global_variable.is_constant() {
            global_ir_variable = global_ir_variable.make_constant();
        };

        let global_name = if global_variable.get_name().ends_with("instance") {
            global_variable.get_name()
        } else {
            global_variable.get_qualified_name()
        };
        let global_name = global_name.to_lowercase();

        if self.online_change.is_enabled() {
            let section = SectionMangler::variable(
                global_name,
                section_names::mangle_type(
                    self.global_index,
                    self.global_index.get_effective_type_by_name(global_variable.get_type_name())?,
                )?,
            )
            .mangle();
            global_ir_variable.set_section(Some(&section));
        }

        Ok(global_ir_variable)
    }
}
