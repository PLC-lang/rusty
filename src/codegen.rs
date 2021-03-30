/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// module to generate llvm intermediate representation for a CompilationUnit
use self::{generators::{
    data_type_generator,
    llvm::LLVM,
    pou_generator::{self, PouGenerator},
    variable_generator,
}, llvm_index::LLVMTypedIndex};
use crate::compile_error::CompileError;

use super::ast::*;
use super::index::*;
use crate::typesystem::{*, DataType};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};

mod generators;
mod llvm_index;
mod llvm_typesystem;
#[cfg(test)]
mod tests;

/// Tuple consisting of an llvm value with it's DataType
type TypeAndValue<'a> = (DataTypeInformation, BasicValueEnum<'a>);

/// Tuple consisting of an llvm pointer-value and the resulting DataType
/// what to expect when loading the given pointer
pub struct TypeAndPointer<'a, 'b> {
    /// the index-entry of the datatype of the dereferenced pointer
    type_entry: &'b DataType,
    /// the pointer value
    ptr_value: PointerValue<'a>,
}

impl<'a, 'b> TypeAndPointer<'a, 'b> {
    /// constructs a new TypeAndPointer
    pub fn new(
        entry: &'b DataType,
        value: PointerValue<'a>,
    ) -> TypeAndPointer<'a, 'b> {
        TypeAndPointer {
            type_entry: entry,
            ptr_value: value,
        }
    }

    /// returns the DataTypeInformation for the pointer's dereferenced type
    pub fn get_type_information(&self) -> &DataTypeInformation {
        self.type_entry.get_type_information()
    }
}

/// the codegen struct carries all dependencies required to generate
/// the IR code for a compilation unit
pub struct CodeGen<'ink> {
    /// the LLVM context used to access the llvm typesystem, and create BasicBlocks
    pub context: &'ink Context,
    /// the module represents a llvm compilation unit
    pub module: Module<'ink>,
    /// the index / symbol table
    pub index: Index,
}

impl<'ink> CodeGen<'ink> {

    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(
        context : &'ink Context,
        index: Index,
        module_name: &str,
    ) -> CodeGen<'ink> {
        let module = context.create_module(module_name);
        let codegen = CodeGen {
            context,
            module,
            index,
        };
        codegen
    }

    fn generate_llvm_index(&self, module : &Module<'ink>, global_index : &Index, ) -> Result<LLVMTypedIndex<'ink>, CompileError>{
        let llvm = LLVM::new(&self.context, self.context.create_builder());
        let mut index = LLVMTypedIndex::new();
        //Generate types index, and any global variables associated with them.
        let llvm_type_index = data_type_generator::generate_data_types(&llvm, global_index)?;
        index.merge(llvm_type_index);
        //Generate global variables
        let llvm_gv_index = variable_generator::generate_global_variables(module, &llvm, global_index, &index)?;
        index.merge(llvm_gv_index);
        //Generate opaque functions for implementations and associate them with their types
        let llvm = LLVM::new(&self.context, self.context.create_builder());
        let llvm_impl_index = pou_generator::generate_implementation_stubs(module, llvm, global_index, &index)?;
        index.merge(llvm_impl_index);
        Ok(index)
    }

    /// generates all TYPEs, GLOBAL-sections and POUs of the given CompilationUnit
    pub fn generate(&self, unit: CompilationUnit) -> Result<String, CompileError> {
        //Associate the index type with LLVM types
        let llvm_index = self.generate_llvm_index(&self.module, &self.index)?;
        
        //generate all pous
        let llvm = LLVM::new(&self.context, self.context.create_builder());
        let pou_generator = PouGenerator::new(llvm, &self.index, &llvm_index);
        //Generate the POU stubs in the first go to make sure they can be referenced.
        for (i,pou) in unit.units.iter().enumerate() {
            //Don't generate external functions
            if pou.linkage != LinkageType::External {
                pou_generator.generate_pou(pou, &unit.implementations[i])?;
            }
        }
        Ok(self.module.print_to_string().to_string())

    }
}
