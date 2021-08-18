// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{
    expression_generator::ExpressionCodeGenerator, llvm::Llvm, pou_generator::PouGenerator,
};
use crate::{ast::{
        flatten_expression_list, AstStatement, ConditionalBlock, Operator, PouType, SourceRange,
    }, codegen::{llvm_typesystem::cast_if_needed, LlvmTypedIndex}, compile_error::CompileError, index::{ImplementationIndexEntry, Index}, resolver::AnnotationMap, typesystem::{
        DataTypeInformation, RANGE_CHECK_LS_FN, RANGE_CHECK_LU_FN, RANGE_CHECK_S_FN,
        RANGE_CHECK_U_FN,
    }};
use inkwell::{
    basic_block::BasicBlock,
    values::{BasicValueEnum, FunctionValue},
    IntPredicate,
};
use std::ops::Range;

/// the full context when generating statements inside a POU
pub struct FunctionContext<'a> {
    /// the current pou's name. This means that a variable x may refer to "`linking_context`.x"
    pub linking_context: ImplementationIndexEntry,
    /// the llvm function to generate statements into
    pub function: FunctionValue<'a>,
}

/// the StatementCodeGenerator is used to generate statements (For, If, etc.) or expressions (references, literals, etc.)
pub struct StatementCodeGenerator<'a, 'b> {
    llvm: &'b Llvm<'a>,
    index: &'b Index,
    annotations: &'b AnnotationMap,
    pou_generator: &'b PouGenerator<'a, 'b>,
    pou_type: PouType,
    llvm_index: &'b LlvmTypedIndex<'a>,
    function_context: &'b FunctionContext<'a>,

    pub load_prefix: String,
    pub load_suffix: String,

    /// the block to jump to when you want to exit the loop
    pub current_loop_exit: Option<BasicBlock<'a>>,
    /// the block to jump to when you want to continue the loop
    pub current_loop_continue: Option<BasicBlock<'a>>,
}

impl<'a, 'b> StatementCodeGenerator<'a, 'b> {
    /// constructs a new StatementCodeGenerator
    pub fn new(
        llvm: &'b Llvm<'a>,
        index: &'b Index,
        annotations: &'b AnnotationMap,
        pou_generator: &'b PouGenerator<'a, 'b>,
        pou_type: PouType,
        llvm_index: &'b LlvmTypedIndex<'a>,
        linking_context: &'b FunctionContext<'a>,
    ) -> StatementCodeGenerator<'a, 'b> {
        StatementCodeGenerator {
            llvm,
            index,
            annotations,
            pou_generator,
            pou_type,
            llvm_index,
            function_context: linking_context,
            load_prefix: "load_".to_string(),
            load_suffix: "".to_string(),
            current_loop_exit: None,
            current_loop_continue: None,
        }
    }

