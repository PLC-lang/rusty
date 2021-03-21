/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

/// module to generate llvm intermediate representation for a CompilationUnit
use self::generators::{
    data_type_generator,
    llvm::LLVM,
    pou_generator::{self, PouGenerator},
    variable_generator,
};
use crate::compile_error::CompileError;

use super::ast::*;
use super::index::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};

mod generators;
#[cfg(test)]
mod tests;
mod typesystem;

/// Touple consisting of an llvm value with it's DataType
type TypeAndValue<'a> = (DataTypeInformation<'a>, BasicValueEnum<'a>);

/// Touple consisting of an llvm pointer-value and the resulting DataType
/// what to expect when loading the given pointer
pub struct TypeAndPointer<'a, 'b> {
    /// the index-entry of the datatype of the dereferenced pointer
    type_entry: &'b DataTypeIndexEntry<'a>,
    /// the pointer value
    ptr_value: PointerValue<'a>,
}

impl<'a, 'b> TypeAndPointer<'a, 'b> {
    /// constructs a new TypeAndPointer
    pub fn new(
        entry: &'b DataTypeIndexEntry<'a>,
        value: PointerValue<'a>,
    ) -> TypeAndPointer<'a, 'b> {
        TypeAndPointer {
            type_entry: entry,
            ptr_value: value,
        }
    }

    /// returns the DataTypeInformation for the pointer's dereferenced type
    pub fn get_type_information(&self) -> &DataTypeInformation<'a> {
        self.type_entry.get_type_information().unwrap() //TODO
    }
}

/// the codegen struct carries all dependencies required to generate
/// the IR code for a compilation unit
pub struct CodeGen<'ctx> {
    /// the LLVM context used to access the llvm typesystem, and create BasicBlocks
    pub context: &'ctx Context,
    /// the module represents a llvm compilation unit
    pub module: Module<'ctx>,
    /// the builder is a reusable object to create ir-statements at a given location
    pub builder: Builder<'ctx>,
    /// the index / symbol table
    pub index: &'ctx mut Index<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(
        context: &'ctx Context,
        index: &'ctx mut Index<'ctx>,
        module_name: &str,
    ) -> CodeGen<'ctx> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let mut codegen = CodeGen {
            context,
            module,
            builder,
            index,
        };
        // TODO: this should be part of constructing the index, when generating multiple modules we want to share
        // the context and index
        codegen.initialize_type_system();
        codegen
    }

    /// generates all TYPEs, GLOBAL-sections and POUs of th egiven CompilationUnit
    pub fn generate(&mut self, unit: CompilationUnit) -> Result<String, CompileError> {
        let llvm = LLVM::new(self.context, self.context.create_builder());
        data_type_generator::generate_data_type_stubs(&llvm, &mut self.index, &unit.types)?;
        data_type_generator::generate_data_type(&self.module, &llvm, &mut self.index, &unit.types)?;

        for global_variables in &unit.global_vars {
            for v in &global_variables.variables {
                variable_generator::generate_global_variable(&self.module, &llvm, self.index, v)?;
            }
        }

        //index all pou's
        for unit in &unit.units {
            let variable_names : Vec<String> = unit.variable_blocks.iter()
                    .flat_map(|it| &it.variables)
                    .map(|it| it.name.to_string())
                    .collect();
            pou_generator::index_pou(unit.name.as_str(), self.context, self.index, variable_names);
        }

        //generate all pou's
        let mut pou_generator = PouGenerator::new(&llvm, &mut self.index);
        //Generate the POU stubs in the first go to make sure they can be referenced.
        for unit in &unit.units {
            pou_generator.generate_pou_stub(unit, &self.module)?;
        }

        for unit in &unit.units {
            //Don't generate external functions
            if unit.linkage != LinkageType::External {
                pou_generator.generate_pou(unit)?;
            }
        }

        // TODO this is not needed
        Ok(self.module.print_to_string().to_string())
    }
}
