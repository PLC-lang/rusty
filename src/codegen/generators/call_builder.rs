use inkwell::{
    values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, PointerValue},
    AddressSpace,
};

use crate::{
    ast::{self, AstStatement},
    codegen::{
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{cast_if_needed, get_llvm_int_type},
    },
    diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR},
    index::{ImplementationIndexEntry, ImplementationType, Index},
    resolver::AstAnnotations,
    typesystem::{DataType, DataTypeInformation, INT_SIZE, INT_TYPE},
};

use super::{
    expression_generator::ExpressionCodeGenerator, llvm::Llvm, statement_generator::FunctionContext,
};

struct ParameterContext<'a, 'b> {
    assignment_statement: &'b AstStatement,
    function_name: &'b str,
    parameter_type: Option<&'b DataType>,
    index: u32,
    parameter_struct: PointerValue<'a>,
}

pub struct InstanceCallStatementBuilder<'ink, 'b> {
    llvm: &'b Llvm<'ink>,
    index: &'b Index,
    annotations: &'b AstAnnotations,
    llvm_index: &'b LlvmTypedIndex<'ink>,
    /// the current function to create blocks in
    function_context: &'b FunctionContext<'ink>,
}

impl<'ink, 'b> InstanceCallStatementBuilder<'ink, 'b> {
    /// creates a new call_statement_builder
    ///
    /// - `llvm` dependencies used to generate llvm IR
    /// - `index` the index / global symbol table
    /// - `type_hint` an optional type hint for generating literals
    /// - `function_context` the current function to create blocks
    pub fn new(
        llvm: &'b Llvm<'ink>,
        index: &'b Index,
        annotations: &'b AstAnnotations,
        llvm_index: &'b LlvmTypedIndex<'ink>,
        function_context: &'b FunctionContext<'ink>,
    ) -> InstanceCallStatementBuilder<'ink, 'b> {
        InstanceCallStatementBuilder {
            llvm,
            index,
            llvm_index,
            annotations,
            function_context,
        }
    }

