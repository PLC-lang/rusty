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
use crate::typesystem::{DataType, *};
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, PointerValue};
use inkwell::{context::Context, values::BasicValue};

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
    pub fn new(entry: &'b DataType, value: PointerValue<'a>) -> TypeAndPointer<'a, 'b> {
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
}

impl<'ink> CodeGen<'ink> {
    /// constructs a new code-generator that generates CompilationUnits into a module with the given module_name
    pub fn new(context: &'ink Context, module_name: &str) -> CodeGen<'ink> {
        let module = context.create_module(module_name);
        CodeGen { context, module }
    }

    fn generate_llvm_index(
        &self,
        module: &Module<'ink>,
        annotations: &AnnotationMap,
        global_index: &Index,
    ) -> Result<LlvmTypedIndex<'ink>, CompileError> {
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let mut index = LlvmTypedIndex::new();
        //Generate types index, and any global variables associated with them.
        let llvm_type_index =
            data_type_generator::generate_data_types(&llvm, global_index, annotations)?;
        index.merge(llvm_type_index);

        //generate llvm values for constants
        for (qualified_name, literal) in global_index.get_all_resolved_constants() {
            match global_index
                .find_variable(None, &qualified_name.split('.').collect::<Vec<&str>>())
                .and_then(|it| global_index.find_effective_type_by_name(it.get_type_name()))
                .and_then(|dt| {
                    index
                        .find_associated_type(dt.get_name())
                        .map(|it| (dt.get_type_information(), it))
                }) {
                Some((data_type, llvm_data_type)) => {
                    let initial_literal = match literal {
                        LiteralValue::Int(val) =>
                            //if llvm_data_type.is_int_type() && data_type.is_int() =>
                        {
                            llvm_data_type
                                .into_int_type()
                                .const_int(*val as u64, data_type.is_signed_int())
                                .as_basic_value_enum()
                        }
                        LiteralValue::Real(val) => {
                            llvm_data_type
                                .into_float_type()
                                .const_float(*val)
                                .as_basic_value_enum()
                        },
                        LiteralValue::Bool(val) => {
                            if *val {
                                llvm.bool_type().const_int(1, false)
                            } else {
                                llvm.bool_type().const_int(0, false)
                            }.as_basic_value_enum()
                        },
                        LiteralValue::String(val) => {
                            llvm.create_const_utf8_string(val.as_str())?.1.as_basic_value_enum()
                        },
                        LiteralValue::WString(val) => {
                            llvm.create_const_utf16_string(val.as_str())?.1.as_basic_value_enum()
                        },
                    };
                    index.associate_constant(qualified_name, initial_literal);
                }
                None => todo!("no datatype for const"),
            }
        }

        //Generate global variables
        let llvm_gv_index = variable_generator::generate_global_variables(
            module,
            &llvm,
            global_index,
            annotations,
            &index,
        )?;
        index.merge(llvm_gv_index);

        //Generate opaque functions for implementations and associate them with their types
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let llvm_impl_index = pou_generator::generate_implementation_stubs(
            module,
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
    ) -> Result<String, CompileError> {
        //Associate the index type with LLVM types
        let llvm_index = self.generate_llvm_index(&self.module, annotations, global_index)?;

        //generate all pous
        let llvm = Llvm::new(self.context, self.context.create_builder());
        let pou_generator = PouGenerator::new(llvm, global_index, annotations, &llvm_index);
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
