use anyhow::{anyhow, Result};
use inkwell::{
    values::{BasicValue, FunctionValue, PointerValue},
    AddressSpace,
};
use itertools::Itertools;
use plc_ast::ast::{Assignment, AstNode, AstStatement};

use crate::{
    codegen::generators::{
        expression_visitor::{ExpressionVisitor, GeneratedValue},
        llvm::Llvm,
    },
    index::{Index, VariableIndexEntry},
    resolver::{AnnotationMap, AstAnnotations},
};

pub struct Argument<'idx, 'ast> {
    formal: &'idx VariableIndexEntry,
    actual: &'ast AstNode,
}

impl<'idx, 'ast> Argument<'idx, 'ast> {
    pub fn new(formal: &'idx VariableIndexEntry, actual: &'ast AstNode) -> Self {
        Argument { formal, actual }
    }

    pub fn get_formal(&self) -> &'idx VariableIndexEntry {
        self.formal
    }

    pub fn get_actual(&self) -> &'ast AstNode {
        self.actual
    }
}

pub struct CallArguments<'idx, 'ast, 'ink> {
    arguments: Vec<Argument<'idx, 'ast>>,
    annotations: &'idx AstAnnotations,
    index: &'idx Index,
    llvm: &'idx Llvm<'ink>,
}



    // generates the actual parameter for a given agument as a GeneratedExpression
    fn generate_actual_parameter<'ink>(gen: &mut ExpressionVisitor<'ink, '_>, argument: &Argument<'_, '_>) -> Result<GeneratedValue<'ink>> {
        gen.generate_expression(argument.actual).map_err(|e| {
            anyhow!(
                "Failed to generate actual parameter for argument: {}. Error: {}",
                argument.formal.get_name(),
                e
            )
        })
    }
    
    /// Builds a list of arguments for a POU call.
    /// This function takes the POU name and a list of actual parameters,
    /// and returns a vector of `Argument` instances that represent pairs of actual and formal parameters.
    /// 
    /// It orders the parameters according to the POU's formal parameter declaration.
    pub(crate) fn build_arguments_list<'idx, 'ast>(gen: &mut ExpressionVisitor<'_, 'idx>, pou_name: &str, actual_parameters: &[&'ast AstNode]) -> Result<Vec<Argument<'idx, 'ast>>> {
        // collect formal and actual parameters
        let formal_parameters =
            gen.index.get_pou_members(pou_name).iter().filter(|e| e.is_parameter()).collect_vec();

        let arguments = if actual_parameters.iter().all(|p| p.is_assignment() || p.is_output_assignment()) {
            // explicit calls in random order: foo(formal := )actual, formal := actual)
            actual_parameters
                .iter()
                .filter_map(|assignment| {
                    if let AstStatement::Assignment(Assignment { left, right: actual, .. })
                    | AstStatement::OutputAssignment(Assignment { left, right: actual, .. }) =
                        assignment.get_stmt()
                    {
                        if let Some(formal) =
                            gen.annotations
                            .get(left)
                            .and_then(|it| it.qualified_name())
                            .and_then(|qname| gen.index.find_fully_qualified_variable(qname))
                        {
                            return Some(Argument::new(formal, actual));
                        }
                    }
                    return None;
                })
                .collect::<Vec<_>>()
        } else {
            // implicit calls in order: foo(actual, actual, actual)
            assert_eq!(formal_parameters.len(), actual_parameters.len());
            // the order is the order of declaration
            formal_parameters
                .iter()
                .zip(actual_parameters.iter())
                .map(|(formal, actual)| Argument::new(formal, actual))
                .collect_vec()
        };
        Ok(arguments)
    }

// passing arguments to a program or function block call
    pub fn program_generate_in_arguments(
        gen: &mut ExpressionVisitor<'_, '_>,
        instance: &PointerValue<'_>,
        arguments: &[Argument<'_, '_>],
    ) -> Result<()> {
        // assign input & inout parameters
        for argument in arguments.iter().filter(|it| it.formal.is_input() || it.formal.is_inout()) {
            let actual = generate_actual_parameter(gen, argument)?;

            // assign it to the formal parameter
            let gep = 
                gen.llvm
                .builder
                .build_struct_gep(
                    *instance,
                    argument.formal.get_location_in_parent(),
                    argument.formal.get_name(),
                )
                .map_err(|_e| anyhow!("Failed to create GEP"))?;

            //TODO: for now we pass everything by value
            let value = gen.as_r_value(actual);
            gen.llvm.builder.build_store(gep, value);
        }
        Ok(())
    }

    pub fn program_generate_out_parameters(
        gen: &mut ExpressionVisitor<'_, '_>,
        instance: &PointerValue<'_>,
        arguments: &[Argument<'_, '_>],
    ) -> Result<()> {
        // assign output parameters
        for argument in arguments.iter().filter(|it| it.formal.is_output()) {
            // get the formal parameters value
            let gep = gen.llvm
                .builder
                .build_struct_gep(
                    *instance,
                    argument.formal.get_location_in_parent(),
                    argument.formal.get_name(),
                )
                .map_err(|_e| anyhow!("Failed to create GEP"))?;

            let loaded_gep = gen.llvm.builder.build_load(gep, "");
            let actual = generate_actual_parameter(gen,argument)?;

            // store
            gen.llvm.builder.build_store(actual.as_pointer_value()?, loaded_gep);
        }
        Ok(())
    }

    pub(crate) fn program_build_call(
        gen: &mut ExpressionVisitor<'_, '_>,
        fv: FunctionValue<'_>,
        instance: &PointerValue<'_>,
        name: &str,
    ) {
        gen.llvm.builder.build_call(fv, &[instance.as_basic_value_enum().into()], name);
    }

impl<'idx, 'ast, 'ink> CallArguments<'idx, 'ast, 'ink> {
    pub fn new(
        annotations: &'idx AstAnnotations,
        index: &'idx Index,
        llvm: &'idx Llvm<'ink>,
        arguments: Vec<Argument<'idx, 'ast>>,
    ) -> Self {
        CallArguments { arguments, annotations, index, llvm }
    }
}

// program calls
impl<'idx, 'ast, 'ink> CallArguments<'idx, 'ast, 'ink> {
    pub fn generate_program_call(
        &self,
        fv: FunctionValue<'_>,
        instance: &PointerValue,
        variable_visitor: &mut ExpressionVisitor<'_, '_>,
    ) -> Result<GeneratedValue<'_>> {
        let input_assignments =
            self.arguments.iter().filter(|it| it.formal.is_input() || it.formal.is_inout()).collect_vec();

        // INPUTs
        self.program_assign_input_arguments(instance, variable_visitor, input_assignments)?;
        self.llvm.builder.build_call(fv, &[instance.as_basic_value_enum().into()], "call"); //todo we should use the function's name here?
                                                                                            // OUTPUTs
        self.program_assign_output_arguments(instance, variable_visitor)?;
        Ok(GeneratedValue::NoValue)
    }

    fn program_assign_output_arguments(
        &self,
        instance: &PointerValue<'_>,
        variable_visitor: &mut ExpressionVisitor<'_, '_>,
    ) -> Result<(), anyhow::Error> {
        for parameter in self.arguments.iter().filter(|it| it.formal.is_output()) {
            // gep the left side and load the value
            let output_variable = self
                .llvm
                .builder
                .build_struct_gep(*instance, parameter.formal.get_location_in_parent(), "")
                .map_err(|_e| anyhow!("Failed to create GEP"))?;
            let value = self.llvm.builder.build_load(output_variable, "");

            // store into the right side
            let target = variable_visitor.generate_expression(parameter.actual)?.as_pointer_value()?;
            self.llvm.builder.build_store(target, value);
        }
        Ok(())
    }

    fn program_assign_input_arguments(
        &self,
        instance: &PointerValue<'_>,
        variable_visitor: &mut ExpressionVisitor<'_, '_>,
        input_assignments: Vec<&Argument<'_, '_>>,
    ) -> Result<(), anyhow::Error> {
        Ok(for argument in input_assignments {
            let actual_value = if argument.formal.is_inout() {
                // if this is an inout, we need to pass a pointer
                variable_visitor.generate_expression(argument.actual)?.as_pointer_value()?.into()
            } else {
                // if this is an input we pass an rvalue
                variable_visitor.generate_r_value(argument.actual)?
            };

            let gep = self
                .llvm
                .builder
                .build_struct_gep(
                    *instance,
                    argument.formal.get_location_in_parent(),
                    argument.formal.get_name(),
                )
                .map_err(|_e| anyhow!("Failed to create GEP"))?;
            self.llvm.builder.build_store(gep, actual_value);
        })
    }
}

