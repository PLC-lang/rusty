/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use super::{expression_generator::{ExpressionCodeGenerator}, llvm::LLVM};
use crate::{ast::{ConditionalBlock, Statement}, codegen::{typesystem}, compile_error::CompileError, index::Index };
use inkwell::{IntPredicate, builder::Builder, types::{BasicTypeEnum}, values::{BasicValueEnum, FunctionValue}};

pub struct FunctionContext<'a> {
    pub linking_context: String,
    pub function: FunctionValue<'a>,
}

pub struct StatementCodeGenerator<'a, 'b> {
    llvm: &'b LLVM<'a>,
    index: &'b Index<'a>,
    type_hint: Option<BasicTypeEnum<'a>>,
    function_context: &'b FunctionContext<'a>,

    pub load_prefix: String,
    pub load_suffix: String,
}

impl<'a, 'b> StatementCodeGenerator<'a, 'b> {
    pub fn new(
        llvm: &'b LLVM<'a>,
        global_index: &'b Index<'a>,
        linking_context: &'b FunctionContext<'a>,
    ) -> StatementCodeGenerator<'a, 'b> {
        StatementCodeGenerator {
            llvm,
            index: global_index,
            type_hint: None,
            function_context: linking_context,
            load_prefix: "load_".to_string(),
            load_suffix: "".to_string(),
        }
    }

    fn create_expr_generator(&self, type_hint: Option<BasicTypeEnum<'a>>) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator::new(self.llvm, self.index, type_hint, self.function_context)
    }
 
    pub fn generate_body(
        &self,
        statements: &Vec<Statement>,
        builder: &Builder<'a>,
    ) -> Result<(), CompileError> {
        for s in statements {
            self.generate_statement(s, builder)?;
        }
        Ok(())
    }

    pub fn generate_statement(
        &self,
        statement: &Statement,
        builder: &Builder<'a>,
    ) -> Result<(), CompileError> {
        match statement {
            Statement::Assignment { left, right } => {
                self.generate_assignment_statement(left, right)?;
            }
            Statement::ForLoopStatement {
                start,
                end,
                counter,
                body,
                by_step,
                ..
            } => {
                self.generate_for_statement(
                    builder,
                    self.function_context.function,
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
                self.create_expr_generator(self.type_hint).generate_expression(statement)?;
            }
        }
        Ok(())
    }

    fn generate_assignment_statement(
        &self,
        left_statement: &Statement,
        right_statement: &Statement,
    ) -> Result<(), CompileError> {
        let exp_gen = self.create_expr_generator(self.type_hint);
        let left = exp_gen.generate_load(left_statement)?;
        let (right_type, right) = exp_gen.generate_expression(right_statement)?;
        let cast_value =
            typesystem::cast_if_needed(self.llvm, &left.get_type_information(), right, &right_type, right_statement)?;
        self.llvm.builder.build_store(left.ptr_value, cast_value);
        Ok(())
    }



    fn generate_for_statement(
        &self,
        builder: &Builder<'a>,
        current_function: FunctionValue,
        counter: &Statement,
        start: &Statement,
        end: &Statement,
        by_step: &Option<Box<Statement>>,
        body: &Vec<Statement>,
    ) -> Result<(), CompileError> {
        self.generate_assignment_statement(counter, start)?;
        let condition_check = self.llvm
            .context
            .append_basic_block(current_function, "condition_check");
        let for_body = self.llvm
            .context
            .append_basic_block(current_function, "for_body");
        let continue_block = self.llvm
            .context
            .append_basic_block(current_function, "continue");
        //Generate an initial jump to the for condition
        builder.build_unconditional_branch(condition_check);

        //Check loop condition
        builder.position_at_end(condition_check);
        let exp_gen = self.create_expr_generator(self.type_hint);
        let (_, counter_statement) = exp_gen.generate_expression(counter)?;
        let (_, end_statement) = exp_gen.generate_expression(end)?;

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
        let expression_generator = self.create_expr_generator(self.type_hint);
        let (_, step_by_value) = by_step
             .as_ref()
             .map_or_else(
                || 
                    expression_generator.generate_literal(
                        &Statement::LiteralInteger{ value: "1".to_string(), location: end.get_location().clone() }),
             |step| 
                expression_generator.generate_expression(&step))?;
             

        let next = builder
            .build_int_add(counter_statement.into_int_value(), step_by_value.into_int_value(), "tmpVar");
                    
        let ptr = expression_generator.generate_load_for(counter)?.ptr_value;
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
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {

        let current_function = self.function_context.function;
        //Continue
        let continue_block = self.llvm
            .context
            .append_basic_block(current_function, "continue");

        let basic_block = builder.get_insert_block().unwrap();
        let exp_gen = self.create_expr_generator(self.type_hint);

        let (_, selector_statement) = exp_gen.generate_expression(&*selector)?;
        let mut cases = Vec::new();

        //generate a int_value and a BasicBlock for every case-body
        for i in 0..conditional_blocks.len() {
            let conditional_block = &conditional_blocks[i];
            let basic_block = self.llvm
                .context
                .append_basic_block(current_function, "case");
            let (_, condition) = exp_gen.generate_expression(&*conditional_block.condition)?; //TODO : Is a type conversion needed here?
            builder.position_at_end(basic_block);
            self.generate_body(&conditional_block.body, builder)?;
            builder.build_unconditional_branch(continue_block);

            cases.push((condition.into_int_value(), basic_block));
        }

        let else_block = self.llvm
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
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
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
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
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
        condition: &Statement,
        body: &Vec<Statement>,
    ) -> Result<Option<BasicValueEnum>, CompileError> {
        let current_function = self.function_context.function;
        let condition_check = self.llvm
            .context
            .append_basic_block(current_function, "condition_check");
        let while_body = self.llvm
            .context
            .append_basic_block(current_function, "while_body");
        let continue_block = self.llvm
            .context
            .append_basic_block(current_function, "continue");

        //Check loop condition
        builder.position_at_end(condition_check);
        let (_, condition_value) = self.create_expr_generator(self.type_hint).generate_expression(condition)?;
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
    ) -> Result<(), CompileError> {
        let mut blocks = Vec::new();
        blocks.push(builder.get_insert_block().unwrap());
        let current_function = self.function_context.function;
        for _ in 1..conditional_blocks.len() {
            blocks.push(
                self.llvm.context
                    .append_basic_block(current_function, "branch"),
            );
        }

        let else_block = if else_body.len() > 0 {
            let result = self
                .llvm.context
                .append_basic_block(current_function, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = self
            .llvm.context
            .append_basic_block(current_function, "continue");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            builder.position_at_end(then_block);

            let (_,condition) = self.create_expr_generator(self.type_hint).generate_expression(&block.condition)?;
            let conditional_block = self
                .llvm.context
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
}
