// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{
    expression_generator::{to_i1, ExpressionCodeGenerator},
    llvm::Llvm,
    pou_generator::PouGenerator,
};
use crate::{
    ast::{flatten_expression_list, AstStatement, ConditionalBlock, Operator, SourceRange},
    codegen::llvm_typesystem,
    codegen::LlvmTypedIndex,
    diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR},
    index::{ImplementationIndexEntry, Index},
    resolver::AstAnnotations,
    typesystem::{
        self, DataTypeInformation, RANGE_CHECK_LS_FN, RANGE_CHECK_LU_FN, RANGE_CHECK_S_FN,
        RANGE_CHECK_U_FN,
    },
};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    values::{BasicValueEnum, FunctionValue},
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
    annotations: &'b AstAnnotations,
    pou_generator: &'b PouGenerator<'a, 'b>,
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
        annotations: &'b AstAnnotations,
        pou_generator: &'b PouGenerator<'a, 'b>,
        llvm_index: &'b LlvmTypedIndex<'a>,
        linking_context: &'b FunctionContext<'a>,
    ) -> StatementCodeGenerator<'a, 'b> {
        StatementCodeGenerator {
            llvm,
            index,
            annotations,
            pou_generator,
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
            self.function_context,
        )
    }

    /// generates a list of statements
    pub fn generate_body(&self, statements: &[AstStatement]) -> Result<(), Diagnostic> {
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
        let buffer_block = context.insert_basic_block_after(
            builder.get_insert_block().expect(INTERNAL_LLVM_ERROR),
            "buffer_block",
        );
        builder.position_at_end(buffer_block);
    }

    /// genertes a single statement
    ///
    /// - `statement` the statement to be generated
    pub fn generate_statement(&self, statement: &AstStatement) -> Result<(), Diagnostic> {
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
            AstStatement::ReturnStatement { .. } => {
                self.pou_generator
                    .generate_return_statement(self.function_context, self.llvm_index)?;
                self.generate_buffer_block();
            }
            AstStatement::ExitStatement { location, .. } => {
                if let Some(exit_block) = &self.current_loop_exit {
                    self.llvm.builder.build_unconditional_branch(*exit_block);
                    self.generate_buffer_block();
                } else {
                    return Err(Diagnostic::codegen_error(
                        "Cannot break out of loop when not inside a loop",
                        location.clone(),
                    ));
                }
            }
            AstStatement::ContinueStatement { location, .. } => {
                if let Some(cont_block) = &self.current_loop_continue {
                    self.llvm.builder.build_unconditional_branch(*cont_block);
                    self.generate_buffer_block();
                } else {
                    return Err(Diagnostic::codegen_error(
                        "Cannot continue loop when not inside a loop",
                        location.clone(),
                    ));
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
    ) -> Result<(), Diagnostic> {
        //TODO: Looks hacky, the strings will be similar so we should look into making the assignment a bit nicer.
        if left_statement.has_direct_access() {
            return self.generate_direct_access_assignment(left_statement, right_statement);
        }
        let exp_gen = self.create_expr_generator();
        let left = exp_gen.generate_element_pointer(left_statement)?;
        let left_type = exp_gen.get_type_hint_info_for(left_statement)?;
        // if the lhs-type is a subrange type we may need to generate a check-call
        // e.g. x := y,  ==> x := CheckSignedInt(y);
        let range_checked_right_side =
            if let DataTypeInformation::SubRange { sub_range, .. } = left_type {
                // there is a sub-range defined, so we need to wrap the right side into the check function if it exists

                //TODO move this to generate_store
                self.find_range_check_implementation_for(left_type)
                    .map(|implementation| {
                        create_call_to_check_function_ast(
                            left_statement,
                            implementation.get_call_name().to_string(),
                            right_statement.clone(),
                            sub_range.clone(),
                            &left_statement.get_location(),
                        )
                    })
            } else {
                None
            };

        let right_statement = range_checked_right_side.as_ref().unwrap_or(right_statement);

        exp_gen.generate_store(left_type, right_statement, left)?;
        Ok(())
    }

    fn generate_direct_access_assignment(
        &self,
        left_statement: &AstStatement,
        right_statement: &AstStatement,
    ) -> Result<(), Diagnostic> {
        //TODO : Validation
        let exp_gen = self.create_expr_generator();
        if let AstStatement::QualifiedReference { elements, .. } = left_statement {
            //Target
            let target: Vec<AstStatement> = elements
                .iter()
                .take_while(|it| !matches!(*it, &AstStatement::DirectAccess { .. }))
                .cloned()
                .collect();
            let id = target.last().unwrap().get_id();
            let target = AstStatement::QualifiedReference {
                elements: target.to_vec(),
                id,
            };

            //Access
            let direct_access: Vec<&AstStatement> = elements
                .iter()
                .skip_while(|it| !matches!(*it, &AstStatement::DirectAccess { .. }))
                .collect();
            let left_type = exp_gen.get_type_hint_for(&target)?;
            let right_type = exp_gen.get_type_hint_for(right_statement)?;

            //special case if we deal with a single bit, then we need to switch to a faked u1 type
            let right_type = if let DataTypeInformation::Integer {
                semantic_size: Some(typesystem::U1_SIZE),
                ..
            } = *right_type.get_type_information()
            {
                self.index.get_type_or_panic(typesystem::U1_TYPE)
            } else {
                right_type
            };

            //Left pointer
            let left = exp_gen.generate_element_pointer(&target)?;
            let left_value = self.llvm.load_pointer(&left, "").into_int_value();
            //Build index
            if let Some((element, direct_access)) = direct_access.split_first() {
                let mut rhs = if let AstStatement::DirectAccess { access, index, .. } = element {
                    exp_gen.generate_direct_access_index(
                        access,
                        index,
                        right_type.get_type_information(),
                        left_type,
                    )
                } else {
                    Err(Diagnostic::syntax_error(
                        &format!("{:?} not a direct access", element),
                        element.get_location(),
                    ))
                }?;
                for element in direct_access {
                    let rhs_next = if let AstStatement::DirectAccess { access, index, .. } = element
                    {
                        exp_gen.generate_direct_access_index(
                            access,
                            index,
                            right_type.get_type_information(),
                            left_type,
                        )
                    } else {
                        Err(Diagnostic::syntax_error(
                            &format!("{:?} not a direct access", element),
                            element.get_location(),
                        ))
                    }?;
                    rhs = self.llvm.builder.build_int_add(rhs, rhs_next, "");
                }
                //Build mask for the index
                //Get the target bit type as all ones
                let rhs_type = self
                    .llvm_index
                    .get_associated_type(right_type.get_name())?
                    .into_int_type();
                let ones = rhs_type.const_all_ones();
                //Extend the mask to the target type
                let extended_mask =
                    self.llvm
                        .builder
                        .build_int_z_extend(ones, left_value.get_type(), "ext");
                //Position the ones in their correct locations
                let shifted_mask = self
                    .llvm
                    .builder
                    .build_left_shift(extended_mask, rhs, "shift");
                //Invert the mask
                let mask = self.llvm.builder.build_not(shifted_mask, "invert");
                //And the result with the mask to erase the set bits at the target location
                let and_value = self.llvm.builder.build_and(left_value, mask, "erase");

                //Generate an expression for the right size
                let right = exp_gen.generate_expression(right_statement)?;
                //Cast the right side to the left side type
                let lhs = llvm_typesystem::cast_if_needed(
                    self.llvm,
                    self.index,
                    self.llvm_index,
                    left_type,
                    right,
                    right_type,
                    right_statement,
                )
                .map(BasicValueEnum::into_int_value)?;
                //Shift left by the direct access
                let value = self.llvm.builder.build_left_shift(lhs, rhs, "value");

                //OR the result and store it in the left side
                let or_value = self.llvm.builder.build_or(and_value, value, "or");
                self.llvm.builder.build_store(left, or_value);
            } else {
                unreachable!();
            }
        } else {
            unreachable!()
        }

        Ok(())
    }

    /// returns the implementation of the sub-range-check-function for a variable of the given dataType
    fn find_range_check_implementation_for(
        &self,
        range_type: &DataTypeInformation,
    ) -> Option<&ImplementationIndexEntry> {
        match range_type {
            DataTypeInformation::Integer { signed, size, .. } if *signed && *size <= 32 => {
                self.index.find_pou_implementation(RANGE_CHECK_S_FN)
            }
            DataTypeInformation::Integer { signed, size, .. } if *signed && *size > 32 => {
                self.index.find_pou_implementation(RANGE_CHECK_LS_FN)
            }
            DataTypeInformation::Integer { signed, size, .. } if !*signed && *size <= 32 => {
                self.index.find_pou_implementation(RANGE_CHECK_U_FN)
            }
            DataTypeInformation::Integer { signed, size, .. } if !*signed && *size > 32 => {
                self.index.find_pou_implementation(RANGE_CHECK_LU_FN)
            }
            DataTypeInformation::Alias { name, .. }
            | DataTypeInformation::SubRange {
                referenced_type: name,
                ..
            } => {
                //traverse to the primitive type
                self.index
                    .find_effective_type_info(name)
                    .and_then(|info| self.find_range_check_implementation_for(info))
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

        builder.build_conditional_branch(
            to_i1(or_eval.into_int_value(), builder),
            for_body,
            continue_block,
        );

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
                    SourceRange::undefined(),
                )
            },
            |step| expression_generator.generate_expression(step),
        )?;

        let next = builder.build_int_add(
            counter_statement.into_int_value(),
            step_by_value.into_int_value(),
            "tmpVar",
        );

        let ptr = expression_generator.generate_element_pointer(counter)?;
        builder.build_store(ptr, next);

        //Loop back
        builder.build_unconditional_branch(condition_check);

        //Continue
        builder.position_at_end(continue_block);

        Ok(())
    }

    fn generate_compare_expression(
        &'a self,
        counter: &AstStatement,
        end: &AstStatement,
        start: &AstStatement,
        exp_gen: &'a ExpressionCodeGenerator,
    ) -> Result<BasicValueEnum<'a>, Diagnostic> {
        let counter_end_ge = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::GreaterOrEqual,
            left: Box::new(counter.to_owned()),
            right: Box::new(end.to_owned()),
        };
        let counter_start_ge = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::GreaterOrEqual,
            left: Box::new(counter.to_owned()),
            right: Box::new(start.to_owned()),
        };
        let counter_end_le = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::LessOrEqual,
            left: Box::new(counter.to_owned()),
            right: Box::new(end.to_owned()),
        };
        let counter_start_le = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::LessOrEqual,
            left: Box::new(counter.to_owned()),
            right: Box::new(start.to_owned()),
        };
        let and_1 = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::And,
            left: Box::new(counter_end_le),
            right: Box::new(counter_start_ge),
        };
        let and_2 = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::And,
            left: Box::new(counter_end_ge),
            right: Box::new(counter_start_le),
        };
        let or = AstStatement::BinaryExpression {
            id: self.annotations.get_bool_id(),
            operator: Operator::Or,
            left: Box::new(and_1),
            right: Box::new(and_2),
        };
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
        selector: &AstStatement,
        conditional_blocks: &[ConditionalBlock],
        else_body: &[AstStatement],
    ) -> Result<Option<BasicValueEnum<'a>>, Diagnostic> {
        let (builder, current_function, context) = self.get_llvm_deps();
        //Continue
        let continue_block = context.append_basic_block(current_function, "continue");

        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let exp_gen = self.create_expr_generator();
        let selector_statement = exp_gen.generate_expression(&*selector)?;

        let mut cases = Vec::new();
        let else_block = context.append_basic_block(current_function, "else");
        let mut current_else_block = else_block;

        for conditional_block in conditional_blocks {
            //craete a block for the case's body
            let case_block = context.prepend_basic_block(else_block, "case");

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
        continue_block
            .move_after(current_else_block)
            .expect(INTERNAL_LLVM_ERROR);

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
    ) -> Result<BasicBlock, Diagnostic> {
        let (builder, _, context) = self.get_llvm_deps();

        let range_then = context.insert_basic_block_after(
            builder.get_insert_block().expect(INTERNAL_LLVM_ERROR),
            "range_then",
        );
        let range_else = context.insert_basic_block_after(range_then, "range_else");
        let exp_gen = self.create_expr_generator();
        let lower_bound = {
            let start_val = exp_gen.generate_expression(start)?;
            let selector_val = exp_gen.generate_expression(selector)?;
            exp_gen.create_llvm_int_binary_expression(
                &Operator::GreaterOrEqual,
                selector_val,
                start_val,
            )
        };

        //jmp to continue if the value is smaller than start
        builder.build_conditional_branch(
            to_i1(lower_bound.into_int_value(), builder),
            range_then,
            range_else,
        );
        builder.position_at_end(range_then);
        let upper_bound = {
            let end_val = exp_gen.generate_expression(end)?;
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
    fn generate_while_statement(
        &self,
        condition: &AstStatement,
        body: &[AstStatement],
    ) -> Result<Option<BasicValueEnum<'a>>, Diagnostic> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let (condition_block, _) = self.generate_base_while_statement(condition, body)?;

        let continue_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);

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
    ) -> Result<Option<BasicValueEnum<'a>>, Diagnostic> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let (_, while_block) = self.generate_base_while_statement(condition, body)?;

        let continue_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);

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
    ) -> Result<(BasicBlock, BasicBlock), Diagnostic> {
        let (builder, current_function, context) = self.get_llvm_deps();
        let condition_check = context.append_basic_block(current_function, "condition_check");
        let while_body = context.append_basic_block(current_function, "while_body");
        let continue_block = context.append_basic_block(current_function, "continue");

        //Check loop condition
        builder.position_at_end(condition_check);
        let condition_value = self
            .create_expr_generator()
            .generate_expression(condition)?;
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
        else_body: &[AstStatement],
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

            let condition = self
                .create_expr_generator()
                .generate_expression(&block.condition)?;
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

    fn get_llvm_deps(&self) -> (&Builder, FunctionValue, &Context) {
        (
            &self.llvm.builder,
            self.function_context.function,
            self.llvm.context,
        )
    }
}

fn create_call_to_check_function_ast(
    target: &AstStatement,
    check_function_name: String,
    parameter: AstStatement,
    sub_range: Range<AstStatement>,
    location: &SourceRange,
) -> AstStatement {
    let range_type_id = sub_range.start.get_id();
    let target_id = target.get_id();
    crate::ast::create_call_to(
        check_function_name,
        vec![parameter, sub_range.start, sub_range.end],
        target_id,
        range_type_id,
        location,
    )
}
