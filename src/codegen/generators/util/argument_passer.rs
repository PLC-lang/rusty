use anyhow::{anyhow, Result};
use inkwell::values::{BasicValue, FunctionValue, PointerValue};
use plc_ast::ast::{Assignment, AstNode, AstStatement};

use crate::{codegen::generators::{expression_visitor::ExpressionVisitor, llvm::Llvm}, index::{Index, VariableIndexEntry}, resolver::AstAnnotations};

use super::reference_builder::GeneratedValue;


pub struct Argument<'idx, 'ast> {
    formal: &'idx VariableIndexEntry,
    actual: &'ast AstNode,
}

pub struct CallArguments<'idx, 'ast, 'ink> {
    arguments: Vec<Argument<'idx, 'ast>>,
    annotations: &'idx AstAnnotations,
    index: &'idx Index,
    llvm: &'idx Llvm<'ink>,
}

impl <'idx, 'ast> Argument<'idx, 'ast> {
    pub fn new(formal: &'idx VariableIndexEntry, actual: &'ast AstNode) -> Self {
        Argument { formal, actual }
    }
}

impl <'idx, 'ast, 'ink> CallArguments<'idx, 'ast, 'ink> {
    pub fn new(annotations: &'idx AstAnnotations, index: &'idx Index, llvm: &'idx Llvm<'ink>, arguments: Vec<Argument<'idx, 'ast>>) -> Self {
        CallArguments { arguments, annotations, index, llvm }
    }


    pub fn generate_program_call(&self, fv: FunctionValue<'_>, instance: &PointerValue, variable_visitor: &mut ExpressionVisitor<'_, '_>) 
    -> Result<GeneratedValue<'_>> {
        let input_assignments = self.arguments
            .iter().filter(|it| it.formal.is_input() || it.formal.is_inout() );

        // INPUTs
        for argument in input_assignments {

            let actual_value = if argument.formal.is_inout() {
                // if this is an inout, we need to pass a pointer
                variable_visitor.generate_expression(argument.actual)?.as_pointer_value()?.into()
            } else {
                // if this is an input we pass an rvalue
                variable_visitor.generate_r_value(dbg!(argument.actual))?
            };

            let gep = self.llvm
                .builder
                .build_struct_gep(
                    *instance,
                    argument.formal.get_location_in_parent(),
                    argument.formal.get_name(),
                )
                .map_err(|_e| anyhow!("Failed to create GEP"))?;
            self.llvm.builder.build_store(gep, actual_value);
        }
        self.llvm.builder.build_call(fv, &[instance.as_basic_value_enum().into()], "call"); //todo we should use the function's name here?

        // OUTPUTs
        for parameter in self.arguments.iter().filter(|it| it.formal.is_output()) {
             // gep the left side and load the value
            let output_variable = self.llvm
                .builder
                .build_struct_gep(*instance, parameter.formal.get_location_in_parent(), "")
                .map_err(|_e| anyhow!("Failed to create GEP"))?;
            let value = self.llvm.builder.build_load(output_variable, "");

            // store into the right side
            let target = variable_visitor.generate_expression(parameter.actual)?.as_pointer_value()?;
            self.llvm.builder.build_store(
                target,
                value);

        }
        Ok(GeneratedValue::NoValue)
    }


}