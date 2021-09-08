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
use crate::{
    codegen::generators::expression_generator::ExpressionCodeGenerator,
    compile_error::CompileError, resolver::AnnotationMap,
};

use super::ast::*;
use super::index::*;
use crate::typesystem::{DataType, *};
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
        self.generate_global_constant_literals(global_index, &llvm, &mut index)?;

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

    /// generates the BasicValueEnums for the const initializers that could be
    /// evaluated at compile time
    /// e.g. `VAR GLOBAL CONST x := 7; END_VAR`
    /// generates the Value for `7` and associates it with the name `x`
    fn generate_global_constant_literals<'a>(
        &self,
        global_index: &Index,
        llvm: &Llvm<'a>,
        index: &mut LlvmTypedIndex<'a>,
    ) -> Result<(), CompileError> {
        for (qualified_name, literal) in global_index.get_all_resolved_constants() {
            match global_index
                .find_variable(None, &qualified_name.split('.').collect::<Vec<&str>>())
                .and_then(|it| global_index.find_effective_type_by_name(it.get_type_name()))
            {
                Some(data_type) => {
                    //TODO this would be much much easier, if we would not generate these just from index, but form the VAR-GLOBAL itself
                    let literal_statement =
                        literal.generate_ast_literal(0, SourceRange::undefined());

                    let annotations = AnnotationMap::with(
                        literal_statement.get_id(),
                        crate::resolver::StatementAnnotation::expression(data_type.get_name()),
                    );
                    let generator = ExpressionCodeGenerator::new_context_free(
                        llvm,
                        global_index,
                        &annotations,
                        index,
                        Some(data_type.clone_type_information()), //TODO why do we need to clone this?
                    );
                    let (_, llvm_value) = generator.generate_literal(&literal_statement)?;
                    index.associate_constant(qualified_name, llvm_value);
                }
                None => {
                    return Err(CompileError::codegen_error(
                        format!(
                            "Unable to generate the global constant initialization for {:}: {:}",
                            qualified_name, "Cannot find the variable's type."
                        ),
                        SourceRange::undefined(),
                    ))
                }
            }
        }
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
