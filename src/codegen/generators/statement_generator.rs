// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::{
    expression_generator::{to_i1, ExpressionCodeGenerator, ExpressionValue},
    llvm::Llvm,
};
use crate::{
    codegen::{
        debug::{Debug, DebugBuilderEnum},
        llvm_typesystem::cast_if_needed,
        CodegenError, LlvmTypedIndex,
    },
    index::{ImplementationIndexEntry, Index},
    resolver::{AnnotationMap, AstAnnotations, StatementAnnotation},
    typesystem::{get_bigger_type, DataTypeInformation, DINT_TYPE},
};
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    values::{FunctionValue, PointerValue},
};
use plc_ast::{
    ast::{
        flatten_expression_list, Allocation, AstNode, AstStatement, JumpStatement, LabelStatement, Operator,
        ReferenceAccess, ReferenceExpr,
    },
    control_statements::{
        AstControlStatement, CaseStatement, ForLoopStatement, IfStatement, LoopStatement, ReturnStatement,
    },
};
use plc_diagnostics::diagnostics::{Diagnostic, INTERNAL_LLVM_ERROR};
use plc_source::source_location::SourceLocation;
use rustc_hash::FxHashMap;

/// the full context when generating statements inside a POU
#[derive(Debug)]
pub struct FunctionContext<'ink, 'b> {
    /// the current pou's name. This means that a variable x may refer to "`linking_context`.x"
    pub linking_context: &'b ImplementationIndexEntry,
    /// the llvm function to generate statements into
    pub function: FunctionValue<'ink>,
    /// The blocks/labels this function can use
    pub blocks: FxHashMap<String, BasicBlock<'ink>>,
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
    fn create_expr_generator(
        &'a self,
        llvm_index: &'b LlvmTypedIndex<'a>,
    ) -> ExpressionCodeGenerator<'a, 'b> {
        ExpressionCodeGenerator::new(
            self.llvm,
            self.index,
            self.annotations,
            llvm_index,
            self.function_context,
            self.debug,
        )
    }

