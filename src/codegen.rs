// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// module to generate llvm intermediate representation for a CompilationUnit
use self::{
    generators::{
        data_type_generator,
        llvm::Llvm,
        pou_generator::{self, PouGenerator},
        variable_generator,
    },
    llvm_index::LlvmTypedIndex,
};
use crate::{compile_error::CompileError, resolver::AnnotationMap};

use super::ast::*;
use super::index::*;
use inkwell::context::Context;
use inkwell::module::Module;

mod generators;
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
}

impl<'ink> CodeGen<'ink> {
    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(context: &'ink Context, module_name: &str) -> CodeGen<'ink> {
        let module = context.create_module(module_name);
        CodeGen { context, module }
    }

    pub fn generate_llvm_index(
        &self,
        annotations: &AnnotationMap,
        global_index: &Index,
    ) -> Result<LlvmTypedIndex<'ink>, CompileError> {
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let mut index = LlvmTypedIndex::default();
        //Generate types index, and any global variables associated with them.
        let llvm_type_index =
            data_type_generator::generate_data_types(&llvm, global_index, annotations)?;
        index.merge(llvm_type_index);

        //Generate global variables
        let llvm_gv_index = variable_generator::generate_global_variables(
            &self.module,
            &llvm,
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
        )?;
        index.merge(llvm_impl_index);
        Ok(index)
    }

    /// generates all TYPEs, GLOBAL-sections and POUs of the given CompilationUnit
    pub fn generate(
        &self,
        unit: &CompilationUnit,
        annotations: &AnnotationMap,
        global_index: &Index,
        llvm_index: &LlvmTypedIndex,
    ) -> Result<String, CompileError> {
        //generate all pous
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let pou_generator = PouGenerator::new(llvm, global_index, annotations, llvm_index);

        //Generate the POU stubs in the first go to make sure they can be referenced.
        for implementation in &unit.implementations {
            //Don't generate external functions
            if implementation.linkage != LinkageType::External {
                pou_generator.generate_implementation(implementation)?;
            }
        }

        Ok(self.module.print_to_string().to_string())
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
