// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::SourceRange,
    codegen::llvm_typesystem,
    index::{ImplementationType, Index},
    resolver::{AnnotationMap, StatementAnnotation},
    typesystem::{Dimension, StringEncoding, INT_SIZE, INT_TYPE, LINT_TYPE},
};
use inkwell::{
    basic_block::BasicBlock,
    types::BasicTypeEnum,
    values::{
        ArrayValue, BasicValue, BasicValueEnum, FloatValue, IntValue, PointerValue, StructValue,
        VectorValue,
    },
    AddressSpace, FloatPredicate, IntPredicate,
};
use std::{collections::HashSet, convert::TryInto};

use crate::{
    ast::{flatten_expression_list, AstStatement, Operator},
    codegen::{
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::{cast_if_needed, get_llvm_int_type},
    },
    compile_error::CompileError,
    typesystem::{DataType, DataTypeInformation},
};

use super::{llvm::Llvm, statement_generator::FunctionContext, struct_generator};

use chrono::{LocalResult, TimeZone, Utc};

/// the generator for expressions
pub struct ExpressionCodeGenerator<'a, 'b> {
    llvm: &'b Llvm<'a>,
    index: &'b Index,
    annotations: &'b AnnotationMap,
    llvm_index: &'b LlvmTypedIndex<'a>,
    /// the current function to create blocks in
    function_context: Option<&'b FunctionContext<'a>>,

    /// the string-prefix to use for temporary variables
    pub temp_variable_prefix: String,
    /// the string-suffix to use for temporary variables
    pub temp_variable_suffix: String,
}

/// context information to generate a parameter
struct ParameterContext<'a, 'b> {
    assignment_statement: &'b AstStatement,
    function_name: &'b str,
    parameter_type: Option<&'b DataType>,
    index: u32,
    parameter_struct: PointerValue<'a>,
}

