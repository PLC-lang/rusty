/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use inkwell::{AddressSpace, FloatPredicate, IntPredicate, basic_block::BasicBlock, types::{BasicType, BasicTypeEnum}, values::{ArrayValue, BasicValue, BasicValueEnum, FloatValue, IntValue, PointerValue, StructValue, VectorValue}};
use std::ops::Range;

use crate::{ast::{Operator, Statement, flatten_expression_list}, codegen::{TypeAndPointer, TypeAndValue, typesystem}, compile_error::{CompileError}, index::{DataTypeIndexEntry, DataTypeInformation, Dimension, Index, VariableIndexEntry}};

use super::{llvm::LLVM, statement_generator::FunctionContext, struct_generator};

/// the generator for expressions
pub struct ExpressionCodeGenerator<'a, 'b> {
    llvm: &'b LLVM<'a>,
    index: &'b Index<'a>,
    /// an optional type hint for generating literals
    type_hint: Option<DataTypeInformation<'a>>,
    /// the current function to create blocks in
    function_context: Option<&'b FunctionContext<'a>>,

    /// the string-prefix to use for temporary variables
    pub temp_variable_prefix: String,
    /// the string-suffix to use for temporary variables
    pub temp_variable_suffix: String,

}

impl<'a, 'b> ExpressionCodeGenerator<'a, 'b> {
    /// creates a new expression generator
    ///
    /// - `llvm` dependencies used to generate llvm IR
    /// - `index` the index / global symbol table
    /// - `type_hint` an optional type hint for generating literals
    /// - `function_context` the current function to create blocks
    pub fn new(
        llvm: &'b LLVM<'a>,
        index: &'b Index<'a>,
        type_hint: Option<DataTypeInformation<'a>>,
        function_context: &'b FunctionContext<'a>,
    ) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            type_hint,
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
        llvm: &'b LLVM<'a>,
        index: &'b Index<'a>,
        type_hint: Option<DataTypeInformation<'a>>,
    ) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator {
            llvm,
            index,
            type_hint,
            function_context: None,
            temp_variable_prefix: "load_".to_string(),
            temp_variable_suffix: "".to_string(),
        }
    }

    pub fn morph_to_typed(&self, type_hint: &DataTypeInformation<'a>) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator {
            llvm: self.llvm,
            index: self.index,
            type_hint: Some(type_hint.clone()),
            function_context: self.function_context ,
            temp_variable_prefix: self.temp_variable_prefix.clone(),
            temp_variable_suffix: self.temp_variable_suffix.clone(),
        }
    }

    /// returns the function context or returns a Compile-Error
    fn get_function_context(&self, statement: &Statement) -> Result<&FunctionContext, CompileError> {
        self.function_context.ok_or_else(||
            CompileError::missing_function(statement.get_location()))
    }

    /// returns an option with the current type_hint as a BasicTypeEnum
    fn get_type_context(&self) -> Option<BasicTypeEnum<'a>> {
        self.type_hint.as_ref().and_then(|it| Some(it.get_type()))
    }

    /// generates the given expression and returns a TypeAndValue as a result of the
    /// given epxression
    pub fn generate_expression(
        &self,
        expression: &Statement
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let builder = &self.llvm.builder;
        match expression {
            Statement::Reference { name, .. } => {
                let load_name = format!("{}{}{}", self.temp_variable_prefix, name, self.temp_variable_suffix);
                let l_value = self.generate_load_for(expression)?;
                Ok(self.llvm.load_pointer(&l_value, load_name.as_str()))
            },
            Statement::QualifiedReference { .. } => {
                let l_value = self.generate_load_for(expression)?;
                Ok(self.llvm.load_pointer(&l_value, &self.temp_variable_prefix))
            },
            Statement::ArrayAccess { .. } => {
                let l_value = self.generate_load_for(expression)?;
                Ok(self.llvm.load_pointer(&l_value, "load_tmpVar"))
            }
            Statement::BinaryExpression {
                left,
                right,
                operator,
            } => {

                //If OR, or AND handle before generating the statements
                match operator {
                    Operator::And | Operator::Or => 
                        return self.generate_short_circuit_boolean_expression(operator, left, right),
                    _ => {}
                }

                let left_type_and_value = self.generate_expression(left)?;
                let right_type_and_value = self.generate_expression(right)?;

                let (common_type, left_value, right_value) = typesystem::promote_if_needed(
                    builder,
                    &left_type_and_value,
                    &right_type_and_value,
                    self.index,
                );

                if common_type.is_int() {
                    Ok(self.create_llvm_int_binary_expression(
                        operator,
                        left_value,
                        right_value,
                        &common_type))
                } else if common_type.is_float() {
                    Ok(self.create_llvm_float_binary_expression(
                        operator,
                        left_value,
                        right_value,
                        &common_type))
                } else {
                    let message = format!("invalid types, cannot generate binary expression for {:?}", common_type);
                    Err(CompileError::codegen_error(message, left.get_location()))
                }
            },
            Statement::CallStatement{ operator, parameters, ..} => {
                self.generate_call_statement(operator, parameters)
            },
            Statement::UnaryExpression { operator, value, ..} => {
                self.generate_unary_expression(operator, value)
            },
            //fallback
            _ => self.generate_literal(expression),
        }
    }

    /// generates a Unary-Expression e.g. -<expr> or !<expr>
    fn generate_unary_expression(
        &self,
        unary_operator: &Operator,
        expression: &Statement,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        let (data_type, loaded_value) = self.generate_expression(expression)?;
        let (data_type, value) = match unary_operator {
            Operator::Not => (
                data_type,
                self.llvm.builder
                    .build_not(loaded_value.into_int_value(), "tmpVar"),
            ),
            Operator::Minus => (
                data_type,
                self.llvm.builder
                    .build_int_neg(loaded_value.into_int_value(), "tmpVar"),
            ),
            _ => unimplemented!(),
        };
        Ok((data_type, BasicValueEnum::IntValue(value)))
    }

    /// generates the given call-statement <operator>(<parameters>)
    /// returns the result of the call as a TypeAndValue (may be an invalid pointer and void-type for PROGRAMs)
    ///
    /// - `operator` - the expression that points to the callable instance (e.g. a PROGRAM, FUNCTION or FUNCTION_BLOCK instance)
    /// - `parameters` - an optional StatementList of parameters
    fn generate_call_statement(
        &self,
        operator: &Statement,
        parameters: &Option<Statement>) -> Result<TypeAndValue<'a>, CompileError> {

        let function_context = self.get_function_context(operator)?;
        let instance_and_index_entry = match operator {
            Statement::Reference { name, .. } => {
                //Get associated Variable or generate a variable for the type with the same name
                let variable = self.index
                    .find_callable_instance_variable(Some(function_context.linking_context.as_str()), &[name.clone()]);
        
                let callable_reference = if let Some(variable_instance) = variable {
                    variable_instance.get_generated_reference()
                        .ok_or_else(||
                            CompileError::CodeGenError{ message: format!("cannot find callable type for {:?}", operator), location: operator.get_location().clone() })?
                } else {
                    self.allocate_function_struct_instance(&name, operator)?
                };
                        
                let call_name = variable
                    .map(|it| it.get_type_name()) // we called f() --> look for f's datatype
                    .or(Some(&name)); // we didnt call a variable ([0so we treat the string as the function's name

                let index_entry = self.index.get_type(call_name.unwrap())?;
                Ok((callable_reference, index_entry))
            }
            _ => Err(CompileError::CodeGenError{ message: format!("cannot generate call statement for {:?}", operator), location: operator.get_location().clone() }),
        };

        let (instance, index_entry) = instance_and_index_entry?;
        let function_name = index_entry.get_name();
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
        self.generate_function_parameters(function_name, instance, parameters, &input_block, &output_block)?;
        //Generate the label jumps from input to call to output 
        builder.build_unconditional_branch(call_block);
        builder.position_at_end(output_block);
        builder.build_unconditional_branch(continue_block);
        builder.position_at_end(call_block);
        let return_type = self
            .index
            .find_member(function_name, function_name)
            .map(VariableIndexEntry::get_type_name)
            .or(Some("__VOID"))
            .and_then(|it| self.index.find_type_information(it));
        let function = index_entry.get_implementation()
                .ok_or_else(|| 
                    CompileError::CodeGenError{ message: format!("No callable implementation associated to {:?}", function_name), location: operator.get_location().clone() })?;
        let call_result = 
        //If the target is a function, declare the struct locally
        //Assign all parameters into the struct values
        builder
            .build_call(function, &[instance.as_basic_value_enum()], "call")
            .try_as_basic_value();
        builder.build_unconditional_branch(output_block);
        //Continue here after function call
        builder.position_at_end(continue_block);

        // !! TODO REVIEW !! we return an uninitialized int pointer for void methods :-/
        // dont touch it!!
        let value = call_result.either(
            |value| Ok(value), 
            |_| self.index.get_type_information("INT")
                    .map(|it| it.get_type())
                    .map(|it| it.ptr_type(AddressSpace::Const).const_null().as_basic_value_enum()))?;

        return Ok(( return_type.unwrap(), value ));
    }

    /// generates a new instance of a function called `function_name` and returns a PointerValue to it
    ///
    /// - `function_name` the name of the function as registered in the index
    /// - `context` the statement used to report a possible CompileError on
    fn allocate_function_struct_instance(&self, function_name: &str, context: &Statement) -> Result<PointerValue<'a>, CompileError> {
        let instance_name = struct_generator::get_pou_instance_variable_name(function_name);
        let function_type = self.index.get_type(function_name)?
                                .get_type()
                                .ok_or_else(|| CompileError::no_type_associated(function_name, context.get_location().clone()))?;

        Ok(self.llvm.create_local_variable(&instance_name, &function_type))
    }

    /// generates the assignments of a function-call's parameters
    /// the call parameters are passed to the function using a struct-instance with all the parameters
    ///
    /// - `function_name` the name of the function we're calling
    /// - `parameter_struct' a pointer to a struct-instance that holds all function-parameters
    /// - `input_block` the block to generate the input-assignments into
    /// - `output_block` the block to generate the output-assignments into
    fn generate_function_parameters(
        &self,
        function_name: &str,
        parameter_struct: PointerValue<'a>,
        parameters: &Option<Statement>,
        input_block : &BasicBlock,
        output_block : &BasicBlock,
    ) -> Result<(), CompileError> {
        match &parameters {
            Some(Statement::ExpressionList { expressions }) => {
                for (index, exp) in expressions.iter().enumerate() {
                    self.generate_single_parameter(exp, function_name, None, index as u32, parameter_struct,input_block, output_block)?;
                }
            }
            Some(statement) => {
                self.generate_single_parameter(statement, function_name, None, 0, parameter_struct, input_block, output_block)?;
            }
            None => {}
        }
        Ok(())
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
        assignment_statement: &Statement,
        function_name: &str,
        parameter_type : Option<&DataTypeIndexEntry<'a>>,
        index: u32,
        parameter_struct: PointerValue<'a>,
        input_block : &BasicBlock,
        output_block : &BasicBlock,
    ) -> Result<(), CompileError> {
        let builder = &self.llvm.builder;
        match assignment_statement {
            // explicit call parameter: foo(param := value)
            Statement::Assignment { left, right } => {
                builder.position_at_end(*input_block);
                if let Statement::Reference { name, ..} = &**left {
                    let parameter = self
                        .index
                        .find_member(function_name, &name)
                        .unwrap();
                    let index = parameter
                        .get_location_in_parent()
                        .unwrap();
                    let param_type = self.index.find_type(parameter.get_type_name());
                    self.generate_single_parameter(right, function_name, param_type, index, parameter_struct, input_block, output_block)?;
                }
            }
            // foo (param => value)
            Statement::OutputAssignment { left, right } => {
                let current_block = builder.get_insert_block().unwrap();
                builder.position_at_end(*output_block);
                if let Statement::Reference { name, ..} = &**left {
                    let parameter = self
                        .index
                        .find_member(function_name, &name)
                        .unwrap();
                    let index = parameter
                        .get_location_in_parent()
                        .unwrap();
                    let param_type = self.index.find_type(parameter.get_type_name()).or_else(|| 
                        self.index.find_input_parameter(function_name, index as u32).and_then(|var| self.index.find_type(var.get_type_name()))).and_then(|var| var.get_type_information()).unwrap();
                    //load the function prameter
                    let pointer_to_param = builder
                        .build_struct_gep(parameter_struct, index as u32, "")
                        .unwrap();

                    
                    let l_value = self.generate_load_for( right).unwrap();
                    let loaded_value = builder.build_load(pointer_to_param,parameter.get_name());
                    let value = typesystem::cast_if_needed(self.llvm, l_value.get_type_information(), loaded_value,param_type, right)?;
                    builder
                        .build_store(l_value.ptr_value, value);
                }
                builder.position_at_end(current_block);
            }
            // foo(x)
            _ => {
                let (value_type, generated_exp) = self.generate_expression(assignment_statement)?;
                let pointer_to_param = builder
                    .build_struct_gep(parameter_struct, index as u32, "")
                    .unwrap();
                let parameter = parameter_type.or_else(|| 
                    self.index.find_input_parameter(function_name, index as u32).and_then(|var| self.index.find_type(var.get_type_name()))).and_then(|var| var.get_type_information()).unwrap();
                let value = typesystem::cast_if_needed(self.llvm, parameter, generated_exp, &value_type, assignment_statement)?;
                builder
                    .build_store(pointer_to_param, value);
            }
        }
        Ok(())
    }



    /// generates an load-statement and returns the resulting pointer and DataTypeInfo
    ///
    /// - `reference_statement` - the statement to load (either a reference, an arrayAccess or a qualifiedReference)
    pub fn generate_load(
        &self,
        reference_statement: &Statement,
    ) -> Result<TypeAndPointer<'a, '_>, CompileError> {
        match reference_statement {
            Statement::Reference { name, .. } => {
                self.create_llvm_pointer_value_for_reference(None, name, reference_statement)
            }

            Statement::ArrayAccess { reference, access } => {
                self.generate_load_for_array(None, reference, access)
                //self.generate_reference_from_value((Some(value.type_information), Some(value.ptr_value)),"tmpVar")
            }
            Statement::QualifiedReference { .. } => {
                self.generate_load_for(reference_statement)
            }
            _ => Err(CompileError::codegen_error(format!("Cannot generate a LValue for {:?}", reference_statement), reference_statement.get_location())),
        }
    }

    /// loads the given reference with an optional qualifier
    ///
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x where myStruct is the qualifier for x)
    /// - `name` the name of the reference-name (e.g. myStruct.x where 'x' is the reference-name)
    /// - `context` the statement to obtain the location from when returning an error
    fn create_llvm_pointer_value_for_reference(
        &self,
        qualifier: Option<&TypeAndPointer<'a,'_>>,
        name: &String,
        context: &Statement,
    ) -> Result<TypeAndPointer<'a,'_>, CompileError> {
        //let (data_type, ptr) = if let Some((qualifier_name, qualifier)) = type_with_context {
        let offset = &context.get_location();
        let l_value = if let Some(l_value) = qualifier {
            let qualifier_name = l_value.type_entry.get_name();
            let member = self.index.find_member(l_value.type_entry.get_name(), name);
            let member_location = member
                .map(|it| it.get_location_in_parent())
                .flatten()
                .ok_or_else(||
                    CompileError::invalid_reference(&format!("{:}.{:}", qualifier_name, name), offset.clone()))?;

            //.unwrap();
            let member_data_type = member.map(|it| it.get_type_name()).unwrap();
            let member_type_entry = self.index.get_type(member_data_type)?;
            let gep = self.llvm.get_member_pointer_from_struct(l_value.ptr_value, member_location, name, offset)?;

            TypeAndPointer::new(member_type_entry, gep)
        } else {
            //no context

            let linking_context = self.get_function_context(context)?.linking_context.as_str();

            let variable_index_entry = self
                .index
                .find_variable(Some(linking_context), &[name.clone()])
                .ok_or_else(|| CompileError::InvalidReference{ reference: name.clone(), location: offset.clone() })?;

            let accessor_ptr = variable_index_entry.get_generated_reference()
                    .ok_or_else(||CompileError::codegen_error(format!("Cannot generate reference for {:}",name),offset.clone()))?;
            let variable_type = self.index.get_type(variable_index_entry.get_type_name())?;

            TypeAndPointer::new(variable_type, accessor_ptr)
        };

        Ok(l_value)
    }

    /// generates the access-expression for an array-reference
    /// myArray[array_expression] where array_expression is the access-expression
    ///
    /// - `dimension` the array's dimension
    /// - `access_expression` the expression inside the array-statement
    fn generate_access_for_dimension(
        &self,
        dimension: &Dimension,
        access_expression: &Statement,
    ) -> Result<IntValue<'a>, CompileError> {
        let start_offset = dimension.start_offset;
        let (_, access_value) = self.generate_expression(access_expression)?;
        //If start offset is not 0, adjust the current statement with an add operation
        if start_offset != 0 {
            Ok(self.llvm.builder.build_int_sub(
                access_value.into_int_value(),
                self.llvm.i32_type().const_int(start_offset as u64, true),
                "",
            ))
        } else {
            Ok(access_value.into_int_value())
        }
    }

    /// generates a load statement for a array-reference with an optional qualifier
    /// 
    /// - `qualifier` an optional qualifier for a reference (e.g. myStruct.x[2] where myStruct is the qualifier for x)
    /// - `reference` the reference-statement pointing to the array
    /// - `access` the accessor expression (the expression between the brackets: reference[access])
    fn generate_load_for_array(
        &self,
        qualifier: Option<&TypeAndPointer<'a, '_>>,
        reference: &Statement,
        access: &Statement,
    ) -> Result<TypeAndPointer<'a, '_>, CompileError> {
        //Load the reference
        self.generate_load_for_rec(qualifier, reference)
            .and_then(|lvalue| {
                if let 
                    DataTypeInformation::Array {
                        inner_type_name,
                        dimensions,
                        ..
                    }
                 = lvalue.get_type_information()
                {
                    //First 0 is to access the pointer, then we access the array
                    let mut indices = vec![self.llvm.i32_type().const_int(0, false)];

                    //Make sure dimensions match statement list
                    let statements = access.get_as_list();
                    if statements.len() == 0 || statements.len() != dimensions.len() {
                        panic!(
                            "Mismatched array access : {} -> {} ",
                            statements.len(),
                            dimensions.len()
                        )
                    }
                    for (i, statement) in statements.iter().enumerate() {
                        indices.push(self.generate_access_for_dimension(
                            &dimensions[i],
                            statement,
                        )?)
                    }
                    //Load the access from that reference
                    let pointer = self.llvm.load_array_element(lvalue.ptr_value, indices.as_slice(), "tmpVar")?;

                    let internal_type = self.index.get_type(inner_type_name)?; //TODO this is WRONG!!! typename is not correct
                    return Ok(TypeAndPointer::new(internal_type, pointer))
               }
                Err(CompileError::codegen_error("Invalid array access".to_string(), access.get_location()))
            })
    }
    
    /// generate a load statement for the given referenc-statement
    /// 
    /// - `reference` the reference-statement (either Reference, AccessRefeference or QualifiedReference)
    pub fn generate_load_for(
        &self,
        reference: &Statement,
    ) -> Result<TypeAndPointer<'a, '_>, CompileError> {
        self.generate_load_for_rec(None, reference)
    }

    /// the entry function for recursive reference-generation (for qualified references)
    ///
    /// - `qualifier` the qualifier (TypeAndPointer) for the given reference-statement
    /// - `reference` the reference to load
    fn generate_load_for_rec(
        &self,
        qualifier: Option<&TypeAndPointer<'a, '_>>, 
        reference: &Statement,
    ) -> Result<TypeAndPointer<'a, '_>, CompileError> {
        match reference {
            Statement::QualifiedReference { elements } => {
                let mut element_iter = elements.iter();
                let current_element = element_iter.next();
                let mut current_lvalue = self.generate_load_for_rec(
                    qualifier,
                    &current_element.unwrap(),
                );

                for it in element_iter {
                    let ctx = current_lvalue?;
                    let context_ptr = ctx.ptr_value;
                    let type_information= ctx.type_entry;

                    current_lvalue = self.generate_load_for_rec(
                        Some(&TypeAndPointer::new(type_information, context_ptr)),
                        it,
                    );
                }
                current_lvalue
            }
            Statement::Reference { name, .. } => self
                .create_llvm_pointer_value_for_reference(
                    qualifier,
                    name,
                    reference,
                ),
            Statement::ArrayAccess { reference, access } => {
                self.generate_load_for_array(qualifier, reference, access)
            }
            _ => Err(CompileError::codegen_error(format!("Unsupported Statement {:?}", reference), reference.get_location())),
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
        target_type: &DataTypeInformation<'a>,
    ) -> TypeAndValue<'a> {
        let int_lvalue = left_value.into_int_value();
        let int_rvalue = right_value.into_int_value();

        let (value, data_type) = match operator {
            Operator::Plus => (
                self.llvm.builder.build_int_add(int_lvalue, int_rvalue, "tmpVar"),
                target_type.clone(),
            ),
            Operator::Minus => (
                self.llvm.builder
                    .build_int_sub(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Multiplication => (
                self.llvm.builder
                    .build_int_mul(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Division => (
                self.llvm.builder
                    .build_int_signed_div(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Modulo => (
                self.llvm.builder
                    .build_int_signed_rem(int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Equal => (
                self.llvm.builder
                    .build_int_compare(IntPredicate::EQ, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::NotEqual => (
                self.llvm.builder
                    .build_int_compare(IntPredicate::NE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Less => (
                self.llvm.builder
                    .build_int_compare(IntPredicate::SLT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Greater => (
                self.llvm.builder
                    .build_int_compare(IntPredicate::SGT, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::LessOrEqual => (
                self.llvm.builder
                    .build_int_compare(IntPredicate::SLE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::GreaterOrEqual => (
                self.llvm.builder
                    .build_int_compare(IntPredicate::SGE, int_lvalue, int_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),
            Operator::Xor => (
                self.llvm.builder.build_xor(int_lvalue, int_rvalue, "tmpVar").into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),
            _ => unimplemented!(),
        };
        (data_type, value.into())
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
        target_type: &DataTypeInformation<'a>,
    ) -> TypeAndValue<'a> {
        let float_lvalue = lvalue.into_float_value();
        let float_rvalue = rvalue.into_float_value();

        let (value, data_type) = match operator {
            Operator::Plus => (
                self.llvm.builder
                    .build_float_add(float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Minus => (
                self.llvm.builder
                    .build_float_sub(float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Multiplication => (
                self.llvm.builder
                    .build_float_mul(float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Division => (
                self.llvm.builder
                    .build_float_div(float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Modulo => (
                self.llvm.builder
                    .build_float_rem(float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                target_type.clone(),
            ),
            Operator::Equal => (
                self.llvm.builder
                    .build_float_compare(FloatPredicate::OEQ, float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::NotEqual => (
                self.llvm.builder
                    .build_float_compare(FloatPredicate::ONE, float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Less => (
                self.llvm.builder
                    .build_float_compare(FloatPredicate::OLT, float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::Greater => (
                self.llvm.builder
                    .build_float_compare(FloatPredicate::OGT, float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::LessOrEqual => (
                self.llvm.builder
                    .build_float_compare(FloatPredicate::OLE, float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            Operator::GreaterOrEqual => (
                self.llvm.builder
                    .build_float_compare(FloatPredicate::OGE, float_lvalue, float_rvalue, "tmpVar")
                    .into(),
                self.index.find_type_information("BOOL").unwrap(),
            ),

            _ => unimplemented!(),
        };
        (data_type, value)
    }

    /// generates the literal statement and returns the resulting value
    ///
    /// - `literal_statement` one of LiteralBool, LiteralInteger, LiteralReal, LiteralString
    pub fn generate_literal(
        &self,
        literal_statement: &Statement,
    ) -> Result<TypeAndValue<'a>, CompileError> {
        match literal_statement {
            Statement::LiteralBool { value, .. } => self.llvm.create_const_bool(self.index, *value),
            Statement::LiteralInteger { value, .. } => {
                self.llvm.create_const_int(self.index, &self.get_type_context(), value)
            }
            Statement::LiteralReal { value, .. } => self.llvm.create_const_real(self.index, &self.get_type_context(), value),
            Statement::LiteralString { value, .. } => self.llvm.create_const_string(value.as_str()),
            Statement::LiteralArray { elements, location} => self.generate_literal_array(elements, location),
            _ => Err(CompileError::codegen_error(
                format!("Cannot generate Literal for {:?}", literal_statement),
                literal_statement.get_location(),
            )),
        }
    }

    /// generates an array literal with the given optional elements (represented as an ExpressionList)
    fn generate_literal_array(&self, elements: &Option<Box<Statement>>, location: &Range<usize>) -> Result<TypeAndValue<'a>, CompileError> {
        if let Some(type_info) = &self.type_hint {
            if let DataTypeInformation::Array{inner_type_hint , ..}  = type_info {
                if let Some(initializer) = elements {
                    let array_value = self.generate_literal_array_value(
                        flatten_expression_list(initializer), 
                        inner_type_hint)?;
                    return Ok((type_info.clone(), array_value.as_basic_value_enum()));
                }
            }
        }
        return Err(CompileError::codegen_error("Internal error when generating Array literal: unknown inner array-type.".to_string(), location.clone()));
    }

    /// constructs an ArrayValue (returned as a BasicValueEnum) of the given element-literals constructing an array-value of the
    /// type described by inner_array_type.
    ///
    /// passing an epxression-lists with LiteralIntegers and inner_array_type is INT-description will return an
    /// i16-array-value
    fn generate_literal_array_value(&self, elements: Vec<&Statement>, inner_array_type: &DataTypeInformation<'a>) -> Result<BasicValueEnum<'a>, CompileError> {
        let element_expression_gen = self.morph_to_typed(inner_array_type);
        let llvm_type = inner_array_type.get_type();
        
        let mut v = Vec::new();
        for e in elements {
            //generate with correct type hint
            let (_, value) = element_expression_gen.generate_literal(e)?;
            v.push(value.as_basic_value_enum());
        }

        //TODO Validation: fail with compile-error if value cannot be converted into... correctly
        let array_value = match llvm_type {
            BasicTypeEnum::ArrayType(_) => 
                llvm_type.into_array_type().const_array(v.iter().map(|it| it.into_array_value()).collect::<Vec<ArrayValue>>().as_slice()),
            BasicTypeEnum::FloatType(_) => 
                llvm_type.into_float_type().const_array(v.iter().map(|it| it.into_float_value()).collect::<Vec<FloatValue>>().as_slice()),
            BasicTypeEnum::IntType(_) => 
                llvm_type.into_int_type().const_array(v.iter().map(|it| it.into_int_value()).collect::<Vec<IntValue>>().as_slice()),
            BasicTypeEnum::PointerType(_) =>
                llvm_type.into_pointer_type().const_array(v.iter().map(|it| it.into_pointer_value()).collect::<Vec<PointerValue>>().as_slice()),
            BasicTypeEnum::StructType(_) => 
                llvm_type.into_struct_type().const_array(v.iter().map(|it| it.into_struct_value()).collect::<Vec<StructValue>>().as_slice()),
            BasicTypeEnum::VectorType(_) => 
                llvm_type.into_vector_type().const_array(v.iter().map(|it| it.into_vector_value()).collect::<Vec<VectorValue>>().as_slice()),
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
        left: &Statement, 
        right: &Statement,
    ) -> Result<TypeAndValue<'a>, CompileError>{
        let builder = &self.llvm.builder;
        let function = self.get_function_context(left)?.function;
        
        let right_branch = self.llvm.context.append_basic_block(function, "");
        let continue_branch = self.llvm.context.append_basic_block(function, "");

        let (left_type, left_value) = self.generate_expression(left)?;
        let final_left_block = builder.get_insert_block().unwrap();
        //Compare left to 0
        let lhs = builder.build_int_compare(IntPredicate::NE, left_value.into_int_value(), left_type.get_type().into_int_type().const_int(0,false), "");
        match operator {
            Operator::Or => builder.build_conditional_branch(lhs,continue_branch,right_branch),
            Operator::And => builder.build_conditional_branch(lhs,right_branch,continue_branch),
            _ => return Err(CompileError::codegen_error(format!("Cannot generate phi-expression for operator {:}", operator), left.get_location()))
        };

        builder.position_at_end(right_branch);
        let (right_type, right_value) = self.generate_expression(right)?;
        let final_right_block = builder.get_insert_block().unwrap();
        let rhs = right_value;
        builder.build_unconditional_branch(continue_branch);

        builder.position_at_end(continue_branch);
        //Generate phi
        let target_type = if left_type.get_size() > right_type.get_size() { left_type } else { right_type };
        let phi_value = builder.build_phi(target_type.get_type(),"");
        phi_value.add_incoming(&[(&left_value.into_int_value(),final_left_block), (&rhs,final_right_block)]);

        Ok((target_type,phi_value.as_basic_value()))
    }
}