    /// convinience method to create an expression-generator
    fn create_expr_generator(&'a self) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator::new(
            self.llvm,
            self.index,
            self.annotations,
            self.llvm_index,
            None,
            self.function_context,
        )
    }

    /// generates a list of statements
    pub fn generate_body(&self, statements: &[AstStatement]) -> Result<(), CompileError> {
        for s in statements {
            self.generate_statement(s)?;
        }
        Ok(())
    }

    /// some versions of llvm will crash on two consecutive return or
    /// unconditional jump statements. the solution is to insert another
    /// building block before the second one, so the don't directly
    /// follow each other. this is what we call a buffer block.
    fn generate_buffer_block(&self) {
        let builder = &self.llvm.builder;
        let buffer_block = self
            .llvm
            .context
            .insert_basic_block_after(builder.get_insert_block().unwrap(), "buffer_block");
        builder.position_at_end(buffer_block);
    }

    /// genertes a single statement
    ///
    /// - `statement` the statement to be generated
    pub fn generate_statement(&self, statement: &AstStatement) -> Result<(), CompileError> {
        match statement {
            AstStatement::EmptyStatement { .. } => {
                //nothing to generate
            }
            AstStatement::Assignment { left, right, .. } => {
                self.generate_assignment_statement(left, right)?;
            }
            AstStatement::ForLoopStatement {
                start,
                end,
                counter,
                body,
                by_step,
                ..
            } => {
                self.generate_for_statement(counter, start, end, by_step, body)?;
            }
            AstStatement::RepeatLoopStatement {
                condition, body, ..
            } => {
                self.generate_repeat_statement(condition, body)?;
            }
            AstStatement::WhileLoopStatement {
                condition, body, ..
            } => {
                self.generate_while_statement(condition, body)?;
            }
            AstStatement::IfStatement {
                blocks, else_block, ..
            } => {
                self.generate_if_statement(blocks, else_block)?;
            }
            AstStatement::CaseStatement {
                selector,
                case_blocks,
                else_block,
                ..
            } => {
                self.generate_case_statement(selector, case_blocks, else_block)?;
            }
            AstStatement::ReturnStatement { location, .. } => {
                self.pou_generator.generate_return_statement(
                    self.function_context,
                    self.llvm_index,
                    self.pou_type,
                    Some(location.clone()),
                )?;
                self.generate_buffer_block();
            }
            AstStatement::ExitStatement { location, .. } => {
                if let Some(exit_block) = &self.current_loop_exit {
                    self.llvm.builder.build_unconditional_branch(*exit_block);
                    self.generate_buffer_block();
                } else {
                    return Err(CompileError::CodeGenError {
                        message: "Cannot break out of loop when not inside a loop".into(),
                        location: location.clone(),
                    });
                }
            }
            AstStatement::ContinueStatement { location, .. } => {
                if let Some(cont_block) = &self.current_loop_continue {
                    self.llvm.builder.build_unconditional_branch(*cont_block);
                    self.generate_buffer_block();
                } else {
                    return Err(CompileError::CodeGenError {
                        message: "Cannot continue loop when not inside a loop".into(),
                        location: location.clone(),
                    });
                }
            }
            _ => {
                self.create_expr_generator()
                    .generate_expression(statement)?;
            }
        }
        Ok(())
    }

    /// generates an assignment statement _left_ := _right_
    ///
    /// `left_statement` the left side of the assignment
    /// `right_statement` the right side of the assignment
    pub fn generate_assignment_statement(
        &self,
        left_statement: &AstStatement,
        right_statement: &AstStatement,
    ) -> Result<(), CompileError> {
        let exp_gen = self.create_expr_generator();
        let left = exp_gen.generate_element_pointer(left_statement)?;
        // if the lhs-type is a subrange type we may need to generate a check-call
        // e.g. x := y,  ==> x := CheckSignedInt(y);
        let range_checked_right_side =
            if let DataTypeInformation::SubRange { sub_range, .. } = left.get_type_information() {
                // there is a sub-range defined, so we need to wrap the right side into the check function if it exists
                self.find_range_check_impolementation_for(left.get_type_information())
                    .map(|implementation| {
                        create_call_to_check_function_ast(
                            implementation.get_call_name().to_string(),
                            right_statement.clone(),
                            sub_range.clone(),
                            &left_statement.get_location(),
                        )
                    })
            } else {
                None
            };

        let (right_type, right) = if let Some(check_call) = range_checked_right_side {
            exp_gen.generate_expression(&check_call)?
        } else {
            exp_gen.generate_expression(right_statement)?
        };

        let cast_value = cast_if_needed(
            self.llvm,
            self.index,
            left.get_type_information(),
            right,
            &right_type,
            right_statement,
        )?;
        self.llvm.builder.build_store(left.ptr_value, cast_value);
        Ok(())
    }

    /// returns the implementation of the sub-range-check-function for a variable of the given dataType
    fn find_range_check_impolementation_for(
        &self,
        data_type: &DataTypeInformation,
    ) -> Option<&ImplementationIndexEntry> {
        let effective_type = self.index.find_effective_type_information(data_type);
        match effective_type {
            Some(DataTypeInformation::Integer { signed, size, .. }) if *signed && *size <= 32 => {
                self.index.find_implementation(RANGE_CHECK_S_FN)
            }
            Some(DataTypeInformation::Integer { signed, size, .. }) if *signed && *size > 32 => {
                self.index.find_implementation(RANGE_CHECK_LS_FN)
            }
            Some(DataTypeInformation::Integer { signed, size, .. }) if !*signed && *size <= 32 => {
                self.index.find_implementation(RANGE_CHECK_U_FN)
            }
            Some(DataTypeInformation::Integer { signed, size, .. }) if !*signed && *size > 32 => {
                self.index.find_implementation(RANGE_CHECK_LU_FN)
            }
            _ => None,
        }
    }

    /// generates a for-loop statement
    ///
    /// FOR `counter` := `start` TO `end` BY `by_step` DO
    ///
    /// - `counter` the counter variable
    /// - `start` the value indicating the start of the for loop
    /// - `end` the value indicating the end of the for loop
    /// - `by_step` the step of the loop
    /// - `body` the statements inside the for-loop
    fn generate_for_statement(
        &self,
        counter: &AstStatement,
        start: &AstStatement,
        end: &AstStatement,
        by_step: &Option<Box<AstStatement>>,
        body: &[AstStatement],
    ) -> Result<(), CompileError> {
        let builder = &self.llvm.builder;
        let current_function = self.function_context.function;
        self.generate_assignment_statement(counter, start)?;
        let condition_check = self
            .llvm
            .context
            .append_basic_block(current_function, "condition_check");
        let for_body = self
            .llvm
            .context
            .append_basic_block(current_function, "for_body");
        let increment_block = self
            .llvm
            .context
            .append_basic_block(current_function, "increment");
        let continue_block = self
            .llvm
            .context
            .append_basic_block(current_function, "continue");

        //Generate an initial jump to the for condition
        builder.build_unconditional_branch(condition_check);

        //Check loop condition
        builder.position_at_end(condition_check);
        let exp_gen = self.create_expr_generator();
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
        let body_generator = StatementCodeGenerator {
            current_loop_exit: Some(continue_block),
            current_loop_continue: Some(increment_block),
            load_prefix: self.load_prefix.clone(),
            load_suffix: self.load_suffix.clone(),
            ..*self
        };
        body_generator.generate_body(body)?;
        builder.build_unconditional_branch(increment_block);

        //Increment
        builder.position_at_end(increment_block);
        let expression_generator = self.create_expr_generator();
        let (_, step_by_value) = by_step.as_ref().map_or_else(
            || {
                expression_generator.generate_literal(&AstStatement::LiteralInteger {
                    value: 1,
                    location: end.get_location(),
                    id: 0, //TODO
                })
            },
            |step| expression_generator.generate_expression(step),
        )?;

        let next = builder.build_int_add(
            counter_statement.into_int_value(),
            step_by_value.into_int_value(),
            "tmpVar",
        );

        let ptr = expression_generator
            .generate_element_pointer(counter)?
            .ptr_value;
        builder.build_store(ptr, next);

        //Loop back
        builder.build_unconditional_branch(condition_check);

        //Continue
        builder.position_at_end(continue_block);

        Ok(())
    }

    /// genertes a case statement
    ///
    /// CASE selector OF
    /// conditional_block#1:
    /// conditional_block#2:
    /// END_CASE;
    ///
    /// - `selector` the case's selector expression
    /// - `conditional_blocks` all case-blocks including the condition and the body
    /// - `else_body` the statements in the else-block
    fn generate_case_statement(
        &self,
        selector: &AstStatement,
        conditional_blocks: &[ConditionalBlock],
        else_body: &[AstStatement],
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
        let builder = &self.llvm.builder;
        let current_function = self.function_context.function;
        //Continue
        let continue_block = self
            .llvm
            .context
            .append_basic_block(current_function, "continue");

        let basic_block = builder.get_insert_block().unwrap();
        let exp_gen = self.create_expr_generator();
        let (selector_type, selector_statement) = exp_gen.generate_expression(&*selector)?;
        //re-brand the expression generator to use the selector's type when generating literals
        let exp_gen = ExpressionCodeGenerator::new(
            self.llvm,
            self.index,
            self.annotations,
            self.llvm_index,
            Some(selector_type),
            self.function_context,
        );

        let mut cases = Vec::new();
        let else_block = self
            .llvm
            .context
            .append_basic_block(current_function, "else");
        let mut current_else_block = else_block;

        for conditional_block in conditional_blocks {
            //craete a block for the case's body
            let case_block = self.llvm.context.prepend_basic_block(else_block, "case");

            //flatten the expression list into a vector of expressions
            let expressions = flatten_expression_list(&*conditional_block.condition);
            for s in expressions {
                if let AstStatement::RangeStatement { start, end, .. } = s {
                    //if this is a range statement, we generate an if (x >= start && x <= end) then the else-section
                    builder.position_at_end(current_else_block);
                    // since the if's generate additional blocks, we use the last one as the else-section
                    current_else_block = self.generate_case_range_condition(
                        selector,
                        start.as_ref(),
                        end.as_ref(),
                        case_block,
                    )?;
                } else {
                    // this should be a a literal or a reference to a constant
                    builder.position_at_end(basic_block);
                    let (_, condition) = exp_gen.generate_expression(s)?; //TODO : Is a type conversion needed here?
                                                                          // collect all literal case blocks to pass to the llvm switch-statement
                    cases.push((condition.into_int_value(), case_block));
                }
            }
            //generate the case's body
            builder.position_at_end(case_block);
            self.generate_body(&conditional_block.body)?;
            // skiop all other case-bodies
            builder.build_unconditional_branch(continue_block);
        }
        // current-else is the last else-block generated by the range-expressions
        builder.position_at_end(current_else_block);
        self.generate_body(else_body)?;
        builder.build_unconditional_branch(continue_block);
        continue_block.move_after(current_else_block).unwrap();

        // now that we collected all cases, go back to the initial block and generate the switch-statement
        builder.position_at_end(basic_block);
        builder.build_switch(selector_statement.into_int_value(), else_block, &cases);

        builder.position_at_end(continue_block);
        Ok(None)
    }

    /// returns the new block to use as else
    ///
    ///
    fn generate_case_range_condition(
        &self,
        selector: &AstStatement,
        start: &AstStatement,
        end: &AstStatement,
        match_block: BasicBlock,
    ) -> Result<BasicBlock, CompileError> {
        let builder = &self.llvm.builder;

        let range_then = self
            .llvm
            .context
            .insert_basic_block_after(builder.get_insert_block().unwrap(), "range_then");
        let range_else = self
            .llvm
            .context
            .insert_basic_block_after(range_then, "range_else");
        let exp_gen = self.create_expr_generator();
        let lower_bound = {
            let (type_info, start_val) = exp_gen.generate_expression(start)?;
            let (_, selector_val) = exp_gen.generate_expression(selector)?;
            let (_, lower_bound_condition) = exp_gen.create_llvm_int_binary_expression(
                &Operator::GreaterOrEqual,
                selector_val,
                start_val,
                &type_info,
            );
            lower_bound_condition
        };

        //jmp to continue if the value is smaller than start
        builder.build_conditional_branch(lower_bound.into_int_value(), range_then, range_else);
        builder.position_at_end(range_then);
        let upper_bound = {
            let (type_info, end_val) = exp_gen.generate_expression(end)?;
            let (_, selector_val) = exp_gen.generate_expression(selector)?;
            let (_, upper_bound_condition) = exp_gen.create_llvm_int_binary_expression(
                &Operator::LessOrEqual,
                selector_val,
                end_val,
                &type_info,
            );
            upper_bound_condition
        };
        self.llvm.builder.build_conditional_branch(
            upper_bound.into_int_value(),
            match_block,
            range_else,
        );
        Ok(range_else)
    }

    /// generates a while statement
    ///
    /// WHILE condition DO
    ///     body
    /// END_WHILE
    ///
    /// - `condition` the while's condition
    /// - `body` the while's body statements
    fn generate_while_statement(
        &self,
        condition: &AstStatement,
        body: &[AstStatement],
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().unwrap();
        self.generate_base_while_statement(condition, body)?;

        let continue_block = builder.get_insert_block().unwrap();

        let condition_block = basic_block.get_next_basic_block().unwrap();
        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(condition_block);

        builder.position_at_end(continue_block);
        Ok(None)
    }

    /// generates a repeat statement
    ///
    ///
    /// REPEAT
    ///     body
    /// UNTIL condition END_REPEAT;
    ///
    /// - `condition` the repeat's condition
    /// - `body` the repeat's body statements
    fn generate_repeat_statement(
        &self,
        condition: &AstStatement,
        body: &[AstStatement],
    ) -> Result<Option<BasicValueEnum<'a>>, CompileError> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().unwrap();
        self.generate_base_while_statement(condition, body)?;

        let continue_block = builder.get_insert_block().unwrap();

        let while_block = continue_block.get_previous_basic_block().unwrap();
        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(while_block);

        builder.position_at_end(continue_block);
        Ok(None)
    }

    /// utility method for while and repeat loops
    fn generate_base_while_statement(
        &self,
        condition: &AstStatement,
        body: &[AstStatement],
    ) -> Result<Option<BasicValueEnum>, CompileError> {
        let builder = &self.llvm.builder;
        let current_function = self.function_context.function;
        let condition_check = self
            .llvm
            .context
            .append_basic_block(current_function, "condition_check");
        let while_body = self
            .llvm
            .context
            .append_basic_block(current_function, "while_body");
        let continue_block = self
            .llvm
            .context
            .append_basic_block(current_function, "continue");

        //Check loop condition
        builder.position_at_end(condition_check);
        let (_, condition_value) = self
            .create_expr_generator()
            .generate_expression(condition)?;
        builder.build_conditional_branch(
            condition_value.into_int_value(),
            while_body,
            continue_block,
        );

        //Enter the for loop
        builder.position_at_end(while_body);
        let body_generator = StatementCodeGenerator {
            current_loop_exit: Some(continue_block),
            current_loop_continue: Some(condition_check),
            load_prefix: self.load_prefix.clone(),
            load_suffix: self.load_suffix.clone(),
            ..*self
        };
        body_generator.generate_body(body)?;
        //Loop back
        builder.build_unconditional_branch(condition_check);

        //Continue
        builder.position_at_end(continue_block);
        Ok(None)
    }

    /// generates an IF-Statement
    ///
    /// - `conditional_blocks` a list of conditions + bodies for every if  (respectivle else-if)
    /// - `else_body` the list of statements in the else-block
    fn generate_if_statement(
        &self,
        conditional_blocks: &[ConditionalBlock],
        else_body: &[AstStatement],
    ) -> Result<(), CompileError> {
        let builder = &self.llvm.builder;
        let mut blocks = vec![builder.get_insert_block().unwrap()];
        let current_function = self.function_context.function;
        for _ in 1..conditional_blocks.len() {
            blocks.push(
                self.llvm
                    .context
                    .append_basic_block(current_function, "branch"),
            );
        }

        let else_block = if !else_body.is_empty() {
            let result = self
                .llvm
                .context
                .append_basic_block(current_function, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = self
            .llvm
            .context
            .append_basic_block(current_function, "continue");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            builder.position_at_end(then_block);

            let (_, condition) = self
                .create_expr_generator()
                .generate_expression(&block.condition)?;
            let conditional_block = self
                .llvm
                .context
                .prepend_basic_block(else_block, "condition_body");

            //Generate if statement condition
            builder.build_conditional_branch(
                condition.into_int_value(),
                conditional_block,
                else_block,
            );

            //Generate if statement content

            builder.position_at_end(conditional_block);
            self.generate_body(&block.body)?;
            builder.build_unconditional_branch(continue_block);
        }
        //Else

        if let Some(else_block) = else_block {
            builder.position_at_end(else_block);
            self.generate_body(else_body)?;
            builder.build_unconditional_branch(continue_block);
        }
        //Continue
        builder.position_at_end(continue_block);
        Ok(())
    }
}

fn create_call_to_check_function_ast(
    check_function_name: String,
    parameter: AstStatement,
    sub_range: Range<AstStatement>,
    location: &SourceRange,
) -> AstStatement {
    AstStatement::CallStatement {
        operator: Box::new(AstStatement::Reference {
            name: check_function_name,
            location: location.clone(),
            id: 0, //TODO
        }),
        parameters: Box::new(Some(AstStatement::ExpressionList {
            expressions: vec![parameter, sub_range.start, sub_range.end],
            id: 0, //TODO
        })),
        location: location.clone(),
        id: 0, //TODO
    }
}