impl<'a, 'b> ExpressionCodeGenerator<'a, 'b> {
    /// creates a new expression generator
    ///
    /// - `llvm` dependencies used to generate llvm IR
    /// - `index` the index / global symbol table
    /// - `type_hint` an optional type hint for generating literals
    /// - `function_context` the current function to create blocks
    pub fn new(
        llvm: &'b Llvm<'a>,
        index: &'b Index,
        annotations: &'b AnnotationMap,
        llvm_index: &'b LlvmTypedIndex<'a>,
        function_context: &'b FunctionContext<'a>,
    ) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            llvm_index,
            annotations,
            function_context: Some(function_context),
            temp_variable_prefix: "load_".to_string(),
            temp_variable_suffix: "".to_string(),
        }
    }

    /// creates a new expression generator without a function context
    /// this expression generator cannot generate all expressions. It can only generate
    /// expressions that need no blocks (e.g. literals, references, etc.)
    ///
    /// - `llvm` dependencies used to generate llvm IR
    /// - `index` the index / global symbol table
    /// - `type_hint` an optional type hint for generating literals
    pub fn new_context_free(
        llvm: &'b Llvm<'a>,
        index: &'b Index,
        annotations: &'b AnnotationMap,
        llvm_index: &'b LlvmTypedIndex<'a>,
    ) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            llvm_index,
            annotations,
            function_context: None,
            temp_variable_prefix: "load_".to_string(),
            temp_variable_suffix: "".to_string(),
        }
    }

    /// returns the function context or returns a Compile-Error
    fn get_function_context(
        &self,
        statement: &AstStatement,
    ) -> Result<&'b FunctionContext<'a>, CompileError> {
        self.function_context
            .ok_or_else(|| CompileError::missing_function(statement.get_location()))
    }

    /// generates the given expression and returns a TypeAndValue as a result of the
    /// given epxression
    pub fn generate_expression(
        &self,
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let v = self.do_generate_expression(expression)?;

        //see if we need a cast
        if let Some(target_type) = self.annotations.get_type_hint(expression, self.index) {
            let actual_type = self.annotations.get_type_or_void(expression, self.index);
            let v = llvm_typesystem::cast_if_needed(
                self.llvm,
                self.index,
                target_type.get_type_information(),
                v,
                actual_type.get_type_information(),
                expression,
            )?;
            Ok(v)
        } else {
            Ok(v)
        }
    }

    fn do_generate_expression(
        &self,
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let _builder = &self.llvm.builder;

        //see if this is a constant - maybe we can short curcuit this codegen
        if let Some(StatementAnnotation::Variable { qualified_name, .. }) =
            self.annotations.get_annotation(expression)
        {
            if let Some(basic_value_enum) = self.llvm_index.find_constant_value(qualified_name) {
                //this is a constant and we have a value for it
                return Ok(basic_value_enum);
            }
        }

        match expression {
            AstStatement::Reference { name, .. } => {
                let load_name = format!(
                    "{}{}{}",
                    self.temp_variable_prefix, name, self.temp_variable_suffix
                );
                let l_value = self.generate_element_pointer(expression)?;
                Ok(self.llvm.load_pointer(&l_value, load_name.as_str()))
            }
            AstStatement::QualifiedReference { elements, .. } => {
                //If direct access, don't load pointers
                if has_direct_access(expression) {
                    //Split the qualified reference at the last element
                    self.generate_directaccess(elements)
                } else {
                    let l_value = self.generate_element_pointer(expression)?;
                    Ok(self.llvm.load_pointer(&l_value, &self.temp_variable_prefix))
                }
            }
            AstStatement::ArrayAccess { .. } => {
                let l_value = self.generate_element_pointer(expression)?;
                Ok(self.llvm.load_pointer(&l_value, "load_tmpVar"))
            }
            AstStatement::PointerAccess { .. } => {
                let l_value = self.generate_element_pointer(expression)?;
                Ok(self.llvm.load_pointer(&l_value, "load_tmpVar"))
            }
            AstStatement::BinaryExpression {
                left,
                right,
                operator,
                ..
            } => {
                //If OR, or AND handle before generating the statements
                if let Operator::And | Operator::Or = operator {
                    return self.generate_short_circuit_boolean_expression(operator, left, right);
                }

                let ltype = self.get_type_hint_info_for(left)?;
                let rtype = self.get_type_hint_info_for(right)?;
                let result_type = self.get_type_hint_info_for(expression)?;

                if ltype.is_int() && rtype.is_int() {
                    Ok(self.create_llvm_int_binary_expression(
                        operator,
                        self.generate_expression(left)?,
                        self.generate_expression(right)?,
                    ))
                } else if ltype.is_float() && rtype.is_float() {
                    Ok(self.create_llvm_float_binary_expression(
                        operator,
                        self.generate_expression(left)?,
                        self.generate_expression(right)?,
                    ))
                } else {
                    Err(CompileError::codegen_error(
                        format!(
                            "invalid types, cannot generate binary expression for {:?}",
                            result_type
                        ),
                        left.get_location(),
                    ))
                }
            }
            AstStatement::CallStatement {
                operator,
                parameters,
                ..
            } => self.generate_call_statement(operator, parameters),
            AstStatement::UnaryExpression {
                operator, value, ..
            } => self.generate_unary_expression(operator, value),
            //fallback
            _ => self.generate_literal(expression),
        }
    }

    fn generate_directaccess(
        &self,
        elements: &[AstStatement],
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let (last, qualifer) = elements.split_last().unwrap();
        let expression = if qualifer.len() == 1 {
            //Create a single reference
            qualifer.first().unwrap().clone()
        } else {
            let id = qualifer.last().unwrap().get_id();
            AstStatement::QualifiedReference {
                elements: qualifer.to_vec(),
                id,
            }
        };
        //Generate a load for the qualifer
        // a.%b1.%x1
        let value = self.generate_expression(&expression)?;
        let expression_type = self.get_type_hint_info_for(&expression)?;
        if let AstStatement::DirectAccess { access, index, .. } = last {
            let datatype = self.get_type_hint_info_for(last)?;
            //Generate and load the index value
            let rhs = match &**index {
                AstStatement::LiteralInteger { value, .. } => {
                    //Convert into the target literal
                    let bitwidth = access.get_bit_width();
                    let value: u64 = (*value).try_into().unwrap_or_default();
                    let index = bitwidth * value;
                    let rhs = self
                        .llvm_index
                        .get_associated_type(expression_type.get_name())
                        .unwrap()
                        .into_int_type()
                        .const_int(index, false);
                    rhs
                }
                AstStatement::Reference { location, .. } => {
                    //Load the reference
                    let reference = self.generate_expression(index)?;
                    let target_type = self.get_type_hint_info_for(index)?;
                    if reference.is_int_value() {
                        let reference = cast_if_needed(
                            self.llvm,
                            self.index,
                            expression_type,
                            reference,
                            target_type,
                            index,
                        )
                        .map(BasicValueEnum::into_int_value)?;
                        //Multiply by the bitwitdh
                        if access.get_bit_width() > 1 {
                            let bitwidth = reference
                                .get_type()
                                .const_int(access.get_bit_width(), datatype.is_signed_int());

                            self.llvm.builder.build_int_mul(reference, bitwidth, "")
                        } else {
                            reference
                        }
                    } else {
                        return Err(CompileError::casting_error(
                            datatype.get_name(),
                            "Integer Type",
                            location.clone(),
                        ));
                    }
                }
                _ => unreachable!("Unexpected index : {:?}", *index),
            };
            //Shift the qualifer value right by the index value
            let shift = self.llvm.builder.build_right_shift(
                value.into_int_value(),
                rhs,
                expression_type.is_signed_int(),
                "shift",
            );
            //Trunc the result to the get only the target size
            let llvm_target_type = self
                .llvm_index
                .get_associated_type(datatype.get_name())
                .unwrap()
                .into_int_type();
            let result =
                self.llvm
                    .builder
                    .build_int_truncate_or_bit_cast(shift, llvm_target_type, "");
            Ok(result.as_basic_value_enum())
        } else {
            unreachable!()
            // Err(CompileError::codegen_error(
            //     "Bitwise operations not possible".into(),
            //     expression.get_location(),
            // ))
        }
    }

    /// generates a Unary-Expression e.g. -<expr> or !<expr>
    fn generate_unary_expression(
        &self,
        unary_operator: &Operator,
        expression: &AstStatement,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let value = match unary_operator {
            Operator::Not => self.llvm.builder.build_not(
                self.generate_expression(expression)?.into_int_value(),
                "tmpVar",
            ),
            Operator::Minus => self.llvm.builder.build_int_neg(
                self.generate_expression(expression)?.into_int_value(),
                "tmpVar",
            ),
            Operator::Address => {
                //datatype is a pointer to the address
                //value is the address
                return self
                    .generate_element_pointer_for_rec(None, expression)
                    .map(|result| result.as_basic_value_enum());
            }
            _ => unimplemented!(),
        };
        Ok(BasicValueEnum::IntValue(value))
    }

    /// generates the given call-statement <operator>(<parameters>)
    /// returns the result of the call as a TypeAndValue (may be an invalid pointer and void-type for PROGRAMs)
    ///
    /// - `operator` - the expression that points to the callable instance (e.g. a PROGRAM, FUNCTION or FUNCTION_BLOCK instance)
    /// - `parameters` - an optional StatementList of parameters
    fn generate_call_statement(
        &self,
        operator: &AstStatement,
        parameters: &Option<AstStatement>,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let function_context = self.get_function_context(operator)?;
        let instance_and_index_entry = match operator {
            AstStatement::Reference { name, .. } => {
                //Get associated Variable or generate a variable for the type with the same name
                let variable = self.index.find_callable_instance_variable(
                    Some(function_context.linking_context.get_type_name()),
                    &[name],
                );

                let (implementation, callable_reference) = if let Some(variable_instance) = variable
                {
                    let implementation = self
                        .index
                        .find_implementation(variable_instance.get_type_name())
                        .unwrap();
                    (
                        implementation,
                        self.llvm_index
                            .find_loaded_associated_variable_value(
                                variable_instance.get_qualified_name(),
                            )
                            .ok_or_else(|| CompileError::CodeGenError {
                                message: format!("cannot find callable type for {:?}", operator),
                                location: operator.get_location(),
                            })?,
                    )
                } else {
                    let implementation = self.index.find_implementation(name);
                    if let Some(implementation) = implementation {
                        (
                            implementation,
                            self.allocate_function_struct_instance(
                                implementation.get_call_name(),
                                operator,
                            )?,
                        )
                    } else {
                        //Look for a possible action
                        let qualified_name = format!(
                            "{}.{}",
                            function_context.linking_context.get_type_name(),
                            name
                        );
                        let function = function_context.function;
                        let ptr = function.get_first_param().unwrap();
                        (
                            self.index.find_implementation(&qualified_name).unwrap(),
                            ptr.into_pointer_value(),
                        )
                    }
                };

                Ok((None, callable_reference, implementation))
            }
            AstStatement::QualifiedReference { .. } => {
                let loaded_value = self.generate_element_pointer_for_rec(None, operator);
                loaded_value.map(|ptr_value| {
                    self.index
                        .find_implementation(self.annotations.get_call_name(operator).unwrap())
                        .map(|implementation| {
                            let (class_struct, method_struct) = if matches!(
                                implementation.get_implementation_type(),
                                &ImplementationType::Method
                            ) {
                                (
                                    Some(ptr_value),
                                    self.allocate_function_struct_instance(
                                        implementation.get_call_name(),
                                        operator,
                                    )
                                    .unwrap(),
                                )
                            } else {
                                (None, ptr_value)
                            };
                            (class_struct, method_struct, implementation)
                        })
                        .ok_or_else(|| CompileError::CodeGenError {
                            message: format!("cannot generate call statement for {:?}", operator),
                            location: operator.get_location(),
                        })
                })?
            }
            _ => Err(CompileError::CodeGenError {
                message: format!("cannot generate call statement for {:?}", operator),
                location: operator.get_location(),
            }),
        };

        let (class_struct, instance, index_entry) = instance_and_index_entry?;
        let function_name = index_entry.get_call_name();
        //Create parameters for input and output blocks
        let current_f = function_context.function;
        let input_block = self.llvm.context.append_basic_block(current_f, "input");
        let call_block = self.llvm.context.append_basic_block(current_f, "call");
        let output_block = self.llvm.context.append_basic_block(current_f, "output");
        let continue_block = self.llvm.context.append_basic_block(current_f, "continue");
        //First go to the input block
        let builder = &self.llvm.builder;
        builder.build_unconditional_branch(input_block);
        builder.position_at_end(input_block);
        //Generate all parameters, this function may jump to the output block
        let parameters = self.generate_function_parameters(
            function_name,
            class_struct,
            instance,
            parameters,
            &input_block,
            &output_block,
        )?;
        //Generate the label jumps from input to call to output
        builder.build_unconditional_branch(call_block);
        builder.position_at_end(output_block);
        builder.build_unconditional_branch(continue_block);
        builder.position_at_end(call_block);
        let function = self
            .llvm_index
            .find_associated_implementation(function_name) //using the non error option to control the output error
            .ok_or_else(|| CompileError::CodeGenError {
                message: format!(
                    "No callable implementation associated to {:?}",
                    function_name
                ),
                location: operator.get_location(),
            })?;
        //If the target is a function, declare the struct locally
        //Assign all parameters into the struct values
        let call_result = builder
            .build_call(function, &parameters, "call")
            .try_as_basic_value();
        builder.build_unconditional_branch(output_block);
        //Continue here after function call
        builder.position_at_end(continue_block);

        // !! REVIEW !! we return an uninitialized int pointer for void methods :-/
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

    /// generates a new instance of a function called `function_name` and returns a PointerValue to it
    ///
    /// - `function_name` the name of the function as registered in the index
    /// - `context` the statement used to report a possible CompileError on
    fn allocate_function_struct_instance(
        &self,
        function_name: &str,
        context: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        let instance_name = struct_generator::get_pou_instance_variable_name(function_name);
        let function_type = self
            .llvm_index
            .find_associated_type(function_name) //Using find instead of get to control the compile error
            .ok_or_else(|| {
                CompileError::no_type_associated(function_name, context.get_location())
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
    fn generate_function_parameters(
        &self,
        function_name: &str,
        class_struct: Option<PointerValue<'a>>,
        parameter_struct: PointerValue<'a>,
        parameters: &Option<AstStatement>,
        input_block: &BasicBlock,
        output_block: &BasicBlock,
    ) -> Result<Vec<BasicValueEnum<'a>>, CompileError> {
        let mut result = if let Some(class_struct) = class_struct {
            vec![
                class_struct.as_basic_value_enum(),
                parameter_struct.as_basic_value_enum(),
            ]
        } else {
            vec![parameter_struct.as_basic_value_enum()]
        };
        match &parameters {
            Some(AstStatement::ExpressionList { expressions, .. }) => {
                for (index, exp) in expressions.iter().enumerate() {
                    let parameter = self.generate_single_parameter(
                        &ParameterContext {
                            assignment_statement: exp,
                            function_name,
                            parameter_type: None,
                            index: index as u32,
                            parameter_struct,
                        },
                        input_block,
                        output_block,
                    )?;
                    if let Some(parameter) = parameter {
                        result.push(parameter);
                    };
                }
            }
            Some(statement) => {
                let parameter = self.generate_single_parameter(
                    &ParameterContext {
                        assignment_statement: statement,
                        function_name,
                        parameter_type: None,
                        index: 0,
                        parameter_struct,
                    },
                    input_block,
                    output_block,
                )?;
                if let Some(parameter) = parameter {
                    result.push(parameter);
                };
            }
            None => {}
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
    fn generate_single_parameter(
        &self,
        param_context: &ParameterContext,
        input_block: &BasicBlock,
        output_block: &BasicBlock,
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
        let assignment_statement = param_context.assignment_statement;

        let parameter_value = match assignment_statement {
            // explicit call parameter: foo(param := value)
            AstStatement::Assignment { left, right, .. } => {
                self.generate_formal_parameter(
                    param_context,
                    left,
                    right,
                    input_block,
                    output_block,
                )?;
                None
            }
            // foo (param => value)
            AstStatement::OutputAssignment { left, right, .. } => {
                self.generate_output_parameter(param_context, left, right, output_block)?;
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
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
        let builder = &self.llvm.builder;
        let function_name = param_context.function_name;
        let index = param_context.index;
        let parameter_struct = param_context.parameter_struct;
        let parameter_type = param_context.parameter_type;
        if self.index.is_declared_parameter(function_name, index) {
            let pointer_to_param = builder
                .build_struct_gep(parameter_struct, index as u32, "")
                .unwrap();

            let parameter = parameter_type
                .or_else(|| {
                    self.index
                        .find_input_parameter(function_name, index as u32)
                        .and_then(|var| self.index.find_type(var.get_type_name()))
                })
                .map(|var| var.get_type_information())
                .unwrap();
            let generated_exp = if let DataTypeInformation::Pointer {
                auto_deref: true, ..
            } = parameter
            {
                //this is VAR_IN_OUT assignemt, so don't load the value, assign the pointer
                self.generate_element_pointer_for_rec(None, expression)?
                    .as_basic_value_enum()
            } else {
                self.generate_expression(expression)?
            };
            builder.build_store(pointer_to_param, generated_exp);
            Ok(None)
        } else {
            Ok(Some(self.generate_expression(expression)?))
        }
    }

    fn generate_output_parameter(
        &self,
        param_context: &ParameterContext,
        left: &AstStatement,
        right: &AstStatement,
        output_block: &BasicBlock,
    ) -> Result<(), CompileError> {
        let builder = &self.llvm.builder;
        let function_name = param_context.function_name;
        let parameter_struct = param_context.parameter_struct;
        let current_block = builder.get_insert_block().unwrap();
        builder.position_at_end(*output_block);
        if let AstStatement::Reference { name, .. } = &*left {
            let parameter = self.index.find_member(function_name, name).unwrap();
            let index = parameter.get_location_in_parent();
            let param_type = self
                .index
                .find_type(parameter.get_type_name())
                .or_else(|| {
                    self.index
                        .find_input_parameter(function_name, index as u32)
                        .and_then(|var| self.index.find_type(var.get_type_name()))
                })
                .map(|var| var.get_type_information())
                .unwrap();
            //load the function prameter
            let pointer_to_param = builder
                .build_struct_gep(parameter_struct, index as u32, "")
                .unwrap();

            let l_value = self.generate_element_pointer_for_rec(None, right)?;
            let loaded_value = builder.build_load(pointer_to_param, parameter.get_name());
            let value = cast_if_needed(
                self.llvm,
                self.index,
                self.get_type_hint_info_for(right)?,
                loaded_value,
                param_type,
                right,
            )?;
            builder.build_store(l_value, value);
        }
        builder.position_at_end(current_block);
        Ok(())
    }

    fn generate_formal_parameter(
        &self,
        param_context: &ParameterContext,
        left: &AstStatement,
        right: &AstStatement,
        input_block: &BasicBlock,
        output_block: &BasicBlock,
    ) -> Result<(), CompileError> {
        let builder = &self.llvm.builder;
        let function_name = param_context.function_name;
        let parameter_struct = param_context.parameter_struct;
        builder.position_at_end(*input_block);
        if let AstStatement::Reference { name, .. } = &*left {
            let parameter = self.index.find_member(function_name, name).unwrap();
            let index = parameter.get_location_in_parent();
            let param_type = self.index.find_type(parameter.get_type_name());
            self.generate_single_parameter(
                &ParameterContext {
                    assignment_statement: right,
                    function_name,
                    parameter_type: param_type,
                    index,
                    parameter_struct,
                },
                input_block,
                output_block,
            )?;
        };
        Ok(())
    }

    /// generates an gep-statement and returns the resulting pointer and DataTypeInfo
    ///
    /// - `reference_statement` - the statement to load (either a reference, an arrayAccess or a qualifiedReference)
    pub fn generate_element_pointer(
        &self,
        reference_statement: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        let result = match reference_statement {
            AstStatement::Reference { name, .. } => {
                self.create_llvm_pointer_value_for_reference(None, name, reference_statement)
            }

            AstStatement::ArrayAccess {
                reference, access, ..
            } => self.generate_element_pointer_for_array(None, reference, access),
            AstStatement::QualifiedReference { elements, .. } => {
                let mut qualifier: Option<PointerValue> = None;
                for e in elements {
                    qualifier = Some(self.generate_element_pointer_for_rec(qualifier.as_ref(), e)?);
                }
                qualifier.ok_or_else(|| {
                    CompileError::codegen_error(
                        format!("Cannot generate a LValue for {:?}", reference_statement),
                        reference_statement.get_location(),
                    )
                })
            }
            AstStatement::PointerAccess { .. } => {
                self.generate_element_pointer_for_rec(None, reference_statement)
            }
            _ => Err(CompileError::codegen_error(
                format!("Cannot generate a LValue for {:?}", reference_statement),
                reference_statement.get_location(),
            )),
        };

        result.and_then(|it| self.auto_deref_if_necessary(it, reference_statement))
    }

    /// geneartes a gep for the given reference with an optional qualifier
    ///
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x where myStruct is the qualifier for x)
    /// - `name` the name of the reference-name (e.g. myStruct.x where 'x' is the reference-name)
    /// - `context` the statement to obtain the location from when returning an error
    fn create_llvm_pointer_value_for_reference(
        &self,
        qualifier: Option<(&PointerValue<'a>, &str)>,
        name: &str,
        context: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        let offset = &context.get_location();
        let l_value = if let Some((l_value, qualifier_name)) = qualifier {
            if let Some(StatementAnnotation::Variable { qualified_name, .. }) =
                self.annotations.get_annotation(context)
            {
                let segments: Vec<&str> = qualified_name.split('.').collect();
                let (qualifier, segments) = if segments.len() > 1 {
                    (
                        Some(segments[0]),
                        segments.iter().skip(1).copied().collect::<Vec<&str>>(),
                    )
                } else {
                    (None, segments)
                };
                let member = self.index.find_variable(qualifier, &segments[..]);
                let member_location =
                    member
                        .map(|it| it.get_location_in_parent())
                        .ok_or_else(|| {
                            CompileError::invalid_reference(qualified_name, offset.clone())
                        })?;

                let gep = self.llvm.get_member_pointer_from_struct(
                    *l_value,
                    member_location,
                    name,
                    offset,
                )?;

                gep
            } else {
                return Err(CompileError::invalid_reference(
                    &format!("{:}.{:}", qualifier_name, name),
                    offset.clone(),
                ));
            }
        } else {
            //no context

            let type_name = self
                .get_function_context(context)?
                .linking_context
                .get_type_name();

            let variable_index_entry = self
                .index
                .find_variable(Some(type_name), &[name])
                .or_else(|| {
                    let annotation = self.annotations.get(context)?;
                    match annotation {
                        StatementAnnotation::Variable { qualified_name, .. } => {
                            //TODO introduce qualified names!
                            let qualifier = &qualified_name[..qualified_name.rfind('.')?];
                            self.index.find_variable(Some(qualifier), &[name])
                        }
                        _ => None,
                    }
                })
                .ok_or_else(|| CompileError::InvalidReference {
                    reference: name.to_string(),
                    location: offset.clone(),
                })?;
            let accessor_ptr = self
                .llvm_index
                .find_loaded_associated_variable_value(variable_index_entry.get_qualified_name())
                .ok_or_else(|| {
                    CompileError::codegen_error(
                        format!("Cannot generate reference for {:}", name),
                        offset.clone(),
                    )
                })?;

            accessor_ptr
        };
        // self.auto_deref_if_necessary(l_value, context)
        Ok(l_value)
    }

    fn deref(&self, accessor_ptr: PointerValue<'a>) -> PointerValue<'a> {
        self.llvm
            .load_pointer(&accessor_ptr, "deref")
            .into_pointer_value()
    }

    /// automatically derefs an inout variable pointer so it can be used like a normal variable
    ///
    /// # Arguments
    /// - `variable_type` the reference's data type, this type will be used to determine if this variable needs to be auto-derefeferenced (var_in_out)
    /// - `access_ptr` the original pointer value loaded for the reference. will be returned if no auto-deref is necessary
    fn auto_deref_if_necessary(
        &self,
        accessor_ptr: PointerValue<'a>,
        statement: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        if let Some(StatementAnnotation::Variable {
            is_auto_deref: true,
            ..
        }) = self.annotations.get_annotation(statement)
        {
            Ok(self.deref(accessor_ptr))
        } else {
            Ok(accessor_ptr)
        }
    }

    /// generates the access-expression for an array-reference
    /// myArray[array_expression] where array_expression is the access-expression
    ///
    /// - `dimension` the array's dimension
    /// - `access_expression` the expression inside the array-statement
    fn generate_access_for_dimension(
        &self,
        dimension: &Dimension,
        access_expression: &AstStatement,
    ) -> Result<IntValue<'a>, CompileError> {
        let start_offset = dimension
            .start_offset
            .as_int_value(self.index)
            .map_err(|it| CompileError::codegen_error(it, access_expression.get_location()))?;

        let access_value = self.generate_expression(access_expression)?;
        //If start offset is not 0, adjust the current statement with an add operation
        if start_offset != 0 {
            Ok(self.llvm.builder.build_int_sub(
                access_value.into_int_value(),
                self.llvm.i32_type().const_int(start_offset as u64, true), //TODO error handling for cast
                "",
            ))
        } else {
            Ok(access_value.into_int_value())
        }
    }

    /// generates a gep statement for a array-reference with an optional qualifier
    ///
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x[2] where myStruct is the qualifier for x)
    /// - `reference` the reference-statement pointing to the array
    /// - `access` the accessor expression (the expression between the brackets: reference[access])
    fn generate_element_pointer_for_array(
        &self,
        qualifier: Option<&PointerValue<'a>>,
        reference: &AstStatement,
        access: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        self.do_generate_element_pointer_for_array(qualifier, reference, access)
            .map_err(|_| {
                CompileError::codegen_error(
                    "Invalid array access".to_string(),
                    access.get_location(),
                )
            })
    }

    fn do_generate_element_pointer_for_array(
        &self,
        qualifier: Option<&PointerValue<'a>>,
        reference: &AstStatement,
        access: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        //Load the reference
        self.generate_element_pointer_for_rec(qualifier, reference)
            .and_then(|lvalue| {
                if let DataTypeInformation::Array { dimensions, .. } =
                    self.get_type_hint_info_for(reference)?
                {
                    //First 0 is to access the pointer, then we access the array
                    let mut indices = vec![self.llvm.i32_type().const_int(0, false)];

                    //Make sure dimensions match statement list
                    let statements = access.get_as_list();
                    if statements.is_empty() || statements.len() != dimensions.len() {
                        return Err(CompileError::codegen_error(
                            format!(
                                "Mismatched array access : {} -> {} ",
                                statements.len(),
                                dimensions.len()
                            ),
                            access.get_location(),
                        ));
                    }
                    for (i, statement) in statements.iter().enumerate() {
                        indices.push(self.generate_access_for_dimension(&dimensions[i], statement)?)
                    }
                    //Load the access from that reference
                    let pointer =
                        self.llvm
                            .load_array_element(lvalue, indices.as_slice(), "tmpVar")?;

                    return Ok(pointer);
                }
                Err(CompileError::codegen_error(
                    "Invalid array access".to_string(),
                    access.get_location(),
                ))
            })
    }

    /// the entry function for recursive reference-generation (for qualified references)
    ///
    /// - `qualifier` the qualifier (TypeAndPointer) for the given reference-statement
    /// - `reference` the reference to load
    fn generate_element_pointer_for_rec(
        &self,
        qualifier: Option<&PointerValue<'a>>,
        reference: &AstStatement,
    ) -> Result<PointerValue<'a>, CompileError> {
        match reference {
            AstStatement::QualifiedReference { elements, .. } => {
                let mut element_iter = elements.iter();
                let current_element = element_iter.next();
                let mut current_lvalue =
                    self.generate_element_pointer_for_rec(qualifier, current_element.unwrap());

                for it in element_iter {
                    let ctx = current_lvalue?;
                    let context_ptr = ctx;

                    current_lvalue = self.generate_element_pointer_for_rec(Some(&context_ptr), it);
                }
                current_lvalue
            }
            AstStatement::Reference { name, .. } => {
                if let Some(qualifier) = qualifier {
                    //see if there is an action with the current name
                    match self.annotations.get_annotation(reference) {
                        Some(StatementAnnotation::Function { qualified_name, .. }) => {
                            let implementation = self.index.find_implementation(qualified_name);
                            if implementation.is_some() {
                                return Ok(qualifier.to_owned());
                            }
                        }
                        Some(StatementAnnotation::Program { qualified_name, .. }) => {
                            let implementation = self.index.find_implementation(qualified_name);
                            if implementation.is_some() {
                                return Ok(qualifier.to_owned());
                            }
                        }
                        _ => {
                            let qualifier_name = self.get_type_hint_for(reference)?.get_name();
                            let qualified_name = format!("{}.{}", qualifier_name, name);
                            let implementation = self.index.find_implementation(&qualified_name);
                            if implementation.is_some() {
                                return Ok(qualifier.to_owned());
                            }
                        }
                    }
                };
                //Otherwise, load a variable reference
                let x = self
                    .create_llvm_pointer_value_for_reference(
                        qualifier.zip(Some(self.get_annotated_qualifier_for(reference)?)),
                        name,
                        reference,
                    )
                    .and_then(|p| self.auto_deref_if_necessary(p, reference));
                x
            }
            AstStatement::ArrayAccess {
                reference, access, ..
            } => self.generate_element_pointer_for_array(qualifier, reference, access),
            AstStatement::PointerAccess { reference, .. } => {
                let pointer = self.generate_element_pointer_for_rec(qualifier, reference)?;
                Ok(self.deref(pointer))
            }
            _ => Err(CompileError::codegen_error(
                format!("Unsupported Statement {:?}", reference),
                reference.get_location(),
            )),
        }
    }

    /// generates the result of an int/bool binary-expression (+, -, *, /, %, ==)
    ///
    /// - `operator` the binary operator
    /// - `left_value` the left side of the binary expression, needs to be an int-value
    /// - `right_value` the right side of the binary expression, needs to be an int-value
    /// - `target_type` the resulting type
    pub fn create_llvm_int_binary_expression(
        &self,
        operator: &Operator,
        left_value: BasicValueEnum<'a>,
        right_value: BasicValueEnum<'a>,
    ) -> BasicValueEnum<'a> {
        let int_lvalue = left_value.into_int_value();
        let int_rvalue = right_value.into_int_value();

        let value = match operator {
            Operator::Plus => self
                .llvm
                .builder
                .build_int_add(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Minus => self
                .llvm
                .builder
                .build_int_sub(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Multiplication => self
                .llvm
                .builder
                .build_int_mul(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Division => self
                .llvm
                .builder
                .build_int_signed_div(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Modulo => self
                .llvm
                .builder
                .build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar"),
            Operator::Equal => self.llvm.builder.build_int_compare(
                IntPredicate::EQ,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::NotEqual => self.llvm.builder.build_int_compare(
                IntPredicate::NE,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::Less => self.llvm.builder.build_int_compare(
                IntPredicate::SLT,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::Greater => self.llvm.builder.build_int_compare(
                IntPredicate::SGT,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::LessOrEqual => self.llvm.builder.build_int_compare(
                IntPredicate::SLE,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),

            Operator::GreaterOrEqual => self.llvm.builder.build_int_compare(
                IntPredicate::SGE,
                int_lvalue,
                int_rvalue,
                "tmpVar",
            ),
            Operator::Xor => self
                .llvm
                .builder
                .build_xor(int_lvalue, int_rvalue, "tmpVar"),
            _ => unimplemented!(),
        };
        value.into()
    }

    /// generates the result of a float binary-expression (+, -, *, /, %, ==)
    ///
    /// - `operator` the binary operator
    /// - `left_value` the left side of the binary expression, needs to be a float-value
    /// - `right_value` the right side of the binary expression, needs to be a float-value
    /// - `target_type` the resulting type
    fn create_llvm_float_binary_expression(
        &self,
        operator: &Operator,
        lvalue: BasicValueEnum<'a>,
        rvalue: BasicValueEnum<'a>,
    ) -> BasicValueEnum<'a> {
        let float_lvalue = lvalue.into_float_value();
        let float_rvalue = rvalue.into_float_value();

        let value = match operator {
            Operator::Plus => self
                .llvm
                .builder
                .build_float_add(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Minus => self
                .llvm
                .builder
                .build_float_sub(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Multiplication => self
                .llvm
                .builder
                .build_float_mul(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Division => self
                .llvm
                .builder
                .build_float_div(float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Modulo => self
                .llvm
                .builder
                .build_float_rem(float_lvalue, float_rvalue, "tmpVar")
                .into(),

            Operator::Equal => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OEQ, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::NotEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::ONE, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Less => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OLT, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::Greater => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OGT, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::LessOrEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OLE, float_lvalue, float_rvalue, "tmpVar")
                .into(),
            Operator::GreaterOrEqual => self
                .llvm
                .builder
                .build_float_compare(FloatPredicate::OGE, float_lvalue, float_rvalue, "tmpVar")
                .into(),

            _ => unimplemented!(),
        };
        value
    }

    fn generate_numeric_literal(
        &self,
        stmt: &AstStatement,
        number: &str,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let literal_type = self
            .get_type_hint_info_for(stmt)
            .map(DataTypeInformation::get_name)
            .and_then(|name| self.llvm_index.get_associated_type(name))?;
        self.llvm.create_const_numeric(&literal_type, number)
    }

    /// generates the literal statement and returns the resulting value
    ///
    /// - `literal_statement` one of LiteralBool, LiteralInteger, LiteralReal, LiteralString
    pub fn generate_literal(
        &self,
        literal_statement: &AstStatement,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        match literal_statement {
            AstStatement::LiteralBool { value, .. } => self
                .llvm
                .create_const_bool(*value),
            AstStatement::LiteralInteger { value, .. } => {
                self.generate_numeric_literal(literal_statement, value.to_string().as_str())
            }
            AstStatement::LiteralReal { value, .. } => {
                self.generate_numeric_literal(literal_statement, value)
            }
            AstStatement::LiteralDate {
                year,
                month,
                day,
                location,
                ..
            } => self.create_const_int(
                calculate_date_time(*year, *month, *day, 0, 0, 0, 0)
                    .map_err(|op| CompileError::codegen_error(op, location.clone()))?,
            ),
            AstStatement::LiteralDateAndTime {
                year,
                month,
                day,
                hour,
                min,
                sec,
                milli,
                location,
                ..
            } => self.create_const_int(
                calculate_date_time(*year, *month, *day, *hour, *min, *sec, *milli)
                    .map_err(|op| CompileError::codegen_error(op, location.clone()))?,
            ),
            AstStatement::LiteralTimeOfDay {
                hour,
                min,
                sec,
                milli,
                location,
                ..
            } => self.create_const_int(
                calculate_date_time(1970, 1, 1, *hour, *min, *sec, *milli)
                    .map_err(|op| CompileError::codegen_error(op, location.clone()))?,
            ),
            AstStatement::LiteralTime {
                day,
                hour,
                min,
                sec,
                milli,
                micro,
                nano,
                negative,
                ..
            } => self.create_const_int(calculate_time_nano(
                *negative,
                calculate_dhm_time_seconds(*day, *hour, *min, *sec),
                *milli,
                *micro,
                *nano,
            )),

            AstStatement::LiteralString {
                value, location, ..
            } => {
                let expected_type = self.get_type_hint_info_for(literal_statement)?;
                if let DataTypeInformation::String { encoding, .. } = expected_type {
                    match encoding {
                        StringEncoding::Utf8 => self
                            .llvm
                            .create_const_utf8_string(value.as_str()),
                        StringEncoding::Utf16 => self
                            .llvm
                            .create_const_utf16_string(value.as_str()),
                    }
                } else {
                    Err(CompileError::codegen_error(
                        format!(
                            "Cannot generate String-Literal for type {}",
                            expected_type.get_name()
                        ),
                        location.clone(),
                    ))
                }
            }
            AstStatement::LiteralArray {
                elements: Some(elements),
                ..
            } => self.generate_literal_array(elements),
            AstStatement::MultipliedStatement { .. } => {
                self.generate_literal_array(literal_statement)
            }
            AstStatement::LiteralNull { .. } => self.llvm.create_null_ptr(),
            // if there is an expression-list this might be a struct-initialization
            AstStatement::ExpressionList { .. } => {
                self.generate_literal_struct(literal_statement, &literal_statement.get_location())
            }
            // if there is just one assignment, this may be an struct-initialization (TODO this is not very elegant :-/ )
            AstStatement::Assignment { .. } => {
                self.generate_literal_struct(literal_statement, &literal_statement.get_location())
            }
            AstStatement::CastStatement { target, .. } => self.generate_expression(target),
            _ => Err(CompileError::codegen_error(
                format!("Cannot generate Literal for {:?}", literal_statement),
                literal_statement.get_location(),
            )),
        }
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_info_for(
        &self,
        statement: &AstStatement,
    ) -> Result<&DataTypeInformation, CompileError> {
        self.get_type_hint_for(statement)
            .map(DataType::get_type_information)
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_for(&self, statement: &AstStatement) -> Result<&DataType, CompileError> {
        self.annotations
            .get_type_hint(statement, self.index)
            .or_else(|| self.annotations.get_type(statement, self.index))
            .ok_or_else(|| {
                CompileError::codegen_error(
                    format!("no type hint available for {:#?}", statement),
                    statement.get_location(),
                )
            })
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_annotated_qualifier_for(
        &self,
        statement: &AstStatement,
    ) -> Result<&str, CompileError> {
        match self.annotations.get_annotation(statement) {
            Some(StatementAnnotation::Value { resulting_type }) => Ok(resulting_type.as_str()),
            Some(StatementAnnotation::Variable { resulting_type, .. }) => {
                Ok(resulting_type.as_str())
            }
            Some(StatementAnnotation::Function { qualified_name, .. }) => {
                Ok(qualified_name.as_str())
            }
            Some(StatementAnnotation::Program { qualified_name }) => Ok(qualified_name.as_str()),
            _ => Err(CompileError::codegen_error(
                format!("no qualifier available for {:#?}", statement),
                statement.get_location(),
            )),
        }
    }

    /// generates a struct literal value with the given value assignments (ExpressionList)
    fn generate_literal_struct(
        &self,
        assignments: &AstStatement,
        declaration_location: &SourceRange,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        if let DataTypeInformation::Struct {
            name: struct_name,
            member_names,
            ..
        } = self.get_type_hint_info_for(assignments)?
        {
            let mut uninitialized_members: HashSet<&str> =
                member_names.iter().map(|it| it.as_str()).collect();
            let mut member_values: Vec<(u32, BasicValueEnum<'a>)> = Vec::new();
            for assignment in flatten_expression_list(assignments) {
                if let AstStatement::Assignment { left, right, .. } = assignment {
                    if let AstStatement::Reference {
                        name: variable_name,
                        location,
                        ..
                    } = &**left
                    {
                        let member = self
                            .index
                            .find_member(struct_name, variable_name)
                            .ok_or_else(|| {
                                CompileError::invalid_reference(
                                    format!("{}.{}", struct_name, variable_name).as_str(),
                                    location.clone(),
                                )
                            })?;

                        let index_in_parent = member.get_location_in_parent();
                        let value = self.generate_expression(right)?;

                        uninitialized_members.remove(member.get_name());
                        member_values.push((index_in_parent, value));
                    } else {
                        return Err(CompileError::codegen_error(
                            "struct member lvalue required as left operand of assignment"
                                .to_string(),
                            left.get_location(),
                        ));
                    }
                } else {
                    return Err(CompileError::codegen_error("struct literal must consist of explicit assignments in the form of member := value".to_string(), assignment.get_location()));
                }
            }

            //fill the struct with fields we didnt mention yet
            for variable_name in uninitialized_members {
                let member = self
                    .index
                    .find_member(struct_name, variable_name)
                    .ok_or_else(|| {
                        CompileError::invalid_reference(
                            format!("{}.{}", struct_name, variable_name).as_str(),
                            declaration_location.clone(),
                        )
                    })?;

                let initial_value = self
                    .llvm_index
                    .find_associated_variable_value(member.get_qualified_name())
                    // .or_else(|| self.index.find_associated_variable_value(name))
                    .or_else(|| {
                        self.llvm_index
                            .find_associated_initial_value(member.get_type_name())
                    })
                    .unwrap();

                member_values.push((member.get_location_in_parent(), initial_value));
            }
            let struct_type = self
                .llvm_index
                .get_associated_type(struct_name)?
                .into_struct_type();
            if member_values.len() == struct_type.count_fields() as usize {
                member_values.sort_by(|(a, _), (b, _)| a.cmp(b));
                let ordered_values: Vec<BasicValueEnum<'a>> =
                    member_values.iter().map(|(_, v)| *v).collect();

                return Ok(struct_type
                    .const_named_struct(ordered_values.as_slice())
                    .as_basic_value_enum());
            } else {
                return Err(CompileError::codegen_error(
                    format!(
                        "Expected {} fields for Struct {}, but found {}.",
                        struct_type.count_fields(),
                        struct_name,
                        member_values.len()
                    ),
                    assignments.get_location(),
                ));
            }
        } else {
            return Err(CompileError::codegen_error(
                format!("Expected Struct-literal, got {:#?}", assignments),
                assignments.get_location(),
            ));
        }
    }

    /// generates an array literal with the given optional elements (represented as an ExpressionList)
    fn generate_literal_array(
        &self,
        initializer: &AstStatement,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let array_value = self.generate_literal_array_value(
            flatten_expression_list(initializer),
            &initializer.get_location(),
        )?;
        return Ok(array_value.as_basic_value_enum());
    }

    /// constructs an ArrayValue (returned as a BasicValueEnum) of the given element-literals constructing an array-value of the
    /// type described by inner_array_type.
    ///
    /// passing an epxression-lists with LiteralIntegers and inner_array_type is INT-description will return an
    /// i16-array-value
    fn generate_literal_array_value(
        &self,
        elements: Vec<&AstStatement>,
        location: &SourceRange,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let inner_type = elements
            .first()
            .ok_or_else(|| {
                CompileError::codegen_error("Cannot generate empty array".into(), location.clone())
            }) //TODO
            .and_then(|it| self.get_type_hint_info_for(it))?;

        let llvm_type = self.llvm_index.get_associated_type(inner_type.get_name())?;

        let mut v = Vec::new();
        for e in elements {
            //generate with correct type hint
            let value = self.generate_literal(e)?;
            v.push(value.as_basic_value_enum());
        }

        //TODO Validation: fail with compile-error if value cannot be converted into... correctly
        let array_value = match llvm_type {
            BasicTypeEnum::ArrayType(_) => llvm_type.into_array_type().const_array(
                v.iter()
                    .map(|it| it.into_array_value())
                    .collect::<Vec<ArrayValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::FloatType(_) => llvm_type.into_float_type().const_array(
                v.iter()
                    .map(|it| it.into_float_value())
                    .collect::<Vec<FloatValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::IntType(_) => llvm_type.into_int_type().const_array(
                v.iter()
                    .map(|it| it.into_int_value())
                    .collect::<Vec<IntValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::PointerType(_) => llvm_type.into_pointer_type().const_array(
                v.iter()
                    .map(|it| it.into_pointer_value())
                    .collect::<Vec<PointerValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::StructType(_) => llvm_type.into_struct_type().const_array(
                v.iter()
                    .map(|it| it.into_struct_value())
                    .collect::<Vec<StructValue>>()
                    .as_slice(),
            ),
            BasicTypeEnum::VectorType(_) => llvm_type.into_vector_type().const_array(
                v.iter()
                    .map(|it| it.into_vector_value())
                    .collect::<Vec<VectorValue>>()
                    .as_slice(),
            ),
        };
        Ok(array_value.as_basic_value_enum())
    }

    /// generates a phi-expression (&& or || expression) with respect to short-circuit evaluation
    ///
    /// - `operator` AND or OR
    /// - `left` the left side of the expression
    /// - `right` the right side of the expression
    pub fn generate_short_circuit_boolean_expression(
        &self,
        operator: &Operator,
        left: &AstStatement,
        right: &AstStatement,
    ) -> Result<BasicValueEnum<'a>, CompileError> {
        let builder = &self.llvm.builder;
        let function = self.get_function_context(left)?.function;

        let right_branch = self.llvm.context.append_basic_block(function, "");
        let continue_branch = self.llvm.context.append_basic_block(function, "");

        let left_type = self.get_type_hint_for(left)?;
        let left_value = self.generate_expression(left)?;

        let final_left_block = builder.get_insert_block().unwrap();
        let left_llvm_type = self.llvm_index.get_associated_type(left_type.get_name())?;
        //Compare left to 0
        let lhs = builder.build_int_compare(
            IntPredicate::NE,
            left_value.into_int_value(),
            left_llvm_type.into_int_type().const_int(0, false),
            "",
        );
        match operator {
            Operator::Or => builder.build_conditional_branch(lhs, continue_branch, right_branch),
            Operator::And => builder.build_conditional_branch(lhs, right_branch, continue_branch),
            _ => {
                return Err(CompileError::codegen_error(
                    format!("Cannot generate phi-expression for operator {:}", operator),
                    left.get_location(),
                ))
            }
        };

        builder.position_at_end(right_branch);
        let (right_type, right_value) = (
            self.get_type_hint_for(right)?,
            self.generate_expression(right)?,
        );
        let final_right_block = builder.get_insert_block().unwrap();
        let rhs = right_value;
        builder.build_unconditional_branch(continue_branch);

        builder.position_at_end(continue_branch);
        //Generate phi
        let target_type = if left_type.get_type_information().get_size()
            > right_type.get_type_information().get_size()
        {
            left_type
        } else {
            right_type
        };
        let llvm_target_type = self
            .llvm_index
            .get_associated_type(target_type.get_name())?;
        let phi_value = builder.build_phi(llvm_target_type, "");
        phi_value.add_incoming(&[
            (&left_value.into_int_value(), final_left_block),
            (&rhs, final_right_block),
        ]);

        Ok(phi_value.as_basic_value())
    }

    fn create_const_int(&self, value: i64) -> Result<BasicValueEnum<'a>, CompileError> {
        let value = self.llvm.create_const_numeric(
            &self.llvm_index.get_associated_type(LINT_TYPE)?,
            value.to_string().as_str(),
        )?;
        Ok(value)
    }
}

/// calculates the seconds in the given days, hours minutes and seconds
fn calculate_dhm_time_seconds(day: f64, hour: f64, min: f64, sec: f64) -> f64 {
    let hours = day * 24_f64 + hour;
    let mins = hours * 60_f64 + min;
    mins * 60_f64 + sec
}

/// calculates the nanos in the given seconds, millis, micros and nano/**
fn calculate_time_nano(negative: bool, sec: f64, milli: f64, micro: f64, nano: u32) -> i64 {
    let millis = sec * 1000_f64 + milli;
    let micro = millis * 1000_f64 + micro;
    let nano = micro * 1000_f64 + nano as f64;
    //go to full micro
    let nanos = (nano).round() as i64;

    if negative {
        -nanos
    } else {
        nanos
    }
}

/// calculates the milliseconds since 1970-01-01-00:00:00 for the given
/// point in time
fn calculate_date_time(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    milli: u32,
) -> Result<i64, String> {
    if let LocalResult::Single(date_time) = Utc
        .ymd_opt(year, month, day)
        .and_hms_milli_opt(hour, min, sec, milli)
    {
        return Ok(date_time.timestamp_millis());
    }
    Err(format!(
        "Invalid Date {}-{}-{}-{}:{}:{}.{}",
        year, month, day, hour, min, sec, milli
    ))
}

/// Returns true if the current statement has a return access.
fn has_direct_access(statement: &AstStatement) -> bool {
    if let AstStatement::QualifiedReference { elements, .. } = statement {
        matches!(elements.last(), Some(AstStatement::DirectAccess { .. }))
    } else {
        false
    }
}
