use std::{ops::Range, thread::current};

use super::{LValue, TypeAndValue, expression_generator, instance_struct_generator::{self, InstanceStructGenerator}, literals, typesystem, variable_generator};
use crate::{ast::{ConditionalBlock, Operator, Statement}, index::{DataTypeIndexEntry, DataTypeInformation, Dimension, Index, VariableIndexEntry}};
use inkwell::{AddressSpace, IntPredicate, basic_block::BasicBlock, builder::Builder, context::Context, types::{BasicType, BasicTypeEnum}, values::{AnyValue, BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue}};

pub struct FunctionContext<'a> {
    pub linking_context: String,
    pub function: FunctionValue<'a>,
}

pub struct StatementCodeGenerator<'a, 'b> {
    context: &'a Context,
    index: &'b Index<'a>,
    type_hint: Option<BasicTypeEnum<'a>>,
    function_context: Option<&'a FunctionContext<'a>>,

    pub load_prefix: String,
    pub load_suffix: String,
}

impl<'a, 'b> StatementCodeGenerator<'a, 'b> {
    pub fn new(
        context: &'a Context,
        global_index: &'b Index<'a>,
        linking_context: Option<&'a FunctionContext<'a>>,
    ) -> StatementCodeGenerator<'a, 'b> {
        StatementCodeGenerator {
            context,
            index: global_index,
            type_hint: None,
            function_context: linking_context,
            load_prefix: "load_".to_string(),
            load_suffix: "".to_string(),
        }
    }

    pub fn new_typed(
        context: &'a Context,
        global_index: &'b Index<'a>,
        linking_context: Option<&'a FunctionContext<'a>>,
        type_hint: BasicTypeEnum<'a>,
    ) -> StatementCodeGenerator<'a, 'b> {
        StatementCodeGenerator {
            context,
            index: global_index,
            type_hint: Some(type_hint),
            function_context: linking_context,
            load_prefix: "load_".to_string(),
            load_suffix: "".to_string(),
        }
    }

    fn get_current_function(&self) -> Result<FunctionValue, String> {
        self.function_context.map(|it|it.function).ok_or_else(|| "Cannog generate code outside of a Function Value".to_string())
    }

    pub fn generate_body(
        &self,
        statements: &Vec<Statement>,
        builder: &Builder<'a>,
    ) -> Result<(), String> {
        for s in statements {
            self.generate_statement(s, builder)?;
        }
        Ok(())
    }

    pub fn generate_statement(
        &self,
        statement: &Statement,
        builder: &Builder<'a>,
    ) -> Result<(), String> {
        match statement {
            Statement::Assignment { left, right } => {
                self.generate_assignment_statement(left, right, builder)?;
            }
            Statement::ForLoopStatement {
                start,
                end,
                counter,
                body,
                by_step,
                ..
            } => {
                let f_context = self
                    .function_context
                    .ok_or_else(|| "Cannot generate for-statement wihtout a function-context")?;
                self.generate_for_statement(
                    builder,
                    f_context.function,
                    counter,
                    start,
                    end,
                    by_step,
                    body,
                )?;
            },
            Statement::RepeatLoopStatement{ condition, body, ..} =>  {
                self.generate_repeat_statement(builder, condition, body)?;
            },
            Statement::WhileLoopStatement{condition, body, ..} => {
                self.generate_while_statement(builder, condition, body)?;
            },
            Statement::IfStatement{ blocks, else_block, ..} => {
                self.generate_if_statement(builder, blocks, else_block)?;
            },
            Statement::CaseStatement{ selector, case_blocks, else_block, ..} => {
                self.generate_case_statement(builder, selector, case_blocks, else_block)?;
            }
            _ => {
                self.generate_expression(statement, builder)?;
            }
        }
        Ok(())
    }