    /// generates a list of statements
    pub fn generate_body(&self, statements: &[AstNode]) -> Result<(), CodegenError> {
        let mut child_index = LlvmTypedIndex::create_child(self.llvm_index);
        for s in statements {
            child_index = self.generate_statement(child_index, s)?;
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
    pub fn generate_statement(
        &self,
        mut llvm_index: LlvmTypedIndex<'b>,
        statement: &AstNode,
    ) -> Result<LlvmTypedIndex<'b>, CodegenError> {
        match statement.get_stmt() {
            AstStatement::EmptyStatement(..) => {
                //nothing to generate
            }
            AstStatement::Assignment(data, ..) => {
                self.generate_assignment_statement(&llvm_index, &data.left, &data.right)?;
            }
            AstStatement::RefAssignment(data, ..) => {
                self.generate_ref_assignment(&llvm_index, &data.left, &data.right)?;
            }
            AstStatement::ControlStatement(ctl_statement, ..) => {
                self.generate_control_statement(&llvm_index, ctl_statement)?
            }
            AstStatement::ReturnStatement(ReturnStatement { condition }) => match condition {
                Some(condition) => {
                    self.generate_conditional_return(&llvm_index, statement, condition)?;
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
                    self.llvm.builder.build_unconditional_branch(*block)?;
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
                        statement,
                    )
                    .into());
                };
                //Set current location as else block
                let current_block = self.llvm.builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
                let else_block = self.llvm.context.insert_basic_block_after(current_block, "else_block");

                self.register_debug_location(condition);
                let expression_generator = self.create_expr_generator(&llvm_index);
                let condition = expression_generator.generate_expression(condition)?;

                self.register_debug_location(statement);
                self.llvm.builder.build_conditional_branch(
                    condition.into_int_value(),
                    *then_block,
                    else_block,
                )?;
                // Make sure further code is at the else block
                self.llvm.builder.position_at_end(else_block);
            }
            AstStatement::ExitStatement(_) => {
                if let Some(exit_block) = &self.current_loop_exit {
                    self.register_debug_location(statement);
                    self.llvm.builder.build_unconditional_branch(*exit_block)?;
                    self.generate_buffer_block();
                } else {
                    return Err(Diagnostic::codegen_error(
                        "Cannot break out of loop when not inside a loop",
                        statement,
                    )
                    .into());
                }
            }
            AstStatement::ContinueStatement(_) => {
                if let Some(cont_block) = &self.current_loop_continue {
                    self.llvm.builder.build_unconditional_branch(*cont_block)?;
                    self.generate_buffer_block();
                } else {
                    return Err(Diagnostic::codegen_error(
                        "Cannot continue loop when not inside a loop",
                        statement,
                    )
                    .into());
                }
            }
            AstStatement::ExpressionList(statements) => {
                let mut llvm_index = LlvmTypedIndex::create_child(&llvm_index);
                for stmt in statements {
                    llvm_index = self.generate_statement(llvm_index, stmt)?;
                }
            }
            AstStatement::AllocationStatement(Allocation { name, reference_type }) => {
                let ty =
                    llvm_index.find_associated_type(reference_type).expect("Type must exist at this point");
                let value = self.llvm.builder.build_alloca(ty, name)?;
                self.llvm.generate_variable_initializer(
                    &llvm_index,
                    self.index,
                    (name, reference_type, &statement.location),
                    value,
                    None,
                    &self.create_expr_generator(&llvm_index),
                )?;
                llvm_index.associate_loaded_local_variable(
                    self.function_context.linking_context.get_type_name(),
                    name,
                    value,
                )?;
            }
            _ => {
                self.create_expr_generator(&llvm_index).generate_expression(statement)?;
            }
        }
        Ok(llvm_index)
    }

    /// genertes a single statement
    ///
    /// - `statement` the control statement to be generated
    pub fn generate_control_statement(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        statement: &AstControlStatement,
    ) -> Result<(), CodegenError> {
        match statement {
            AstControlStatement::If(ifstmt) => self.generate_if_statement(llvm_index, ifstmt),
            AstControlStatement::ForLoop(for_stmt) => self.generate_for_statement(llvm_index, for_stmt),
            AstControlStatement::WhileLoop(stmt) | AstControlStatement::RepeatLoop(stmt) => {
                self.generate_loop_statement(llvm_index, stmt)
            }
            AstControlStatement::Case(stmt) => self.generate_case_statement(llvm_index, stmt),
        }
    }

    /// Generates IR for a `REF=` assignment, which is syntactic sugar for an assignment where the
    /// right-hand side is wrapped in a `REF(...)` call. Specifically `foo REF= bar` and
    /// `foo := REF(bar)` are the same.
    ///
    /// Note: Although somewhat similar to the [`generate_assignment_statement`] function, we can't
    /// apply the code here because the left side of a `REF=` assignment is flagged as auto-deref.
    /// For `REF=` assignments we don't want (and can't) deref without generating incorrect IR.
    pub fn generate_ref_assignment(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<(), CodegenError> {
        let exp = self.create_expr_generator(llvm_index);
        let ref_builtin = self.index.get_builtin_function("REF").expect("REF must exist");

        let AstStatement::ReferenceExpr(data) = &left.stmt else {
            unreachable!("should be covered by a validation")
        };

        let left_ptr_val = {
            let expr = exp.generate_reference_expression(&data.access, data.base.as_deref(), left)?;
            expr.get_basic_value_enum().into_pointer_value()
        };
        let right_expr_val = if right.is_zero() {
            exp.generate_literal(right)?
        } else {
            ref_builtin.codegen(&exp, &[right], right.get_location())?
        };

        self.llvm.builder.build_store(left_ptr_val, right_expr_val.get_basic_value_enum())?;
        Ok(())
    }

    /// generates an assignment statement _left_ := _right_
    ///
    /// `left_statement` the left side of the assignment
    /// `right_statement` the right side of the assignment
    pub fn generate_assignment_statement(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        left_statement: &AstNode,
        right_statement: &AstNode,
    ) -> Result<(), CodegenError> {
        //Register any debug info for the store
        self.register_debug_location(left_statement);
        //TODO: Looks hacky, the strings will be similar so we should look into making the assignment a bit nicer.
        if left_statement.has_direct_access() {
            return self.generate_assignment_statement_direct_access(
                llvm_index,
                left_statement,
                right_statement,
            );
        }

        if self.annotations.get(left_statement).is_some_and(|it| {
            // TODO(mhasel): ideally the resolver decides which assignment statement to call when lowering the init functions,
            // but that requires refactoring of how `aliases` and `reference to` LHS/RHS nodes are annotated. this is a workaround.
            self.function_context.linking_context.is_init() && (it.is_alias() || it.is_reference_to())
        }) {
            return self.generate_ref_assignment(llvm_index, left_statement, right_statement);
        };

        let exp_gen = self.create_expr_generator(llvm_index);
        let left: PointerValue = exp_gen.generate_expression_value(left_statement).and_then(|it| {
            it.get_basic_value_enum()
                .try_into()
                .map_err(|err| CodegenError::new(format!("{err:?}").as_str(), left_statement))
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
        self.debug.set_debug_location(self.llvm, self.function_context, line, column);
    }

    fn generate_assignment_statement_direct_access(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        left_statement: &AstNode,
        right_statement: &AstNode,
    ) -> Result<(), CodegenError> {
        let exp_gen = self.create_expr_generator(llvm_index);

        // Left pointer
        let Some((base, _)) = collect_base_and_direct_access_for_assignment(left_statement) else {
            unreachable!("Invalid direct-access expression: {left_statement:#?}")
        };
        let left_expr_value = exp_gen.generate_expression_value(base)?;
        let left_value = left_expr_value.as_r_value(self.llvm, None)?.into_int_value();
        let left_pointer = left_expr_value.get_basic_value_enum().into_pointer_value();

        // Generate an expression for the right size
        let right_type = exp_gen.get_type_hint_for(right_statement)?;
        let right_expr = exp_gen.generate_expression(right_statement)?;

        exp_gen.generate_assignment_with_direct_access(
            left_statement,
            left_value,
            left_pointer,
            right_type,
            right_expr,
        )
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
        llvm_index: &'a LlvmTypedIndex<'b>,
        stmt: &ForLoopStatement,
    ) -> Result<(), CodegenError> {
        let (builder, current_function, context) = self.get_llvm_deps();
        let exp_gen = self.create_expr_generator(llvm_index);

        let end_ty = self.annotations.get_type_or_void(&stmt.end, self.index);
        let counter_ty = self.annotations.get_type_or_void(&stmt.counter, self.index);
        let cast_target_ty = get_bigger_type(self.index.get_type_or_panic(DINT_TYPE), counter_ty, self.index);
        let cast_target_llty = llvm_index.find_associated_type(cast_target_ty.get_name()).unwrap();

        let step_ty = stmt.by_step.as_ref().map(|it| {
            self.register_debug_location(it);
            self.annotations.get_type_or_void(it, self.index)
        });

        let eval_step = || {
            step_ty.map_or_else(
                || self.llvm.create_const_numeric(&cast_target_llty, "1", SourceLocation::internal()),
                |step_ty| {
                    let step = exp_gen.generate_expression(stmt.by_step.as_ref().unwrap())?;
                    cast_if_needed!(exp_gen, cast_target_ty, step_ty, step, None)
                },
            )
        };

        let predicate_incrementing = context.append_basic_block(current_function, "predicate_sle");
        let predicate_decrementing = context.append_basic_block(current_function, "predicate_sge");
        let loop_body = context.append_basic_block(current_function, "loop");
        let increment = context.append_basic_block(current_function, "increment");
        let afterloop = context.append_basic_block(current_function, "continue");

        self.generate_assignment_statement(llvm_index, &stmt.counter, &stmt.start)?;
        let counter = exp_gen.generate_lvalue(&stmt.counter)?;
        let counter_pointee = {
            let datatype = self.annotations.get_type(&stmt.counter, self.index).unwrap();
            self.llvm_index.get_associated_type(&datatype.name).unwrap()
        };

        // generate loop predicate selector. since `STEP` can be a reference, this needs to be a runtime eval
        // XXX(mhasel): IR could possibly be improved by generating phi instructions.
        //              Candidate for frontend optimization for builds without optimization when `STEP`
        //              is a compile-time constant
        let is_incrementing = builder.build_int_compare(
            inkwell::IntPredicate::SGT,
            eval_step()?.into_int_value(),
            self.llvm
                .create_const_numeric(&cast_target_llty, "0", SourceLocation::internal())?
                .into_int_value(),
            "is_incrementing",
        )?;
        builder.build_conditional_branch(is_incrementing, predicate_incrementing, predicate_decrementing)?;
        // generate predicates for incrementing and decrementing counters
        let generate_predicate = |predicate| {
            builder.position_at_end(match predicate {
                inkwell::IntPredicate::SLE => predicate_incrementing,
                inkwell::IntPredicate::SGE => predicate_decrementing,
                _ => unreachable!(),
            });

            let end = exp_gen.generate_expression_value(&stmt.end).unwrap();
            let end_value = match end {
                ExpressionValue::LValue(value, pointee) => builder.build_load(pointee, value, "")?,
                ExpressionValue::RValue(val) => val,
            };

            let counter_value = builder.build_load(counter_pointee, counter, "")?;
            let cmp = builder.build_int_compare(
                predicate,
                cast_if_needed!(exp_gen, cast_target_ty, counter_ty, counter_value, None)?.into_int_value(),
                cast_if_needed!(exp_gen, cast_target_ty, end_ty, end_value, None)?.into_int_value(),
                "condition",
            )?;
            builder.build_conditional_branch(cmp, loop_body, afterloop)?;
            Ok::<(), CodegenError>(())
        };
        generate_predicate(inkwell::IntPredicate::SLE)?;
        generate_predicate(inkwell::IntPredicate::SGE)?;

        // generate loop body
        builder.position_at_end(loop_body);
        let body_builder = StatementCodeGenerator {
            current_loop_continue: Some(increment),
            current_loop_exit: Some(afterloop),
            load_prefix: self.load_prefix.clone(),
            load_suffix: self.load_suffix.clone(),
            ..*self
        };
        body_builder.generate_body(&stmt.body)?;
        // Place debug location to end of loop
        self.debug.set_debug_location(
            self.llvm,
            self.function_context,
            stmt.end_location.get_line_plus_one(),
            stmt.end_location.get_column(),
        );

        // increment counter
        builder.build_unconditional_branch(increment)?;
        builder.position_at_end(increment);
        let counter_value = builder.build_load(counter_pointee, counter, "")?;
        let inc = inkwell::values::BasicValue::as_basic_value_enum(&builder.build_int_add(
            eval_step()?.into_int_value(),
            cast_if_needed!(exp_gen, cast_target_ty, counter_ty, counter_value, None)?.into_int_value(),
            "next",
        )?);
        builder.build_store(
            counter,
            cast_if_needed!(exp_gen, counter_ty, cast_target_ty, inc, None)?.into_int_value(),
        )?;

        // check condition
        builder.build_conditional_branch(is_incrementing, predicate_incrementing, predicate_decrementing)?;
        // continue
        builder.position_at_end(afterloop);
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
        llvm_index: &'a LlvmTypedIndex<'b>,
        stmt: &CaseStatement,
    ) -> Result<(), CodegenError> {
        let (builder, current_function, context) = self.get_llvm_deps();
        //Continue
        let continue_block = context.append_basic_block(current_function, "continue");

        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let exp_gen = self.create_expr_generator(llvm_index);
        let selector_statement = exp_gen.generate_expression(&stmt.selector)?;

        let mut cases = Vec::new();
        let else_block = context.append_basic_block(current_function, "else");
        let mut current_else_block = else_block;

        for conditional_block in &stmt.case_blocks {
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
                        llvm_index,
                        &stmt.selector,
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
            //Add debug location to the end of the case
            self.debug.set_debug_location(
                self.llvm,
                self.function_context,
                stmt.end_location.get_line_plus_one(),
                stmt.end_location.get_column(),
            );
            // skip all other case-bodies
            builder.build_unconditional_branch(continue_block)?;
        }
        // current-else is the last else-block generated by the range-expressions
        builder.position_at_end(current_else_block);
        self.generate_body(&stmt.else_block)?;
        //Add debug location to the end of the case
        self.debug.set_debug_location(
            self.llvm,
            self.function_context,
            stmt.end_location.get_line_plus_one(),
            stmt.end_location.get_column(),
        );
        builder.build_unconditional_branch(continue_block)?;
        continue_block.move_after(current_else_block).expect(INTERNAL_LLVM_ERROR);

        // now that we collected all cases, go back to the initial block and generate the switch-statement
        builder.position_at_end(basic_block);

        self.register_debug_location(&stmt.selector);
        builder.build_switch(selector_statement.into_int_value(), else_block, &cases)?;

        builder.position_at_end(continue_block);
        Ok(())
    }

    /// returns the new block to use as else
    ///
    ///
    fn generate_case_range_condition(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        selector: &AstNode,
        start: &AstNode,
        end: &AstNode,
        match_block: BasicBlock,
    ) -> Result<BasicBlock<'_>, CodegenError> {
        let (builder, _, context) = self.get_llvm_deps();

        let range_then = context
            .insert_basic_block_after(builder.get_insert_block().expect(INTERNAL_LLVM_ERROR), "range_then");
        let range_else = context.insert_basic_block_after(range_then, "range_else");
        let exp_gen = self.create_expr_generator(llvm_index);
        let lower_bound = {
            self.register_debug_location(start);
            let start_val = exp_gen.generate_expression(start)?;
            self.register_debug_location(selector);
            let selector_val = exp_gen.generate_expression(selector)?;
            exp_gen.create_llvm_int_binary_expression(&Operator::GreaterOrEqual, selector_val, start_val)?
        };

        //jmp to continue if the value is smaller than start
        builder.build_conditional_branch(
            to_i1(lower_bound.into_int_value(), builder)?,
            range_then,
            range_else,
        )?;
        builder.position_at_end(range_then);
        let upper_bound = {
            self.register_debug_location(end);
            let end_val = exp_gen.generate_expression(end)?;
            self.register_debug_location(selector);
            let selector_val = exp_gen.generate_expression(selector)?;
            exp_gen.create_llvm_int_binary_expression(&Operator::LessOrEqual, selector_val, end_val)?
        };
        builder.build_conditional_branch(
            to_i1(upper_bound.into_int_value(), builder)?,
            match_block,
            range_else,
        )?;
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
    fn generate_loop_statement(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        stmt: &LoopStatement,
    ) -> Result<(), CodegenError> {
        let builder = &self.llvm.builder;
        let basic_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);
        let (condition_block, _) =
            self.generate_base_while_statement(llvm_index, &stmt.condition, &stmt.body, &stmt.end_location)?;

        let continue_block = builder.get_insert_block().expect(INTERNAL_LLVM_ERROR);

        builder.position_at_end(basic_block);
        builder.build_unconditional_branch(condition_block)?;

        builder.position_at_end(continue_block);
        Ok(())
    }

    /// utility method for while and repeat loops
    fn generate_base_while_statement(
        &self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        condition: &AstNode,
        body: &[AstNode],
        end_location: &SourceLocation,
    ) -> Result<(BasicBlock<'_>, BasicBlock<'_>), CodegenError> {
        let (builder, current_function, context) = self.get_llvm_deps();
        let condition_check = context.append_basic_block(current_function, "condition_check");
        let while_body = context.append_basic_block(current_function, "while_body");
        let continue_block = context.append_basic_block(current_function, "continue");

        //Check loop condition
        builder.position_at_end(condition_check);
        self.register_debug_location(condition);
        let condition_value = self.create_expr_generator(llvm_index).generate_expression(condition)?;
        builder.build_conditional_branch(
            to_i1(condition_value.into_int_value(), builder)?,
            while_body,
            continue_block,
        )?;

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
        //Set the debug location to the end of the loop
        self.debug.set_debug_location(
            self.llvm,
            self.function_context,
            end_location.get_line_plus_one(),
            end_location.get_column(),
        );
        //Loop back
        builder.build_unconditional_branch(condition_check)?;

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
        llvm_index: &'a LlvmTypedIndex<'b>,
        stmt: &IfStatement,
    ) -> Result<(), CodegenError> {
        let (builder, current_function, context) = self.get_llvm_deps();
        let mut blocks = vec![builder.get_insert_block().expect(INTERNAL_LLVM_ERROR)];
        for _ in 1..stmt.blocks.len() {
            blocks.push(context.append_basic_block(current_function, "branch"));
        }

        let else_block = if !stmt.else_block.is_empty() {
            let result = context.append_basic_block(current_function, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };
        //Continue
        let continue_block = context.append_basic_block(current_function, "continue");
        blocks.push(continue_block);

        for (i, block) in stmt.blocks.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            builder.position_at_end(then_block);

            self.register_debug_location(&block.condition);
            let condition = self.create_expr_generator(llvm_index).generate_expression(&block.condition)?;
            let conditional_block = context.prepend_basic_block(else_block, "condition_body");

            //Generate if statement condition
            builder.build_conditional_branch(
                to_i1(condition.into_int_value(), builder)?,
                conditional_block,
                else_block,
            )?;

            //Generate if statement content

            builder.position_at_end(conditional_block);
            self.generate_body(&block.body)?;
            // Place debug location to end of if
            self.debug.set_debug_location(
                self.llvm,
                self.function_context,
                stmt.end_location.get_line_plus_one(),
                stmt.end_location.get_column(),
            );
            builder.build_unconditional_branch(continue_block)?;
        }
        //Else

        if let Some(else_block) = else_block {
            builder.position_at_end(else_block);
            self.generate_body(&stmt.else_block)?;
            // Place debug location to end of if
            self.debug.set_debug_location(
                self.llvm,
                self.function_context,
                stmt.end_location.get_line_plus_one(),
                stmt.end_location.get_column(),
            );
            builder.build_unconditional_branch(continue_block)?;
        }
        //Continue
        builder.position_at_end(continue_block);
        Ok(())
    }

    /// generates the function's return statement only if the given pou_type is a `PouType::Function`
    ///
    /// a function returns the value of the local variable that has the function's name
    pub fn generate_return_statement(&self) -> Result<(), CodegenError> {
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
                self.llvm.builder.build_return(None)?;
            } else {
                // generate return statement
                let call_name = &self.function_context.linking_context.get_call_name_for_ir();
                let var_name = format!("{call_name}_ret"); // TODO: Naming convention (see plc_util/src/convention.rs)
                let ret_name = ret_v.get_qualified_name();
                let value_ptr =
                    self.llvm_index.find_loaded_associated_variable_value(ret_name).ok_or_else(|| {
                        CodegenError::new(
                            format!("Cannot generate return variable for {call_name:}"),
                            SourceLocation::undefined(),
                        )
                    })?;

                let pointee = self.llvm_index.get_associated_type(ret_v.get_type_name()).unwrap();
                let loaded_value = self.llvm.load_pointer(pointee, &value_ptr, var_name.as_str())?;
                self.llvm.builder.build_return(Some(&loaded_value))?;
            }
        } else {
            self.llvm.builder.build_return(None)?;
        }
        Ok(())
    }

    /// Generates LLVM IR for conditional returns, which return if a given condition evaluates to true and
    /// does nothing otherwise.
    pub fn generate_conditional_return(
        &'a self,
        llvm_index: &'a LlvmTypedIndex<'b>,
        statement: &AstNode,
        condition: &AstNode,
    ) -> Result<(), CodegenError> {
        let expression_generator = self.create_expr_generator(llvm_index);

        self.register_debug_location(condition);
        let condition = expression_generator.generate_expression(condition)?;

        let then_block = self.llvm.context.append_basic_block(self.function_context.function, "then_block");
        let else_block = self.llvm.context.append_basic_block(self.function_context.function, "else_block");

        self.llvm.builder.build_conditional_branch(
            to_i1(condition.into_int_value(), &self.llvm.builder)?,
            then_block,
            else_block,
        )?;

        self.llvm.builder.position_at_end(then_block);
        self.register_debug_location(statement);
        self.generate_return_statement()?;
        self.llvm.builder.position_at_end(else_block);

        Ok(())
    }

    fn get_llvm_deps(&self) -> (&Builder<'_>, FunctionValue<'_>, &Context) {
        (&self.llvm.builder, self.function_context.function, self.llvm.context)
    }
}

/// Deconstructs assignments such as `a.b.c.%W3.%X2 := 2` into a base statement and its direct-access sequences.
/// For the given example this function would return `(Node(a.b.c), vec![Node(%W3), Node(%X2)])`
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
