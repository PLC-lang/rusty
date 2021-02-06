/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use crate::compile_error::CompileError;
use self::{pou_generator::PouGenerator};
use super::ast::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum};
use inkwell::values::{
    BasicValueEnum, PointerValue,
};
use super::index::*;

#[cfg(test)]
mod tests;
mod typesystem;
mod literals;
mod pou_generator;
mod statement_generator;
mod instance_struct_generator;
mod variable_generator;
mod expression_generator;
mod data_type_generator;

type ExpressionValue<'a> = (Option<DataTypeInformation<'a>>, Option<BasicValueEnum<'a>>);
type TypeAndValue<'a> = (DataTypeInformation<'a>, BasicValueEnum<'a>);

///
/// a touple (name, data_type, initializer) describing the declaration of a variable.
///
type VariableDeclarationInformation<'a> = (String, BasicTypeEnum<'a>, Option<BasicValueEnum<'a>>);

pub struct LValue<'ctx> {
    type_name: String,
    type_information: DataTypeInformation<'ctx>,
    ptr_value: PointerValue<'ctx>
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub index: &'ctx mut Index<'ctx>,
    pub new_lines: NewLines,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, index: &'ctx mut Index<'ctx>, new_lines: NewLines) -> CodeGen<'ctx> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let mut codegen = CodeGen {
            context,
            module,
            builder,
            index,
            new_lines,
        };
        codegen.initialize_type_system();
        codegen
    }

    pub fn generate(&mut self, root: CompilationUnit) -> Result<String, CompileError> {
        
        self.generate_compilation_unit(root)?;
        Ok(self.module.print_to_string().to_string())
    }

    pub fn generate_compilation_unit(&mut self, root: CompilationUnit) -> Result<(), CompileError> {
        data_type_generator::generate_data_type_stubs(&self.context, &mut self.index, &root.types)?;
        data_type_generator::generate_data_type(&self.module, &self.context, &mut self.index, &root.types)?;

        for global_variables in &root.global_vars {
            for v in &global_variables.variables {
                variable_generator::generate_global_variable(&self.module, self.context, &self.builder, self.index, v)?;
            }
        }

        //index all pou's
        for unit in &root.units {
            pou_generator::index_pou(unit.name.as_str(), self.context, self.index);
        }
        
        //generate all pou's
        for unit in &root.units {
            let mut pou_generator = PouGenerator::new(
                &self.context,
                &mut self.index);
                pou_generator.generate_pou(unit, &self.module)?;
            }
            
        Ok(())
    }
}