    fn generate_assignment_statement(
        &self,
        left: &Statement,
        right: &Statement,
        builder: &Builder,
    ) -> Result<(), String> {
        let left = self.generate_l_value(left, builder)?;
        let (right_type, right) = {
            //let expected_type = left.type_information.get_type();
            //TODO: this typing produces wrong results!
            let sub_statement_gen = StatementCodeGenerator::new(
                self.context,
                self.index,
                self.function_context,
            );
            sub_statement_gen.generate_expression(right, builder)?
        };
        let cast_value =
            typesystem::cast_if_needed(builder, &left.type_information, right, &right_type)?;
        builder.build_store(left.ptr_value, cast_value);
        Ok(())
    }

    fn generate_call_statement(
        &self,
        builder: &Builder<'a>,
        operator: &Statement,
        parameters: &Option<Statement>) -> Result<TypeAndValue<'a>, String> {
        let instance_and_index_entry = match operator {
            Statement::Reference { name, .. } => {
                //Get associated Variable or generate a variable for the type with the same name
                let variable = self.index
                    .find_callable_instance_variable(self.function_context.map(|it|it.linking_context.as_str()), &[name.clone()]);
        
                let callable_reference = if let Some(variable_instance) = variable {
                    variable_instance.get_generated_reference()
                        .ok_or_else(|| format!("cannot find callable type for {:}", name).to_string())?
                } else {
                    let instance_generator = InstanceStructGenerator::new(self.context, self.index);
                    let callable = instance_generator.allocate_struct_instance(builder, &name)?;
                    callable
                };
                        
                let call_name = variable
                    .map(|it| it.get_type_name()) // we called f() --> look for f's datatype
                    .or(Some(&name)); // we didnt call a variable ([0so we treat the string as the function's name

                let index_entry = self.index.get_type(call_name.unwrap())?;
                Ok((callable_reference, index_entry))
            }
            _ => Err(format!("cannot generate call statement for {:?}", operator)),
        };