    /// generates a new instance of a function called `function_name` and returns a PointerValue to it
    ///
    /// - `function_name` the name of the function as registered in the index
    /// - `context` the statement used to report a possible Diagnostic on
    fn allocate_function_struct_instance(
        &self,
        function_name: &str,
        context: &AstStatement,
    ) -> Result<PointerValue<'ink>, Diagnostic> {
        let instance_name = format!("{}_instance", function_name);
        let function_type = self
            .llvm_index
            .find_associated_pou_type(function_name) //Using find instead of get to control the compile error
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    &format!("No type associated with '{:}'", instance_name),
                    context.get_location(),
                )
            })?;

        Ok(self
            .llvm
            .create_local_variable(&instance_name, &function_type))
    }

    /// generates the assignments of a function-call's parameters
    /// the call parameters are passed to the function using a struct-instance with all the parameters
    ///
    /// - `function_name` the name of the function we're calling
    /// - `parameter_struct` a pointer to a struct-instance that holds all function-parameters
    /// - `input_block` the block to generate the input-assignments into
    /// - `output_block` the block to generate the output-assignments into
    fn generate_input_function_parameters(
        &self,
        function_name: &str,
        class_struct: Option<PointerValue<'ink>>,
        parameter_struct: PointerValue<'ink>,
        parameters: &Option<AstStatement>,
    ) -> Result<Vec<BasicMetadataValueEnum<'ink>>, Diagnostic> {
        let mut result = class_struct
            .map(|class_struct| {
                vec![
                    class_struct.as_basic_value_enum().into(),
                    parameter_struct.as_basic_value_enum().into(),
                ]
            })
            .unwrap_or_else(|| vec![parameter_struct.as_basic_value_enum().into()]);

        let expressions = parameters
            .as_ref()
            .map(ast::flatten_expression_list)
            .unwrap_or_else(std::vec::Vec::new);

        for (index, exp) in expressions.iter().enumerate() {
            let parameter = self.generate_single_input_parameter(&ParameterContext {
                assignment_statement: exp,
                function_name,
                parameter_type: None,
                index: index as u32,
                parameter_struct,
            })?;
            if let Some(parameter) = parameter {
                result.push(parameter.into());
            };
        }
        Ok(result)
    }

    /// generates an assignemnt of a single call's parameter
    ///
    /// - `assignment_statement' the parameter-assignment, either an AssignmentStatement, an OutputAssignmentStatement or an expression
    /// - `function_name` the name of the callable
    /// - `parameter_type` the datatype of the parameter
    /// - `index` the index of the parameter (0 for first parameter, 1 for the next one, etc.)
    /// - `parameter_struct' a pointer to a struct-instance that holds all function-parameters
    /// - `input_block` the block to generate the input-assignments into
    /// - `output_block` the block to generate the output-assignments into
    fn generate_single_input_parameter(
        &self,
        param_context: &ParameterContext,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let assignment_statement = param_context.assignment_statement;

        let parameter_value = match assignment_statement {
            // explicit call parameter: foo(param := value)
            AstStatement::Assignment { left, right, .. } => {
                self.generate_formal_parameter(param_context, left, right)?;
                None
            }
            // foo (param => value)
            AstStatement::OutputAssignment { .. } => {
                //ignore here
                None
            }
            // foo(x)
            _ => self.generate_nameless_parameter(param_context, assignment_statement)?,
        };

        Ok(parameter_value)
    }

    fn generate_nameless_parameter(
        &self,
        param_context: &ParameterContext,
        expression: &AstStatement,
    ) -> Result<Option<BasicValueEnum<'ink>>, Diagnostic> {
        let builder = &self.llvm.builder;
        let function_name = param_context.function_name;
        let index = param_context.index;
        let parameter_struct = param_context.parameter_struct;
        let parameter_type = param_context.parameter_type;
        if self.index.is_declared_parameter(function_name, index) {
            let pointer_to_param = builder
                .build_struct_gep(parameter_struct, index as u32, "")
                .map_err(|_| {
                    Diagnostic::codegen_error(
                        &format!("Cannot build generate parameter: {:#?}", expression),
                        expression.get_location(),
                    )
                })?;

            let parameter = parameter_type
                .or_else(|| {
                    self.index
                        .find_input_parameter(function_name, index as u32)
                        .and_then(|var| self.index.find_effective_type(var.get_type_name()))
                })
                .map(|var| var.get_type_information())
                .unwrap_or_else(|| self.index.get_void_type().get_type_information());

            let exp_gen = ExpressionCodeGenerator::new(
                self.llvm,
                self.index,
                self.annotations,
                self.llvm_index,
                self.function_context,
            );
            if let DataTypeInformation::Pointer {
                auto_deref: true, ..
            } = parameter
            {
                //this is VAR_IN_OUT assignemt, so don't load the value, assign the pointer
                let generated_exp = exp_gen
                    .generate_element_pointer(expression)?
                    .as_basic_value_enum();

                builder.build_store(pointer_to_param, generated_exp);
            } else {
                exp_gen.generate_store(parameter, expression, pointer_to_param)?;
            };

            Ok(None)
        } else {
            let exp_gen = ExpressionCodeGenerator::new(
                self.llvm,
                self.index,
                self.annotations,
                self.llvm_index,
                self.function_context,
            );
            Ok(Some(exp_gen.generate_expression(expression)?))
        }
    }

    fn generate_formal_parameter(
        &self,
        param_context: &ParameterContext,
        left: &AstStatement,
        right: &AstStatement,
    ) -> Result<(), Diagnostic> {
        let function_name = param_context.function_name;
        let parameter_struct = param_context.parameter_struct;
        if let AstStatement::Reference { name, .. } = left {
            let parameter = self
                .index
                .find_member(function_name, name)
                .ok_or_else(|| Diagnostic::unresolved_reference(name, left.get_location()))?;
            let index = parameter.get_location_in_parent();
            let param_type = self.index.find_effective_type(parameter.get_type_name());
            self.generate_single_input_parameter(&ParameterContext {
                assignment_statement: right,
                function_name,
                parameter_type: param_type,
                index,
                parameter_struct,
            })?;
        };
        Ok(())
    }

    /// generates the output assignments of a function-call's parameters
    /// the call parameters are passed to the function using a struct-instance with all the parameters
    ///
    /// - `function_name` the name of the function we're calling
    /// - `parameter_struct` a pointer to a struct-instance that holds all function-parameters
    /// - `input_block` the block to generate the input-assignments into
    fn generate_output_function_parameters(
        &self,
        function_name: &str,
        parameter_struct: PointerValue,
        parameters: &Option<AstStatement>,
    ) -> Result<(), Diagnostic> {
        let expressions = parameters
            .as_ref()
            .map(ast::flatten_expression_list)
            .unwrap_or_else(std::vec::Vec::new);

        for exp in expressions.iter() {
            if let AstStatement::OutputAssignment { left, right, .. } = exp {
                self.generate_output_parameter(function_name, parameter_struct, left, right)?;
            }
        }
        Ok(())
    }

    fn generate_output_parameter(
        &self,
        function_name: &str,
        parameter_struct: PointerValue,
        left: &AstStatement,
        right: &AstStatement,
    ) -> Result<(), Diagnostic> {
        let builder = &self.llvm.builder;

        // (output => ) output assignments are optional, in this case  ignore codegen
        if !matches!(right, AstStatement::EmptyStatement { .. }) {
            if let AstStatement::Reference { name, .. } = left {
                let parameter = self
                    .index
                    .find_member(function_name, name)
                    .ok_or_else(|| Diagnostic::unresolved_reference(name, left.get_location()))?;
                let index = parameter.get_location_in_parent();
                let param_type = self
                    .index
                    .find_effective_type(parameter.get_type_name())
                    .or_else(|| {
                        self.index
                            .find_input_parameter(function_name, index as u32)
                            .and_then(|var| self.index.find_effective_type(var.get_type_name()))
                    })
                    .ok_or_else(|| {
                        Diagnostic::unknown_type(parameter.get_type_name(), left.get_location())
                    })?;
                //load the function prameter
                let pointer_to_param = builder
                    .build_struct_gep(parameter_struct, index as u32, "")
                    .expect(INTERNAL_LLVM_ERROR);

                let exp_gen = ExpressionCodeGenerator::new(
                    self.llvm,
                    self.index,
                    self.annotations,
                    self.llvm_index,
                    self.function_context,
                );
                let l_value = exp_gen.generate_element_pointer(right)?;
                let loaded_value = builder.build_load(pointer_to_param, parameter.get_name());
                let value = cast_if_needed(
                    self.llvm,
                    self.index,
                    self.llvm_index,
                    exp_gen.get_type_hint_for(right)?,
                    loaded_value,
                    param_type,
                    right,
                )?;
                builder.build_store(l_value, value);
            }
        }
        Ok(())
    }

    /// generates a callstatement to a function
    ///
    /// function-call statements pass arguments in a flat way
    pub fn generate_function_call_statement(
        &self,
        function_impl: &ImplementationIndexEntry,
        parameters: &Option<AstStatement>,
        context: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        // generate all parameters

        let exp_gen = ExpressionCodeGenerator::new(
            self.llvm,
            self.index,
            self.annotations,
            self.llvm_index,
            self.function_context,
        );

        // generate values for all parameters
        let param_values = parameters.as_ref()
            .map(|it| ast::flatten_expression_list(it))
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|it| exp_gen.generate_expression(it).map(|it| it.into()))
            .collect::<Result<Vec<_>, _>>()?;

        //generate the call
        let fun_name = function_impl.get_call_name();
        self.generate_function_call(fun_name, param_values, context)
    }

    /// generates a callstatement to a instance (Program, Functionblock or Class)
    ///
    /// instance-calls pass a single instance-struct
    pub fn generate_instance_call_statement(
        &self,
        pou_target: &ImplementationIndexEntry,
        parameters: &Option<AstStatement>,
        operator: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let exp_gen = ExpressionCodeGenerator::new(
            self.llvm,
            self.index,
            self.annotations,
            self.llvm_index,
            self.function_context,
        );
        let (class_ptr, call_ptr) = match pou_target {
            ImplementationIndexEntry {
                implementation_type: ImplementationType::Function,
                ..
            } => {
                panic!("Function passed to generate_instance_call_statement"); //this should eventually be removed when submitted
                                                                               // let call_ptr = self
                                                                               //     .allocate_function_struct_instance(pou_target.get_call_name(), operator)?;
                                                                               // (None, call_ptr)
            }
            ImplementationIndexEntry {
                implementation_type: ImplementationType::Method,
                ..
            } => {
                let class_ptr = exp_gen.generate_element_pointer(operator)?;
                let call_ptr =
                    self.allocate_function_struct_instance(pou_target.get_call_name(), operator)?;
                (Some(class_ptr), call_ptr)
            }
            ImplementationIndexEntry {
                implementation_type: ImplementationType::Action,
                ..
            } if matches!(operator, AstStatement::Reference { .. }) => {
                //Special handling for local actions, get the parameter from the function context
                if let Some(call_ptr) = self.function_context.function.get_first_param() {
                    (None, call_ptr.into_pointer_value())
                } else {
                    return Err(Diagnostic::codegen_error(
                        &format!("cannot find parameter for {}", pou_target.get_call_name()),
                        operator.get_location(),
                    ));
                }
            }
            _ => {
                let class_ptr = exp_gen.generate_element_pointer(operator)?;
                (None, class_ptr)
            }
        };

        let (class_struct, instance, index_entry) = (class_ptr, call_ptr, pou_target);
        let function_name = index_entry.get_call_name();
        //First go to the input block
        //Generate all parameters, this function may jump to the output block
        let parameters_data = self.generate_input_function_parameters(
            function_name,
            class_struct,
            instance,
            parameters,
        )?;

        let call_result = self.generate_function_call(function_name, parameters_data, operator)?;
        //build output-parameters
        self.generate_output_function_parameters(function_name, instance, parameters)?;

        Ok(call_result)
    }

    fn generate_function_call(
        &self,
        function_name: &str,
        parameters_data: Vec<BasicMetadataValueEnum<'ink>>,
        context: &AstStatement,
    ) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let function = self
            .llvm_index
            .find_associated_implementation(function_name) //using the non error option to control the output error
            .ok_or_else(|| {
                Diagnostic::codegen_error(
                    &format!(
                        "No callable implementation associated to {:?}",
                        function_name
                    ),
                    context.get_location(),
                )
            })?;
        let call_result = self
            .llvm
            .builder
            .build_call(function, &parameters_data, "call")
            .try_as_basic_value();

        // we return an uninitialized int pointer for void methods :-/
        // dont deref it!!
        let value = call_result.either(Ok, |_| {
            get_llvm_int_type(self.llvm.context, INT_SIZE, INT_TYPE).map(|int| {
                int.ptr_type(AddressSpace::Const)
                    .const_null()
                    .as_basic_value_enum()
            })
        })?;
        Ok(value)
    }

}
