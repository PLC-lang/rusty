// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// module to generate llvm intermediate representation for a CompilationUnit
use self::{
    debug::{Debug, DebugBuilderEnum},
    generators::{
        data_type_generator,
        llvm::{GlobalValueExt, Llvm},
        pou_generator::{self, PouGenerator},
        variable_generator,
    },
    llvm_index::LlvmTypedIndex,
};
use crate::{
    diagnostics::{Diagnostic, Diagnostician},
    resolver::{AstAnnotations, StringLiterals},
    DebugLevel, OptimizationLevel,
};

use super::ast::*;
use super::index::*;
use inkwell::module::Module;
use inkwell::{context::Context, types::BasicType};

mod debug;
pub(crate) mod generators;
mod llvm_index;
mod llvm_typesystem;
#[cfg(test)]
mod tests;

/// the codegen struct carries all dependencies required to generate
/// the IR code for a compilation unit
pub struct CodeGen<'ink> {
    /// the LLVM context used to access the llvm typesystem, and create BasicBlocks
    pub context: &'ink Context,
    /// the module represents a llvm compilation unit
    pub module: Module<'ink>,
    /// the debugging module creates debug information at appropriate locations
    pub debug: DebugBuilderEnum<'ink>,
}

impl<'ink> CodeGen<'ink> {
    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(
        context: &'ink Context,
        module_name: &str,
        module_location: &str,
        optimization_level: OptimizationLevel,
        debug_level: DebugLevel,
    ) -> CodeGen<'ink> {
        let module = context.create_module(module_name);
        module.set_source_file_name(module_location);
        let debug = debug::DebugBuilderEnum::new(context, &module, optimization_level, debug_level);
        CodeGen {
            context,
            module,
            debug,
        }
    }

    pub fn generate_llvm_index(
        &mut self,
        annotations: &AstAnnotations,
        literals: StringLiterals,
        global_index: &Index,
        diagnostician: &Diagnostician,
    ) -> Result<LlvmTypedIndex<'ink>, Diagnostic> {
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let mut index = LlvmTypedIndex::default();
        //Generate types index, and any global variables associated with them.
        let llvm_type_index = data_type_generator::generate_data_types(
            &llvm,
            &mut self.debug,
            global_index,
            annotations,
            diagnostician,
        )?;
        index.merge(llvm_type_index);

        //Generate global variables
        let llvm_gv_index = variable_generator::generate_global_variables(
            &self.module,
            &llvm,
            &self.debug,
            global_index,
            annotations,
            &index,
        )?;
        index.merge(llvm_gv_index);

        //Generate opaque functions for implementations and associate them with their types
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let llvm_impl_index = pou_generator::generate_implementation_stubs(
            &self.module,
            llvm,
            global_index,
            annotations,
            &index,
            &mut self.debug,
        )?;
        let llvm = Llvm::new(self.context, self.context.create_builder());
        index.merge(llvm_impl_index);
        let llvm_values_index = pou_generator::generate_global_constants_for_pou_members(
            &self.module,
            &llvm,
            global_index,
            annotations,
            &index,
        )?;
        index.merge(llvm_values_index);

        //Generate constants for string-literal
        //generate literals but first sort, so we get reproducable builds
        let mut utf08s = literals.utf08.into_iter().collect::<Vec<String>>();
        utf08s.sort_unstable();
        for (idx, literal) in utf08s.into_iter().enumerate() {
            let len = literal.len() + 1;
            let data_type = llvm.context.i8_type().array_type(len as u32);
            let literal_variable = llvm.create_global_variable(
                &self.module,
                format!("utf08_literal_{}", idx).as_str(),
                data_type.as_basic_type_enum(),
            );
            let initializer = llvm.create_const_utf8_string(literal.as_str(), len)?;
            literal_variable
                .make_constant()
                .set_initializer(&initializer);

            index.associate_utf08_literal(literal, literal_variable);
        }
        //generate literals but first sort, so we get reproducable builds
        let mut utf16s = literals.utf16.into_iter().collect::<Vec<String>>();
        utf16s.sort_unstable();
        for (idx, literal) in utf16s.into_iter().enumerate() {
            let len = literal.len() + 1;
            let data_type = llvm.context.i16_type().array_type(len as u32);
            let literal_variable = llvm.create_global_variable(
                &self.module,
                format!("utf16_literal_{}", idx).as_str(),
                data_type.as_basic_type_enum(),
            );
            let initializer =
                llvm.create_const_utf16_string(literal.as_str(), literal.len() + 1)?;
            literal_variable
                .make_constant()
                .set_initializer(&initializer);

            index.associate_utf16_literal(literal, literal_variable);
        }

        Ok(index)
    }

    /// generates all TYPEs, GLOBAL-sections and POUs of the given CompilationUnit
    pub fn generate(
        &self,
        unit: &CompilationUnit,
        annotations: &AstAnnotations,
        global_index: &Index,
        llvm_index: &LlvmTypedIndex,
    ) -> Result<(), Diagnostic> {
        //generate all pous
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let pou_generator = PouGenerator::new(llvm, global_index, annotations, llvm_index);

        //Generate the POU stubs in the first go to make sure they can be referenced.
        for implementation in &unit.implementations {
            //Don't generate external or generic functions
            if let Some(entry) = global_index.find_pou(implementation.name.as_str()) {
                if !entry.is_generic() && entry.get_linkage() != &LinkageType::External {
                    pou_generator.generate_implementation(
                        implementation,
                        &self.debug,
                        &unit.new_lines,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Finalize needs to be called on the debug builder, to signify that the code generation is
    /// done and that the debug builder can now mark the debug information as complete. This is
    /// required to be called on the debug builder by the LLVM API, and has to happen on a module
    /// before it gets generated into object or IR
    pub fn finalize(&self) -> Result<(), Diagnostic> {
        self.debug.finalize();
        #[cfg(feature = "verify")]
        {
            self.module.verify().map_err(|it| Diagnostic::GeneralError {
                message: it.to_string(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            })
        }

        #[cfg(not(feature = "verify"))]
        Ok(())
    }
}

#[cfg(test)]
mod casting_big_numbers {
    #[test]
    fn casting_between_i128_and_u64() {
        let n: i128 = u64::MAX as i128;
        let nn: u64 = n as u64;
        assert_eq!(0xFFFF_FFFF_FFFF_FFFF_u64, nn);

        let n: i128 = i64::MAX as i128;
        let nn: u64 = n as u64;
        assert_eq!(0x7FFF_FFFF_FFFF_FFFF_u64, nn);
    }
}
