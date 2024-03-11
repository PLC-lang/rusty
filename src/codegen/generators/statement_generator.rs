use std::collections::HashMap;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{
    expression_generator::{to_i1, ExpressionCodeGenerator},
    llvm::Llvm,
};
use crate::{
    codegen::{debug::Debug, llvm_typesystem::cast_if_needed},
    codegen::{debug::DebugBuilderEnum, LlvmTypedIndex},
    index::{ImplementationIndexEntry, Index},
    resolver::{AnnotationMap, AstAnnotations, StatementAnnotation},
    typesystem::{self, DataTypeInformation},
};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    values::{BasicValueEnum, FunctionValue, PointerValue},
};
use plc_ast::{
    ast::{
        flatten_expression_list, AstFactory, AstNode, AstStatement, JumpStatement, LabelStatement, Operator,
        ReferenceAccess, ReferenceExpr,
    },
    control_statements::{AstControlStatement, ConditionalBlock, ReturnStatement},
};
use plc_diagnostics::diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR};
use plc_source::source_location::SourceLocation;

/// the full context when generating statements inside a POU
pub struct FunctionContext<'ink, 'b> {
    /// the current pou's name. This means that a variable x may refer to "`linking_context`.x"
    pub linking_context: &'b ImplementationIndexEntry,
    /// the llvm function to generate statements into
    pub function: FunctionValue<'ink>,
    /// The blocks/labels this function can use
    pub blocks: HashMap<String, BasicBlock<'ink>>,
}

/// the StatementCodeGenerator is used to generate statements (For, If, etc.) or expressions (references, literals, etc.)
pub struct StatementCodeGenerator<'a, 'b> {
    llvm: &'b Llvm<'a>,
    index: &'b Index,
    annotations: &'b AstAnnotations,
    llvm_index: &'b LlvmTypedIndex<'a>,
    function_context: &'b FunctionContext<'a, 'b>,

    pub load_prefix: String,
    pub load_suffix: String,

    /// the block to jump to when you want to exit the loop
    pub current_loop_exit: Option<BasicBlock<'a>>,
    /// the block to jump to when you want to continue the loop
    pub current_loop_continue: Option<BasicBlock<'a>>,

    pub debug: &'b DebugBuilderEnum<'a>,
}

impl<'a, 'b> StatementCodeGenerator<'a, 'b> {
    /// constructs a new StatementCodeGenerator
    pub fn new(
        llvm: &'b Llvm<'a>,
        index: &'b Index,
        annotations: &'b AstAnnotations,
        llvm_index: &'b LlvmTypedIndex<'a>,
        linking_context: &'b FunctionContext<'a, 'b>,
        debug: &'b DebugBuilderEnum<'a>,
    ) -> StatementCodeGenerator<'a, 'b> {
        StatementCodeGenerator {
            llvm,
            index,
            annotations,
            llvm_index,
            function_context: linking_context,
            load_prefix: "load_".to_string(),
            load_suffix: "".to_string(),
            current_loop_exit: None,
            current_loop_continue: None,
            debug,
        }
    }