impl<'idx, 'ast, 'ink> CallArguments<'idx, 'ast, 'ink> {
    pub fn generate_function_call(
        &self,
        fv: FunctionValue<'ink>,
        generator: &mut ExpressionVisitor<'ink, '_>,
        call_statement_node: &AstNode,
    ) -> Result<GeneratedValue<'ink>> {
        let args = self
            .arguments
            .iter()
            .map(|Argument { actual, formal }| {
                dbg!(&actual);
                dbg!(&formal);
                let actual_hint = self.annotations.get_hint_or_void(&actual, &self.index);

                if actual_hint.is_string() {
                    // this should be passed as a pointer
                    generator
                        .generate_expression(actual)
                        .map_err(|e| anyhow!("Failed to generate expression for argument: {}", e)) //TODO  get rid of diagnostic error
                        .and_then(|it| it.as_pointer_value())
                        .map(|ptr| {
                            self.llvm
                                .builder
                                .build_pointer_cast(
                                    ptr,
                                    self.llvm.context.i8_type().ptr_type(AddressSpace::default()),
                                    "",
                                )
                                .into()
                        })
                } else if actual_hint.is_aggregate_type() {
                    generator
                        .generate_expression(actual)
                        .map_err(|e| anyhow!("Failed to generate expression for argument: {}", e)) //TODO  get rid of diagnostic error
                        .and_then(|it| it.as_pointer_value())
                        .map(|it| it.into())
                } else if formal.is_inout() || formal.is_output() {
                    generator
                        .generate_expression(actual)
                        .map_err(|e| anyhow!("Failed to generate expression for argument: {}", e)) //TODO  get rid of diagnostic error
                        .and_then(|it| it.as_pointer_value())
                        .map(|it| it.into())
                } else {
                    generator
                        .generate_expression(actual)
                        .map(|it| generator.as_r_value(it))
                        .map(|it| it.into())
                        .map_err(|e| anyhow!("Failed to generate expression for argument: {}", e))
                    //TODO  get rid of diagnostic error
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let function_result = self.llvm.builder.build_call(fv, args.as_slice(), "call"); //todo we should use the function's name here?

        // reutrn either the return value or a NoValue
        Ok(function_result
            .try_as_basic_value()
            .left()
            .map(|it| GeneratedValue::RValue((it, call_statement_node.get_id())))
            .unwrap_or_else(|| GeneratedValue::NoValue))
    }
}