        let (instance, index_entry) = instance_and_index_entry?;
        let function_name = index_entry.get_name();
        //Create parameters for input and output blocks
        let current_f = self.get_current_function()?;
        let input_block = self.context.append_basic_block(current_f, "input");
        let call_block = self.context.append_basic_block(current_f, "call");
        let output_block = self.context.append_basic_block(current_f, "output");
        let continue_block = self.context.append_basic_block(current_f, "continue");
        //First go to the input block
        builder.build_unconditional_branch(input_block);
        builder.position_at_end(input_block);
        //Generate all parameters, this function may jump to the output block
        self.generate_function_parameters(builder, function_name, instance, parameters, &input_block, &output_block)?;
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
                .ok_or_else(|| format!("No callable implementation associated to {:?}", function_name))?;
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
            |value| value, 
            |_| dbg!(self.index.get_type_information("INT").unwrap().get_type().ptr_type(AddressSpace::Const).const_null().into()));

        return Ok(( return_type.unwrap(), value ));
    }

    fn generate_function_parameters(
        &self,
        builder: &Builder<'a>,
        function_name: &str,
        variable: PointerValue<'a>,
        parameters: &Option<Statement>,
        input_block : &BasicBlock,
        output_block : &BasicBlock,
    ) -> Result<(), String> {
        match &parameters {
            Some(Statement::ExpressionList { expressions }) => {
                for (index, exp) in expressions.iter().enumerate() {
                    self.generate_single_parameter(builder, exp, function_name, None, index as u32, variable,input_block, output_block)?;
                }
            }
            Some(statement) => {
                self.generate_single_parameter(builder, statement, function_name, None, 0, variable,input_block, output_block)?;
            }
            None => {}
        }
        Ok(())
    }

    fn generate_single_parameter(
        &self,
        builder: &Builder<'a>,
        statement: &Statement,
        function_name: &str,
        parameter_type : Option<&DataTypeIndexEntry<'a>>,
        index: u32,
        pointer_value: PointerValue<'a>,
        input_block : &BasicBlock,
        output_block : &BasicBlock,
    ) -> Result<(), String> {
        match statement {
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
                    self.generate_single_parameter(builder, right, function_name, param_type, index, pointer_value, input_block, output_block)?;
                }
            }
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
                        .build_struct_gep(pointer_value, index as u32, "")
                        .unwrap();

                    
                    let (target_type,pointer_to_target) = self.generate_lvalue_for(builder, right).unwrap();
                    let loaded_value = builder.build_load(pointer_to_param,parameter.get_name());
                    let value = typesystem::cast_if_needed(&builder, &target_type, loaded_value,param_type).unwrap();
                    builder
                        .build_store(pointer_to_target, value);
                }
                builder.position_at_end(current_block);
            }
            _ => {
                if let (value_type, generated_exp) = self.generate_expression(statement, builder)? {
                    let pointer_to_param = builder
                        .build_struct_gep(pointer_value, index as u32, "")
                        .unwrap();
                    let parameter = parameter_type.or_else(|| 
                        self.index.find_input_parameter(function_name, index as u32).and_then(|var| self.index.find_type(var.get_type_name()))).and_then(|var| var.get_type_information()).unwrap();
                    let value = typesystem::cast_if_needed(&builder, parameter, generated_exp, &value_type);
                    builder
                        .build_store(pointer_to_param, value.unwrap());
                }
            }
        }
        Ok(())
    }



    fn generate_for_statement(
        &self,
        builder: &Builder<'a>,
        current_function: FunctionValue,
        counter: &Box<Statement>,
        start: &Box<Statement>,
        end: &Box<Statement>,
        by_step: &Option<Box<Statement>>,
        body: &Vec<Statement>,
    ) -> Result<(), String> {
        self.generate_assignment_statement(counter, start, builder)?;
        let condition_check = self
            .context
            .append_basic_block(current_function, "condition_check");
        let for_body = self
            .context
            .append_basic_block(current_function, "for_body");
        let continue_block = self
            .context
            .append_basic_block(current_function, "continue");
        //Generate an initial jump to the for condition
        builder.build_unconditional_branch(condition_check);

        //Check loop condition
        builder.position_at_end(condition_check);
        let (_, counter_statement) = self.generate_expression(counter, builder)?;
        let (_, end_statement) = self.generate_expression(end, builder)?;

        let compare = builder.build_int_compare(
            IntPredicate::SLE,
            counter_statement.into_int_value(),
            end_statement.into_int_value(),
            "tmpVar",
        );
        builder.build_conditional_branch(compare, for_body, continue_block);

        //Enter the for loop
        builder.position_at_end(for_body);
        self.generate_body(body, builder)?;

        //Increment
        let (_, step_by_value) = by_step
             .as_ref()
             .map_or_else(
                 || self.generate_literal(
                     &Statement::LiteralInteger{ value: "1".to_string(), location: end.get_location() } ),
             |step| self.generate_expression(&step, builder))?;
             

        let next = builder
            .build_int_add(counter_statement.into_int_value(), step_by_value.into_int_value(), "tmpVar");
                    
        let (_, ptr) = self.generate_lvalue_for(builder, counter)?;
        builder.build_store(ptr, next);

        //Loop back
        builder.build_unconditional_branch(condition_check);

        //Continue
        builder.position_at_end(continue_block);

        Ok(())
    }

    fn generate_case_statement(
        &self,
        builder: &Builder<'a>,
        selector: &Statement,
        conditional_blocks: &Vec<ConditionalBlock>,
        else_body: &Vec<Statement>,
    ) -> Result<Option<BasicValueEnum<'a>>, String> {

        let current_function = self.get_current_function()?;
        //Continue
        let continue_block = self
            .context
            .append_basic_block(current_function, "continue");

        let basic_block = builder.get_insert_block().unwrap();
        
        let (_, selector_statement) = self.generate_expression(&*selector, builder)?;
        let mut cases = Vec::new();

        //generate a int_value and a BasicBlock for every case-body
        for i in 0..conditional_blocks.len() {
            let conditional_block = &conditional_blocks[i];
            let basic_block = self
                .context
                .append_basic_block(current_function, "case");
            let (_, condition) = self.generate_expression(&*conditional_block.condition, builder)?; //TODO : Is a type conversion needed here?
            builder.position_at_end(basic_block);
            self.generate_body(&conditional_block.body, builder)?;
            builder.build_unconditional_branch(continue_block);

            cases.push((condition.into_int_value(), basic_block));
        }

        let else_block = self
            .context
            .append_basic_block(current_function, "else");
        builder.position_at_end(else_block);
        self.generate_body(else_body, builder)?;
        builder.build_unconditional_branch(continue_block);

        //Move the continue block to after the else block
        continue_block.move_after(else_block).unwrap();
        //Position in initial block
        builder.position_at_end(basic_block);
        builder
            .build_switch(selector_statement.into_int_value(), else_block, &cases);
        builder.position_at_end(continue_block);
        Ok(None)
    }


    fn generate_while_statement(
        &self,
        builder: &Builder<'a>,
        condition: &Box<Statement>,
        body: &Vec<Statement>,
    ) -> Result<Option<BasicValueEnum<'a>>, String> {
        let basic_block = builder.get_insert_block().unwrap();
        self.generate_base_while_statement(builder, condition, body)?;

        let continue_block = builder.get_insert_block().unwrap();

        let condition_block = basic_block.get_next_basic_block().unwrap();
        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(condition_block);

        builder.position_at_end(continue_block);
        Ok(None)
    }

    fn generate_repeat_statement(
        &self,
        builder: &Builder<'a>,
        condition: &Box<Statement>,
        body: &Vec<Statement>,
    ) -> Result<Option<BasicValueEnum<'a>>, String> {
        let basic_block = builder.get_insert_block().unwrap();
        self.generate_base_while_statement(builder, condition, body)?;

        let continue_block = builder.get_insert_block().unwrap();

        let while_block = continue_block.get_previous_basic_block().unwrap();
        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(while_block);

        builder.position_at_end(continue_block);
        Ok(None)
    }

    fn generate_base_while_statement(
        &self,
        builder: &Builder<'a>,
        condition: &Box<Statement>,
        body: &Vec<Statement>,
    ) -> Result<Option<BasicValueEnum>, String> {
        let current_function = self.get_current_function()?;
        let condition_check = self
            .context
            .append_basic_block(current_function, "condition_check");
        let while_body = self
            .context
            .append_basic_block(current_function, "while_body");
        let continue_block = self
            .context
            .append_basic_block(current_function, "continue");

        //Check loop condition
        builder.position_at_end(condition_check);
        let (_, condition_value) = self.generate_expression(condition, builder)?;
        builder
            .build_conditional_branch(condition_value.into_int_value(), while_body, continue_block);

        //Enter the for loop
        builder.position_at_end(while_body);
        self.generate_body(&body, builder)?;
        //Loop back
        builder.build_unconditional_branch(condition_check);

        //Continue
        builder.position_at_end(continue_block);
        Ok(None)
    }

    fn generate_if_statement(
        &self,
        builder: &Builder<'a>,
        conditional_blocks: &Vec<ConditionalBlock>,
        else_body: &Vec<Statement>,
    ) -> Result<(), String> {
        let mut blocks = Vec::new();
        blocks.push(builder.get_insert_block().unwrap());
        let current_function = self.function_context.map(|it|it.function).unwrap();
        for _ in 1..conditional_blocks.len() {
            blocks.push(
                self.context
                    .append_basic_block(current_function, "branch"),
            );
        }

        let else_block = if else_body.len() > 0 {
            let result = self
                .context
                .append_basic_block(current_function, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = self
            .context
            .append_basic_block(current_function, "continue");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            builder.position_at_end(then_block);

            let (_,condition) = self.generate_expression(&block.condition, builder)?;
            let conditional_block = self
                .context
                .prepend_basic_block(else_block, "condition_body");

            //Generate if statement condition
            builder.build_conditional_branch(condition.into_int_value(), conditional_block, else_block);

            //Generate if statement content

            builder.position_at_end(conditional_block);
            self.generate_body(&block.body, builder)?;
            builder.build_unconditional_branch(continue_block);
        }
        //Else

       if let Some(else_block) = else_block {
            builder.position_at_end(else_block);
            self.generate_body(&else_body, builder)?;
            builder.build_unconditional_branch(continue_block);
        }
        //Continue
        builder.position_at_end(continue_block);
        Ok(())
    }

    pub fn generate_expression(
        &self,
        statement: &Statement,
        builder: &Builder<'a>,
    ) -> Result<TypeAndValue<'a>, String> {
        match statement {
            Statement::Reference { name, location } => {
                let pointer =
                    self.create_llvm_pointer_value_for_reference(None, name, builder, location)?;
                let load_name = format!("{}{}{}", self.load_prefix, name, self.load_suffix);
                variable_generator::create_llvm_load_pointer(builder, &pointer, &load_name)
            },
            Statement::QualifiedReference { .. } => {
                let (data_type, ptr_value) = self.generate_lvalue_for(builder, statement)?;
                let lvalue = LValue{ ptr_value, type_information: data_type, type_name: "".to_string()};
                variable_generator::create_llvm_load_pointer(builder, &lvalue, &self.load_prefix)
            },
            Statement::ArrayAccess { .. } => {
                let (a, b) = self.generate_lvalue_for(builder, statement)?;
                let lvalue = LValue {
                    type_name: "".to_string(),
                    type_information: a,
                    ptr_value: b,
                };
                variable_generator::create_llvm_load_pointer(builder, &lvalue, "load_tmpVar")
                //TODO get find better name
            }
            Statement::BinaryExpression {
                left,
                right,
                operator,
            } => {

                //If OR, or AND handle before generating the statements
                match operator {
                    Operator::And | Operator::Or => 
                        return self.generate_phi_expression(builder, operator, left, right),
                    _ => {}
                }

                let left_type_and_value = self.generate_expression(left, builder)?;
                let right_type_and_value = self.generate_expression(right, builder)?;

                let (common_type, left_value, right_value) = typesystem::promote_if_needed(
                    builder,
                    &left_type_and_value,
                    &right_type_and_value,
                    self.index,
                );

                if common_type.is_int() {
                    Ok(expression_generator::create_llvm_int_binary_expression(
                        builder,
                        self.index,
                        operator,
                        left_value,
                        right_value,
                        &common_type,
                    ))
                } else if common_type.is_float() {
                    Ok(expression_generator::create_llvm_float_binary_expression(
                        builder,
                        self.index,
                        operator,
                        left_value,
                        right_value,
                        &common_type,
                    ))
                } else {
                    Err(format!(
                        "invalid types, cannot generate binary expression for {:?}",
                        common_type
                    ))
                }
            },
            Statement::CallStatement{ operator, parameters, ..} => {
                self.generate_call_statement(builder, operator, parameters)
            },
            Statement::UnaryExpression { operator, value, ..} => {
                self.generate_unary_expression(builder, operator, value)
            },
            _ => self.generate_literal(statement),
        }
    }

    fn generate_unary_expression(
        &self,
        builder: &Builder<'a>,
        operator: &Operator,
        value: &Box<Statement>,
    ) -> Result<TypeAndValue<'a>, String> {
        let (data_type, loaded_value) = self.generate_expression(value, builder)?;
        let (data_type, value) = match operator {
            Operator::Not => (
                data_type,
                builder
                    .build_not(loaded_value.into_int_value(), "tmpVar"),
            ),
            Operator::Minus => (
                data_type,
                builder
                    .build_int_neg(loaded_value.into_int_value(), "tmpVar"),
            ),
            _ => unimplemented!(),
        };
        Ok((data_type, BasicValueEnum::IntValue(value)))
    }

    fn generate_phi_expression(
        &self, 
        builder: &Builder<'a>,
        operator: &Operator, 
        left: &Box<Statement>, 
        right: &Box<Statement>
    ) -> Result<TypeAndValue<'a>, String>{
        let current_function = self.function_context.map(|it| it.function).unwrap();
        let right_branch = self.context.append_basic_block(current_function, "");
        let continue_branch = self.context.append_basic_block(current_function, "");

        let (left_type, left_value) = self.generate_expression(left, builder)?;
        let final_left_block = builder.get_insert_block().unwrap();
        //Compare left to 0
        let lhs = builder.build_int_compare(IntPredicate::NE, left_value.into_int_value(), left_type.get_type().into_int_type().const_int(0,false), "");
        match operator {
            Operator::Or => builder.build_conditional_branch(lhs,continue_branch,right_branch),
            Operator::And => builder.build_conditional_branch(lhs,right_branch,continue_branch),
            _ => unreachable!() 
        };

        builder.position_at_end(right_branch);
        let (right_type, right_value) = self.generate_expression(right, builder)?;
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

    /// generates an L-value (something with an adress), returns a pointer
    //TODO change this to TypeAndPointer
    fn generate_l_value(
        &self,
        statement: &Statement,
        builder: &Builder<'a>,
    ) -> Result<LValue<'a>, String> {
        match statement {
            Statement::Reference { name, location } => {
                self.create_llvm_pointer_value_for_reference(None, name, builder, location)
            }

            Statement::ArrayAccess { reference, access } => {
                self.generate_lvalue_for_array(builder, None, reference, access)
                //self.generate_reference_from_value((Some(value.type_information), Some(value.ptr_value)),"tmpVar")
            }
            Statement::QualifiedReference { .. } => {
                let (data_type, ptr_value) = self.generate_lvalue_for(builder, statement)?;
                Ok(LValue{type_name:"".to_string(), ptr_value, type_information: data_type })
            }

            _ => Err(format!("Cannot generate a LValue for {:?}", statement)),
        }
    }

    pub fn generate_literal(&self, statement: &Statement) -> Result<TypeAndValue<'a>, String> {
        match statement {
            Statement::LiteralBool { value, .. } => {
                literals::create_llvm_const_bool(self.context, self.index, *value)
            },
            Statement::LiteralInteger { value, .. } => {
                literals::create_llvm_const_int(self.context, self.index, &self.type_hint, value)
            }, //TODO
            Statement::LiteralReal { value, .. } => {
                literals::create_llvm_const_real(self.context, self.index, &self.type_hint, value)
            },
            Statement::LiteralString { value, .. } => {
                literals::create_llvm_const_string(self.context, value)
            }
            //Statement::LiteralString{value, ..} => {},
            _ => Err(format!("Cannot generate a Literal for {:?}", statement)),
        }
    }

    fn create_llvm_pointer_value_for_reference(
        &self,
        type_with_context: Option<(&str, PointerValue<'a>)>,
        name: &String,
        builder: &Builder<'a>,
        offset: &Range<usize>,
    ) -> Result<LValue<'a>, String> {
        let (data_type, ptr) = if let Some((qualifier_name, qualifier)) = type_with_context {
            let member = self.index.find_member(qualifier_name, name);
            let member_location = member
                .map(|it| it.get_location_in_parent())
                .flatten()
                .ok_or(format!(
                    "Unknown reference '{:}.{:}' at {:?}",
                    qualifier_name,
                    name,
                    offset /*self.new_lines.get_location_information(offset)*/
                ))?;

            //.unwrap();
            let member_data_type = member.map(|it| it.get_type_name()).unwrap();
            let gep = builder.build_struct_gep(qualifier, member_location, name);

            (member_data_type, gep.ok())
        } else {
            //no context
            let linking_context = self
                .function_context
                .as_ref()
                .map(|it| it.linking_context.as_str());

            let variable_index_entry = self
                .index
                .find_variable(linking_context, &[name.clone()])
                .ok_or("cannot find reference TODO")?;

            let accessor_ptr = variable_index_entry.get_generated_reference();
            (variable_index_entry.get_type_name(), accessor_ptr)
        };

        //return a LValue for the new pointer (type_name, type_info, pointer_value)
        ptr.map(|it| LValue {
            type_name: data_type.to_string(),
            type_information: self.index.find_type_information(data_type).unwrap(),
            ptr_value: it,
        })
        .ok_or(format!("Could not generate reference for {}", name))
    }

    fn generate_access_for_dimension(
        &self,
        builder: &Builder<'a>,
        dimension: &Dimension,
        access_statement: &Statement,
    ) -> Result<IntValue<'a>, String> {
        let start_offset = dimension.start_offset;
        let (_, access_value) = self.generate_expression(access_statement, builder)?;
        //If start offset is not 0, adjust the current statement with an add operation
        if start_offset != 0 {
            Ok(builder.build_int_sub(
                access_value.into_int_value(),
                self.context.i32_type().const_int(start_offset as u64, true),
                "",
            ))
        } else {
            Ok(access_value.into_int_value())
        }
    }

    fn generate_lvalue_for_array(
        &self,
        builder: &Builder<'a>,
        type_with_context: Option<(&str, PointerValue<'a>)>,
        reference: &Statement,
        access: &Statement,
    ) -> Result<LValue<'a>, String> {
        //Load the reference
        self.generate_lvalue_for_rec(builder, type_with_context, reference)
            .and_then(|lvalue| {
                let type_info_and_ptr = (lvalue.type_information, lvalue.ptr_value);
                if let (
                    DataTypeInformation::Array {
                        name,
                        internal_type_information,
                        dimensions,
                        ..
                    },
                    value,
                ) = type_info_and_ptr
                {
                    //First 0 is to access the pointer, then we access the array
                    let mut indices = vec![self.context.i32_type().const_int(0, false)];

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
                            builder,
                            &dimensions[i],
                            statement,
                        )?)
                    }
                    //Load the access from that reference
                    let pointer =
                        unsafe { builder.build_in_bounds_gep(value, indices.as_slice(), "tmpVar") };
                    return Ok(LValue {
                        type_name: name,
                        type_information: *internal_type_information,
                        ptr_value: pointer,
                    });
                }
                Err(format!("Invalid array access at {:}", 0)) //TODO
            })
    }
    fn generate_lvalue_for(
        &self,
        builder: &Builder<'a>,
        statement: &Statement,
    ) -> Result<(DataTypeInformation<'a>, PointerValue<'a>), String> {
        self.generate_lvalue_for_rec(builder, None, statement).map(
            |LValue {
                 type_name: _,
                 type_information,
                 ptr_value,
             }| (type_information, ptr_value),
        )
    }

    fn generate_lvalue_for_rec(
        &self,
        builder: &Builder<'a>,
        type_with_context: Option<(&str, PointerValue<'a>)>,
        statement: &Statement,
    ) -> Result<LValue<'a>, String> {
        match statement {
            Statement::QualifiedReference { elements } => {
                let mut element_iter = elements.iter();
                let current_element = element_iter.next();
                let mut current_lvalue = self.generate_lvalue_for_rec(
                    builder,
                    type_with_context,
                    &current_element.unwrap(),
                );

                for it in element_iter {
                    let LValue {
                        type_name: context_type_name,
                        type_information: _,
                        ptr_value: context_ptr,
                    } = current_lvalue?;

                    current_lvalue = self.generate_lvalue_for_rec(
                        builder,
                        Some((context_type_name.as_str(), context_ptr)),
                        it,
                    );
                }
                current_lvalue
            }
            Statement::Reference { name, location, .. } => self
                .create_llvm_pointer_value_for_reference(
                    type_with_context,
                    name,
                    builder,
                    location,
                ),
            Statement::ArrayAccess { reference, access } => {
                self.generate_lvalue_for_array(builder, type_with_context, reference, access)
            }
            _ => Err(format!("Unsupported Statement {:?}", statement)),
        }
    }
}