    /// convinience method to create an expression-generator
    fn create_expr_generator(&'a self) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator::new(
            self.llvm,
            self.index,
            self.annotations,
            self.llvm_index,
            self.function_context,
            self.debug,
        )
    }

    /// generates a list of statements
    pub fn generate_body(&self, statements: &[AstNode]) -> Result<(), Diagnostic> {
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
        let (builder, _, context) = self.get_llvm_deps();
        let buffer_block = context
            .insert_basic_block_after(builder.get_insert_block().expect(INTERNAL_LLVM_ERROR), "buffer_block");
        builder.position_at_end(buffer_block);
    }

    /// genertes a single statement
    ///
    /// - `statement` the statement to be generated
    pub fn generate_statement(&self, statement: &AstNode) -> Result<(), Diagnostic> {
        match statement.get_stmt() {
            AstStatement::EmptyStatement(..) => {
                //nothing to generate
            }
            AstStatement::Assignment(data, ..) => {
                self.generate_assignment_statement(&data.left, &data.right)?;
            }

            AstStatement::ControlStatement(ctl_statement, ..) => {
                self.generate_control_statement(ctl_statement)?
            }
            AstStatement::ReturnStatement(ReturnStatement { condition }) => match condition {
                Some(condition) => {
                    self.generate_conditional_return(statement, condition)?;
                }
                None => {
                    self.register_debug_location(statement);
                    self.generate_return_statement()?;
                    self.generate_buffer_block(); // XXX(volsa): This is not needed on x86 but if removed segfaults on ARM
                }
            },
            AstStatement::LabelStatement(LabelStatement { name }) => {
                if let Some(block) = self.function_context.blocks.get(name) {
                    //unconditionally jump to the label
                    self.register_debug_location(statement);
                    self.llvm.builder.build_unconditional_branch(*block);
                    //Place the current instert block at the label statement
                    self.llvm.builder.position_at_end(*block);
                }
            }
            AstStatement::JumpStatement(JumpStatement { condition, .. }) => {
                //Find the label to jump to
                let Some(then_block) = self.annotations.get(statement).and_then(|label| {
                    if let StatementAnnotation::Label { name } = label {
                        self.function_context.blocks.get(name)
                    } else {
                        None
                    }
                }) else {
                    return Err(Diagnostic::codegen_error(
                        "Could not find label for {statement:?}",
                        statement.get_location(),
                    ));
                };
                //Set current location as else block
                let current_block = self.llvm.builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
                let else_block = self.llvm.context.insert_basic_block_after(current_block, "else_block");

                self.register_debug_location(condition);
                let expression_generator = self.create_expr_generator();
                let condition = expression_generator.generate_expression(condition)?;

                self.register_debug_location(statement);
                self.llvm.builder.build_conditional_branch(
                    condition.into_int_value(),
                    *then_block,
                    else_block,
                );
                // Make sure further code is at the else block
                self.llvm.builder.position_at_end(else_block);
            }
            AstStatement::ExitStatement(_) => {
                if let Some(exit_block) = &self.current_loop_exit {
                    self.register_debug_location(statement);
                    self.llvm.builder.build_unconditional_branch(*exit_block);
                    self.generate_buffer_block();
                } else {
                    return Err(Diagnostic::codegen_error(
                        "Cannot break out of loop when not inside a loop",
                        statement.get_location(),
                    ));
                }
            }
            AstStatement::ContinueStatement(_) => {
                if let Some(cont_block) = &self.current_loop_continue {
                    self.llvm.builder.build_unconditional_branch(*cont_block);
                    self.generate_buffer_block();
                } else {
                    return Err(Diagnostic::codegen_error(
                        "Cannot continue loop when not inside a loop",
                        statement.get_location(),
                    ));
                }
            }
            _ => {
                self.create_expr_generator().generate_expression(statement)?;
            }
        }
        Ok(())
    }

    /// genertes a single statement
    ///
    /// - `statement` the control statement to be generated
    pub fn generate_control_statement(&self, statement: &AstControlStatement) -> Result<(), Diagnostic> {
        match statement {
            AstControlStatement::If(ifstmt) => self.generate_if_statement(&ifstmt.blocks, &ifstmt.else_block),
            AstControlStatement::ForLoop(for_stmt) => self.generate_for_statement(
                &for_stmt.counter,
                &for_stmt.start,
                &for_stmt.end,
                &for_stmt.by_step,
                &for_stmt.body,
            ),
            AstControlStatement::WhileLoop(stmt) => {
                self.generate_while_statement(&stmt.condition, &stmt.body)
            }
            AstControlStatement::RepeatLoop(stmt) => {
                self.generate_repeat_statement(&stmt.condition, &stmt.body)
            }
            AstControlStatement::Case(stmt) => {
                self.generate_case_statement(&stmt.selector, &stmt.case_blocks, &stmt.else_block)
            }
        }
    }

    /// generates an assignment statement _left_ := _right_
    ///
    /// `left_statement` the left side of the assignment
    /// `right_statement` the right side of the assignment
    pub fn generate_assignment_statement(
        &self,
        left_statement: &AstNode,
        right_statement: &AstNode,
    ) -> Result<(), Diagnostic> {
        //Register any debug info for the store
        self.register_debug_location(left_statement);
        //TODO: Looks hacky, the strings will be similar so we should look into making the assignment a bit nicer.
        if left_statement.has_direct_access() {
            return self.generate_direct_access_assignment(left_statement, right_statement);
        }
        //TODO: Also hacky but for now we cannot generate assignments for hardware access
        if matches!(left_statement.get_stmt(), AstStatement::HardwareAccess { .. }) {
            return Ok(());
        }
        let exp_gen = self.create_expr_generator();
        let left: PointerValue = exp_gen.generate_expression_value(left_statement).and_then(|it| {
            it.get_basic_value_enum().try_into().map_err(|err| {
                Diagnostic::codegen_error(format!("{err:?}").as_str(), left_statement.get_location())
            })
        })?;

        let left_type = exp_gen.get_type_hint_info_for(left_statement)?;
        // if the lhs-type is a subrange type we may need to generate a check-call
        // e.g. x := y,  ==> x := CheckSignedInt(y);
        let range_checked_right_side = if let DataTypeInformation::SubRange { .. } = left_type {
            // there is a sub-range defined, so we need to wrap the right side into the check function if it exists
            self.annotations.get_hidden_function_call(right_statement)
        } else {
            None
        };

        let right_statement = range_checked_right_side.unwrap_or(right_statement);

        exp_gen.generate_store(left, left_type, right_statement)?;
        Ok(())
    }

    fn register_debug_location(&self, statement: &AstNode) {
        let line = statement.get_location().get_line_plus_one();
        let column = statement.get_location().get_column();
        self.debug.set_debug_location(self.llvm, &self.function_context.function, line, column);
    }

    fn generate_direct_access_assignment(
        &self,
        left_statement: &AstNode,
        right_statement: &AstNode,
    ) -> Result<(), Diagnostic> {
        //TODO : Validation
        let exp_gen = self.create_expr_generator();

        // given a complex direct-access assignemnt: a.b.c.%W3,%X1
        // we want to deconstruct the targe-part (a.b.c) and the direct-access sequence (%W3.%X1)
        let Some((target, access_sequence)) = collect_base_and_direct_access_for_assignment(left_statement)
        else {
            unreachable!("Invalid direct-access expression: {left_statement:#?}")
        };

        let left_type = exp_gen.get_type_hint_for(target)?;
        let right_type = exp_gen.get_type_hint_for(right_statement)?;

        //special case if we deal with a single bit, then we need to switch to a faked u1 type
        let right_type =
            if let DataTypeInformation::Integer { semantic_size: Some(typesystem::U1_SIZE), .. } =
                *right_type.get_type_information()
            {
                self.index.get_type_or_panic(typesystem::U1_TYPE)
            } else {
                right_type
            };

        //Left pointer
        let left_expression_value = exp_gen.generate_expression_value(target)?;
        let left_value = left_expression_value.as_r_value(self.llvm, None).into_int_value();
        let left = left_expression_value.get_basic_value_enum().into_pointer_value();
        //Build index
        if let Some((element, direct_access)) = access_sequence.split_first() {
            let mut rhs = if let AstStatement::DirectAccess(data, ..) = element.get_stmt() {
                exp_gen.generate_direct_access_index(
                    &data.access,
                    &data.index,
                    right_type.get_type_information(),
                    left_type,
                )
            } else {
                //TODO: using the global context we could get a slice here
                Err(Diagnostic::new(format!("{element:?} not a direct access"))
                    .with_error_code("E055")
                    .with_location(element.get_location()))
            }?;
            for element in direct_access {
                let rhs_next = if let AstStatement::DirectAccess(data, ..) = element.get_stmt() {
                    exp_gen.generate_direct_access_index(
                        &data.access,
                        &data.index,
                        right_type.get_type_information(),
                        left_type,
                    )
                } else {
                    //TODO: using the global context we could get a slice here
                    Err(Diagnostic::new(&format!("{element:?} not a direct access"))
                        .with_error_code("E055")
                        .with_location(element.get_location()))
                }?;
                rhs = self.llvm.builder.build_int_add(rhs, rhs_next, "");
            }
            //Build mask for the index
            //Get the target bit type as all ones
            let rhs_type = self.llvm_index.get_associated_type(right_type.get_name())?.into_int_type();
            let ones = rhs_type.const_all_ones();
            //Extend the mask to the target type
            let extended_mask = self.llvm.builder.build_int_z_extend(ones, left_value.get_type(), "ext");
            //Position the ones in their correct locations
            let shifted_mask = self.llvm.builder.build_left_shift(extended_mask, rhs, "shift");
            //Invert the mask
            let mask = self.llvm.builder.build_not(shifted_mask, "invert");
            //And the result with the mask to erase the set bits at the target location
            let and_value = self.llvm.builder.build_and(left_value, mask, "erase");

            //Generate an expression for the right size
            let right = exp_gen.generate_expression(right_statement)?;
            //Cast the right side to the left side type
            let lhs = cast_if_needed!(self, left_type, right_type, right, None).into_int_value();
            //Shift left by the direct access
            let value = self.llvm.builder.build_left_shift(lhs, rhs, "value");

            //OR the result and store it in the left side
            let or_value = self.llvm.builder.build_or(and_value, value, "or");
            self.llvm.builder.build_store(left, or_value);
        } else {
            unreachable!();
        }
        Ok(())
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
        counter: &AstNode,
        start: &AstNode,
        end: &AstNode,
        by_step: &Option<Box<AstNode>>,
        body: &[AstNode],
    ) -> Result<(), Diagnostic> {
        let (builder, current_function, context) = self.get_llvm_deps();
        self.generate_assignment_statement(counter, start)?;
        let condition_check = context.append_basic_block(current_function, "condition_check");
        let for_body = context.append_basic_block(current_function, "for_body");
        let increment_block = context.append_basic_block(current_function, "increment");
        let continue_block = context.append_basic_block(current_function, "continue");

        //Generate an initial jump to the for condition
        builder.build_unconditional_branch(condition_check);

        //Check loop condition
        builder.position_at_end(condition_check);
        let exp_gen = self.create_expr_generator();
        let counter_statement = exp_gen.generate_expression(counter)?;

        //.                                                           /            and_2                \
        //.                  /             and 1               \
        //.                   (counter_end_le && counter_start_ge) || (counter_end_ge && counter_start_le)
        let or_eval = self.generate_compare_expression(counter, end, start, &exp_gen)?;

        builder.build_conditional_branch(to_i1(or_eval.into_int_value(), builder), for_body, continue_block);

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
        let step_by_value = by_step.as_ref().map_or_else(
            || {
                self.llvm.create_const_numeric(
                    &counter_statement.get_type(),
                    "1",
                    SourceLocation::undefined(),
                )
            },
            |step| {
                self.register_debug_location(step);
                expression_generator.generate_expression(step)
            },
        )?;

        let next = builder.build_int_add(
            counter_statement.into_int_value(),
            step_by_value.into_int_value(),
            "tmpVar",
        );

        let ptr = expression_generator.generate_lvalue(counter)?;
        builder.build_store(ptr, next);

        //Loop back
        builder.build_unconditional_branch(condition_check);

        //Continue
        builder.position_at_end(continue_block);

        Ok(())
    }

    fn generate_compare_expression(
        &'a self,
        counter: &AstNode,
        end: &AstNode,
        start: &AstNode,
        exp_gen: &'a ExpressionCodeGenerator,
    ) -> Result<BasicValueEnum<'a>, Diagnostic> {
        let bool_id = self.annotations.get_bool_id();
        let counter_end_ge = AstFactory::create_binary_expression(
            counter.clone(),
            Operator::GreaterOrEqual,
            end.clone(),
            bool_id,
        );
        let counter_start_ge = AstFactory::create_binary_expression(
            counter.clone(),
            Operator::GreaterOrEqual,
            start.clone(),
            bool_id,
        );
        let counter_end_le = AstFactory::create_binary_expression(
            counter.clone(),
            Operator::LessOrEqual,
            end.clone(),
            bool_id,
        );
        let counter_start_le = AstFactory::create_binary_expression(
            counter.clone(),
            Operator::LessOrEqual,
            start.clone(),
            bool_id,
        );
        let and_1 =
            AstFactory::create_binary_expression(counter_end_le, Operator::And, counter_start_ge, bool_id);
        let and_2 =
            AstFactory::create_binary_expression(counter_end_ge, Operator::And, counter_start_le, bool_id);
        let or = AstFactory::create_binary_expression(and_1, Operator::Or, and_2, bool_id);

        self.register_debug_location(&or);
        let or_eval = exp_gen.generate_expression(&or)?;
        Ok(or_eval)
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
        selector: &AstNode,
        conditional_blocks: &[ConditionalBlock],
        else_body: &[AstNode],
    ) -> Result<(), Diagnostic> {
        let (builder, current_function, context) = self.get_llvm_deps();
        //Continue
        let continue_block = context.append_basic_block(current_function, "continue");

        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let exp_gen = self.create_expr_generator();
        self.register_debug_location(selector);
        let selector_statement = exp_gen.generate_expression(selector)?;

        let mut cases = Vec::new();
        let else_block = context.append_basic_block(current_function, "else");
        let mut current_else_block = else_block;

        for conditional_block in conditional_blocks {
            //craete a block for the case's body
            let case_block = context.prepend_basic_block(else_block, "case");

            //flatten the expression list into a vector of expressions
            let expressions = flatten_expression_list(&conditional_block.condition);
            for s in expressions {
                if let AstStatement::RangeStatement(data, ..) = s.get_stmt() {
                    //if this is a range statement, we generate an if (x >= start && x <= end) then the else-section
                    builder.position_at_end(current_else_block);
                    // since the if's generate additional blocks, we use the last one as the else-section
                    current_else_block = self.generate_case_range_condition(
                        selector,
                        data.start.as_ref(),
                        data.end.as_ref(),
                        case_block,
                    )?;
                } else {
                    // this should be a a literal or a reference to a constant
                    builder.position_at_end(basic_block);
                    let condition = exp_gen.generate_expression(s)?; //TODO : Is a type conversion needed here?
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
        continue_block.move_after(current_else_block).expect(INTERNAL_LLVM_ERROR);

        // now that we collected all cases, go back to the initial block and generate the switch-statement
        builder.position_at_end(basic_block);
        builder.build_switch(selector_statement.into_int_value(), else_block, &cases);

        builder.position_at_end(continue_block);
        Ok(())
    }

    /// returns the new block to use as else
    ///
    ///
    fn generate_case_range_condition(
        &self,
        selector: &AstNode,
        start: &AstNode,
        end: &AstNode,
        match_block: BasicBlock,
    ) -> Result<BasicBlock, Diagnostic> {
        let (builder, _, context) = self.get_llvm_deps();

        let range_then = context
            .insert_basic_block_after(builder.get_insert_block().expect(INTERNAL_LLVM_ERROR), "range_then");
        let range_else = context.insert_basic_block_after(range_then, "range_else");
        let exp_gen = self.create_expr_generator();
        let lower_bound = {
            self.register_debug_location(start);
            let start_val = exp_gen.generate_expression(start)?;
            self.register_debug_location(selector);
            let selector_val = exp_gen.generate_expression(selector)?;
            exp_gen.create_llvm_int_binary_expression(&Operator::GreaterOrEqual, selector_val, start_val)
        };

        //jmp to continue if the value is smaller than start
        builder.build_conditional_branch(
            to_i1(lower_bound.into_int_value(), builder),
            range_then,
            range_else,
        );
        builder.position_at_end(range_then);
        let upper_bound = {
            self.register_debug_location(end);
            let end_val = exp_gen.generate_expression(end)?;
            self.register_debug_location(selector);
            let selector_val = exp_gen.generate_expression(selector)?;
            exp_gen.create_llvm_int_binary_expression(&Operator::LessOrEqual, selector_val, end_val)
        };
        builder.build_conditional_branch(
            to_i1(upper_bound.into_int_value(), builder),
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
    fn generate_while_statement(&self, condition: &AstNode, body: &[AstNode]) -> Result<(), Diagnostic> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let (condition_block, _) = self.generate_base_while_statement(condition, body)?;

        let continue_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);

        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(condition_block);

        builder.position_at_end(continue_block);
        Ok(())
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
    fn generate_repeat_statement(&self, condition: &AstNode, body: &[AstNode]) -> Result<(), Diagnostic> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);

        // for REPEAT .. UNTIL blocks, the abort condition logic needs to be inverted to be correct
        let condition = AstFactory::create_not_expression(
            condition.clone(),
            condition.get_location(),
            condition.get_id(),
        );
        let (_, while_block) = self.generate_base_while_statement(&condition, body)?;

        let continue_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);

        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(while_block);

        builder.position_at_end(continue_block);
        Ok(())
    }

    /// utility method for while and repeat loops
    fn generate_base_while_statement(
        &self,
        condition: &AstNode,
        body: &[AstNode],
    ) -> Result<(BasicBlock, BasicBlock), Diagnostic> {
        let (builder, current_function, context) = self.get_llvm_deps();
        let condition_check = context.append_basic_block(current_function, "condition_check");
        let while_body = context.append_basic_block(current_function, "while_body");
        let continue_block = context.append_basic_block(current_function, "continue");

        //Check loop condition
        builder.position_at_end(condition_check);
        self.register_debug_location(condition);
        let condition_value = self.create_expr_generator().generate_expression(condition)?;
        builder.build_conditional_branch(
            to_i1(condition_value.into_int_value(), builder),
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
        Ok((condition_check, while_body))
    }

    /// generates an IF-Statement
    ///
    /// - `conditional_blocks` a list of conditions + bodies for every if  (respectivle else-if)
    /// - `else_body` the list of statements in the else-block
    fn generate_if_statement(
        &self,
        conditional_blocks: &[ConditionalBlock],
        else_body: &[AstNode],
    ) -> Result<(), Diagnostic> {
        let (builder, current_function, context) = self.get_llvm_deps();
        let mut blocks = vec![builder.get_insert_block().expect(INTERNAL_LLVM_ERROR)];
        for _ in 1..conditional_blocks.len() {
            blocks.push(context.append_basic_block(current_function, "branch"));
        }

        let else_block = if !else_body.is_empty() {
            let result = context.append_basic_block(current_function, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = context.append_basic_block(current_function, "continue");
        blocks.push(continue_block);

        for (i, block) in conditional_blocks.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            builder.position_at_end(then_block);

            self.register_debug_location(&block.condition);
            let condition = self.create_expr_generator().generate_expression(&block.condition)?;
            let conditional_block = context.prepend_basic_block(else_block, "condition_body");

            //Generate if statement condition
            builder.build_conditional_branch(
                to_i1(condition.into_int_value(), builder),
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

    /// generates the function's return statement only if the given pou_type is a `PouType::Function`
    ///
    /// a function returns the value of the local variable that has the function's name
    pub fn generate_return_statement(&self) -> Result<(), Diagnostic> {
        if let Some(ret_v) =
            self.index.find_return_variable(self.function_context.linking_context.get_type_name())
        {
            if self
                .index
                .find_effective_type_by_name(ret_v.get_type_name())
                .map(|it| it.is_aggregate_type())
                .unwrap_or(false)
            {
                //generate return void
                self.llvm.builder.build_return(None);
            } else {
                // renerate return statement
                let call_name = self.function_context.linking_context.get_call_name();
                let var_name = format!("{call_name}_ret"); // TODO: Naming convention (see plc_util/src/convention.rs)
                let ret_name = ret_v.get_qualified_name();
                let value_ptr =
                    self.llvm_index.find_loaded_associated_variable_value(ret_name).ok_or_else(|| {
                        Diagnostic::codegen_error(
                            format!("Cannot generate return variable for {call_name:}"),
                            SourceLocation::undefined(),
                        )
                    })?;
                let loaded_value = self.llvm.load_pointer(&value_ptr, var_name.as_str());
                self.llvm.builder.build_return(Some(&loaded_value));
            }
        } else {
            self.llvm.builder.build_return(None);
        }
        Ok(())
    }

    /// Generates LLVM IR for conditional returns, which return if a given condition evaluates to true and
    /// does nothing otherwise.
    pub fn generate_conditional_return(
        &'a self,
        statement: &AstNode,
        condition: &AstNode,
    ) -> Result<(), Diagnostic> {
        let expression_generator = self.create_expr_generator();

        self.register_debug_location(condition);
        let condition = expression_generator.generate_expression(condition)?;

        let then_block = self.llvm.context.append_basic_block(self.function_context.function, "then_block");
        let else_block = self.llvm.context.append_basic_block(self.function_context.function, "else_block");

        self.llvm.builder.build_conditional_branch(
            to_i1(condition.into_int_value(), &self.llvm.builder),
            then_block,
            else_block,
        );

        self.llvm.builder.position_at_end(then_block);
        self.register_debug_location(statement);
        self.generate_return_statement()?;
        self.llvm.builder.position_at_end(else_block);

        Ok(())
    }

    fn get_llvm_deps(&self) -> (&Builder, FunctionValue, &Context) {
        (&self.llvm.builder, self.function_context.function, self.llvm.context)
    }
}

/// when generating an assignment to a direct-access (e.g. a.b.c.%W3.%X2 := 2;)
/// we want to deconstruct the sequence into the base-statement  (a.b.c) and the sequence
/// of direct-access commands (vec![%W3, %X2])
fn collect_base_and_direct_access_for_assignment(
    left_statement: &AstNode,
) -> Option<(&AstNode, Vec<&AstNode>)> {
    let mut current = Some(left_statement);
    let mut access_sequence = Vec::new();
    while let Some(AstStatement::ReferenceExpr(ReferenceExpr { access: ReferenceAccess::Member(m), base })) =
        current.map(|it| it.get_stmt())
    {
        if matches!(m.get_stmt(), AstStatement::DirectAccess { .. }) {
            access_sequence.insert(0, m.as_ref());
            current = base.as_deref();
        } else {
            break;
        }
    }
    current.zip(Some(access_sequence))
}
