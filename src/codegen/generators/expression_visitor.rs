use std::{collections::HashMap, default};

use anyhow::{anyhow, bail, Error, Result};
use inkwell::{
    values::{BasicValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue},
    FloatPredicate, IntPredicate,
};
use itertools::Itertools;
use plc_ast::{
    ast::{
        flatten_expression_list, Assignment, AstId, AstNode, AstStatement, BinaryExpression, Operator,
        ReferenceAccess, ReferenceExpr,
    },
    literals::AstLiteral,
    visitor::AstVisitor,
};
use plc_diagnostics::diagnostics::{Diagnostic, ResultDiagnosticExt};
use plc_source::source_location::SourceLocation;

use crate::{
    codegen::{
        generators::{literals_generator::IntrinsicLiteralsGenerator, util::call_builder::{self, Argument,  CallArguments}},
        llvm_index::LlvmTypedIndex,
        llvm_typesystem::cast_if_needed,
    },
    index::{indexer::pou_indexer::PouIndexer, Index, PouIndexEntry, VariableIndexEntry},
    resolver::{AnnotationMap, AnnotationMapImpl, AstAnnotations, StatementAnnotation},
    typesystem::{DataType, DataTypeInformation, StringEncoding},
};

use super::{llvm::Llvm, statement_generator::FunctionContext, util::array_access_generator};

#[derive(Debug)]
pub enum GeneratedValue<'ink> {
    RValue((BasicValueEnum<'ink>, AstId)),
    // LValue(PointerValue<'ink>),
    LValue((PointerValue<'ink>, AstId)),
    NoValue,
}

impl<'ink> GeneratedValue<'ink> {
    pub fn as_pointer_value(&self) -> Result<PointerValue<'ink>> {
        match self {
            GeneratedValue::LValue((pv, ..)) => Ok(*pv),
            _ => bail!("Expected LValue but got {:#?}", self),
        }
    }

    // treat this value as a r-value, even if it is an l-value
    pub fn into_r_value(self) -> Result<GeneratedValue<'ink>> {
        match self {
            GeneratedValue::RValue((v, id)) => Ok(GeneratedValue::RValue((v, id))),
            GeneratedValue::LValue((pv, id)) => {
                // convert LValue to RValue
                Ok(GeneratedValue::RValue((pv.as_basic_value_enum(), id)))
            }
            GeneratedValue::NoValue => Ok(GeneratedValue::NoValue),
        }
    }

    pub fn is_r_value(&self) -> bool {
        matches!(self, GeneratedValue::RValue(_))
    }

    pub fn is_l_value(&self) -> bool {
        matches!(self, GeneratedValue::LValue(_))
    }
}

pub struct ExpressionVisitor<'ink, 'a> {
    pub llvm: &'a Llvm<'ink>,
    pub llvm_index: &'a LlvmTypedIndex<'ink>,
    pub annotations: &'a AstAnnotations,
    pub index: &'a Index,

    literals_generator: IntrinsicLiteralsGenerator<'ink, 'a>,
    result_stack: Vec<Result<GeneratedValue<'ink>, Diagnostic>>,

    function_context: Option<&'a FunctionContext<'ink, 'a>>,
}

impl<'ink, 'a> ExpressionVisitor<'ink, 'a> {
    pub fn new(
        llvm: &'ink Llvm<'ink>,
        llvm_index: &'ink LlvmTypedIndex<'ink>,
        annotations: &'a AstAnnotations,
        index: &'a Index,
        function_context: Option<&'a FunctionContext<'ink, 'a>>,
    ) -> Self {
        let literals_generator = IntrinsicLiteralsGenerator::new(llvm, llvm_index, index);
        Self {
            llvm,
            llvm_index,
            annotations,
            index,
            literals_generator,
            result_stack: Vec::new(),
            function_context,
        }
    }

    fn get_load_name(&self, id: usize) -> Option<&str> {
        if matches!(self.annotations.get_with_id(id), Some(StatementAnnotation::Value { .. })) {
            Some("tmpVar")
        } else {
            self.annotations.get_identifier_name_from_id(id)
        }
    }

    pub fn get_function_context(&self) -> Result<&'a FunctionContext<'ink, 'a>> {
        self.function_context
            .ok_or_else(|| anyhow!("Cannot generate expression outside of a function context."))
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_info_for(&self, statement: &AstNode) -> Result<&DataTypeInformation, Diagnostic> {
        self.get_type_hint_for(statement).map(DataType::get_type_information)
    }
}

impl<'ink> AstVisitor for ExpressionVisitor<'ink, '_> {
    fn visit_literal(&mut self, stmt: &AstLiteral, node: &plc_ast::ast::AstNode) {
        let do_visit_literal = || -> Result<GeneratedValue<'ink>> {
            let type_hint = self.get_type_hint_for(node)?.get_type_information();
            match stmt {
                // Integer, Bool, Date, Time, DateAndTime
                _ if stmt.is_int_numerical() => {
                    let value = stmt.try_int_value().expect("Parser should have checked this"); //parser should have checked this
                    if type_hint.is_float() {
                        self.literals_generator.generate_const_float(type_hint, value as f64, node)
                    } else {
                        self.literals_generator.generate_const_int(type_hint, value, node)
                    }
                }
                AstLiteral::Null => {
                    let Some(t) = self.llvm_index.find_associated_type(type_hint.get_name()) else {
                        bail!("Cannot find type for null literal: {type_hint:?}");
                    };
                    if !t.is_pointer_type() {
                        bail!("Cannot generate null literal for non-pointer type: {type_hint:?}");
                    }
                    Ok(GeneratedValue::RValue((
                        t.into_pointer_type().const_null().as_basic_value_enum(),
                        node.get_id(),
                    )))
                }
                AstLiteral::Real(v) => {
                    let value = v.parse::<f64>().expect("Failed to parse float"); //parser should have checked this
                    self
                        .literals_generator
                        .generate_const_float(type_hint, value, node)
                }
                AstLiteral::String(string_value) => {
                    //don't look at the hint here, we probably need to cast a string
                    let t = self.annotations.get_type_or_void(node, self.index).get_type_information();
                    self
                        .literals_generator
                        .generate_const_string(t, string_value.value(), node)
                }
                AstLiteral::Array(_array) => todo!(),
                _ => {
                    unreachable!("Unsupported literal type {stmt:?}");
                }
            }
        };

        self.push_value(
            node,
            do_visit_literal().map_err(|e| Diagnostic::codegen_error(format!("{e}"), node.get_location())),
        );
    }

    fn visit_reference_expr(&mut self, stmt: &plc_ast::ast::ReferenceExpr, node: &AstNode) {
        let mut do_visit_reference = || -> anyhow::Result<GeneratedValue<'ink>> {
            match (&stmt.access, &stmt.base) {
                (ReferenceAccess::Global(ast_node), None) | (ReferenceAccess::Member(ast_node), None) => {
                    let annotation = self
                        .annotations
                        .get(ast_node)
                        .ok_or_else(|| anyhow!("Cannot find annotation for {ast_node:?}"))?;
                    self.generate_reference_access(node, &annotation, None)
                }
                (ReferenceAccess::Member(member_node), Some(base)) => {
                    let base_value = self.generate_expression(base)?;
                    let member_annotation = self
                        .annotations
                        .get(&member_node)
                        .ok_or_else(|| anyhow!("Cannot find annotation for {member_node:?}"))?;

                    self.generate_reference_access(
                        &member_node,
                        member_annotation,
                        Some(base_value.as_pointer_value()?),
                    )
                }
                (ReferenceAccess::Cast(inner_node), _) => {
                    if inner_node.is_literal() {
                        Ok(GeneratedValue::RValue((self.generate_r_value(inner_node)?, node.get_id())))
                    } else {
                        let annotation = self
                            .annotations
                            .get_hint(inner_node)
                            .or_else(|| self.annotations.get(inner_node))
                            .expect("no annotation found");
                        self.generate_reference_access(&inner_node, annotation, None)
                    }
                }
                (ReferenceAccess::Index(index_access), Some(array_reference)) => {
                    let access = array_access_generator::generate_element_pointer_for_array(
                        &array_reference,
                        &index_access,
                        self,
                    );
                    access.map(|v| GeneratedValue::LValue((v, node.get_id())))
                }
                (ReferenceAccess::Deref, Some(base)) => {
                    // dereference a pointer
                    let ptr = self.generate_expression(base)?;
                    let derefed = self.as_r_value_with_name(ptr, Some("deref"));
                    Ok(GeneratedValue::LValue((derefed.into_pointer_value(), node.get_id())))
                }
                _ => {
                    unreachable!("Unsupported reference type {stmt:?}");
                }
            }
        };

        let v =
            do_visit_reference().map_err(|e| Diagnostic::codegen_error(format!("{e}"), node.get_location()));
        self.push_value(node, v);
    }

    fn visit_unary_expression(&mut self, stmt: &plc_ast::ast::UnaryExpression, node: &AstNode) {
        let mut do_visit_unary_expression = || -> anyhow::Result<GeneratedValue<'ink>> {
            match stmt.operator {
                Operator::Plus => {
                    // nothing to do
                    self.generate_expression(&stmt.value)
                        .map_err(|e| anyhow!("Cannot generate unary plus: {e}"))
                }
                Operator::Minus => {
                    // generate a `0-expression`
                    let exp = self.generate_r_value(&stmt.value)?;
                    let negative_value = if exp.is_int_value() {
                        self.llvm
                            .builder
                            .build_int_sub(
                                exp.get_type().into_int_type().const_zero(),
                                exp.into_int_value(),
                                "tmpVar",
                            )
                            .as_basic_value_enum()
                    } else if exp.is_float_value() {
                        self.llvm
                            .builder
                            .build_float_sub(
                                exp.get_type().into_float_type().const_zero(),
                                exp.into_float_value(),
                                "tmpVar",
                            )
                            .as_basic_value_enum()
                    } else {
                        anyhow::bail!("Unsupported type for unary minus: {:?}", exp.get_type());
                    };
                    Ok(GeneratedValue::RValue((negative_value, node.get_id())))
                }
                Operator::Not => {
                    let operator = self.generate_r_value(&stmt.value)?.into_int_value();
                    let operator = if self
                        .get_type_hint_for(&stmt.value)
                        .map(|it| it.get_type_information().is_bool())
                        .unwrap_or_default()
                    {
                        self.make_bool_with_name(operator, "")?
                    } else {
                        operator
                    };

                    Ok(GeneratedValue::RValue((
                        self.llvm.builder.build_not(operator, "tmpVar").as_basic_value_enum(),
                        node.get_id(),
                    )))
                }
                _ => {
                    anyhow::bail!("Unsupported unary operator {:?}", stmt.operator);
                }
            }
        };
        let v = do_visit_unary_expression()
            .map_err(|e| Diagnostic::codegen_error(format!("{e}"), node.get_location()));
        self.push_value(node, v);
    }

    fn visit_binary_expression(&mut self, stmt: &plc_ast::ast::BinaryExpression, node: &AstNode) {
        let mut do_visit_binary_expression = || -> anyhow::Result<GeneratedValue<'ink>> {
            self.build_arithmetic_expression(stmt).map(|v| GeneratedValue::RValue((v, node.get_id())))
        };
        let v = do_visit_binary_expression()
            .map_err(|e| Diagnostic::codegen_error(format!("{e}"), node.get_location()));
        self.push_value(node, v);
    }

    fn visit_call_statement(&mut self, stmt: &plc_ast::ast::CallStatement, node: &AstNode) {
        let mut do_visit_call_statement = || -> anyhow::Result<GeneratedValue<'ink>> {
            let actual_parameters =
                stmt.parameters.as_deref().map(|p| flatten_expression_list(p)).unwrap_or_default();

            match self.annotations.get_call_name(&stmt.operator).zip(self.annotations.get(&stmt.operator)) {
                Some((call_name, StatementAnnotation::Function { qualified_name, .. })) => {
                    if let Some(builtin) = self.index.get_builtin_function(call_name) {
                        // this is a builtin function, we can generate it directly
                        //relabel the result to the call's id
                        // TODO: find a better solution for the result relabeling
                        let result = match builtin.codegen(self, &actual_parameters, node.get_location())? {
                            GeneratedValue::RValue((v, _)) => GeneratedValue::RValue((v, node.get_id())),
                            GeneratedValue::LValue((v, _)) => GeneratedValue::LValue((v, node.get_id())),
                            GeneratedValue::NoValue => todo!(),
                        };
                        Ok(result)
                    } else {
                        let function_to_call =
                            self.llvm_index.find_associated_implementation(call_name).expect("");

                        let pou = self
                            .index
                            .find_pou(&qualified_name)
                            .ok_or_else(|| anyhow!("Cannot find function {:#?}", qualified_name))?;

                        let formal_parameters = self
                            .index
                            .get_pou_members(pou.get_name())
                            .iter()
                            .filter(|e| e.is_parameter())
                            .collect_vec();

                        if pou.is_method() {
                            // method call
                            //todo: harmonize parameters with function case
                            self.generate_method_call(
                                pou,
                                function_to_call,
                                &actual_parameters,
                                &stmt.operator,
                                &node,
                            )
                        } else if pou.is_function() {
                            // function call
                            self.generate_function_call(
                                function_to_call,
                                &formal_parameters,
                                &actual_parameters,
                                &node,
                            )
                        } else {
                            bail!("Expected a function or method but got {:#?}", pou);
                        }
                    }
                }
                Some((call_name, StatementAnnotation::Program { qualified_name, .. })) => {
                    let prg_or_action = self.index.find_pou(qualified_name).expect("program not found");

                    // todo: move to helper method
                    // find the instance pointer for this call to a prg or action
                    // let container = self.index.find_pou(prg_or_action.get_container()).expect("Action without a container?");
                    // the operator should be a qualifed expression
                    let instance =
                        if let AstStatement::ReferenceExpr(ReferenceExpr { base: Some(base), .. }) =
                            &stmt.operator.get_stmt_peeled()
                        {
                            self.generate_expression(base)?.as_pointer_value()?
                        } else {
                            self.llvm_index
                                .find_loaded_associated_variable_value(
                                    format!("{}.{}", prg_or_action.get_container(), "__this").as_str(), //todo: use constant for this, maybe call it just "this"?
                                )
                                .or_else(
                                    || {
                                        self.llvm_index
                                            .find_global_value(prg_or_action.get_container())
                                            .map(|it| it.as_pointer_value())
                                    }, // works for pou and action
                                )
                                .expect("global value not found")
                        };

                    // program call
                    self.generate_member_call(
                        self.llvm_index.find_associated_implementation(call_name).expect(""),
                        &instance,
                        prg_or_action,
                        &actual_parameters,
                    );
                    Ok(GeneratedValue::NoValue)
                }
                Some((call_name, StatementAnnotation::Variable { resulting_type: qualified_name, .. })) => {
                    // function block instance
                    let pou = self.index.find_pou(&qualified_name).expect("");

                    let instance = self.generate_expression(&stmt.operator)?.as_pointer_value()?;
                    self.generate_member_call(
                        self.llvm_index.find_associated_implementation(call_name).expect(""),
                        &instance,
                        pou,
                        &actual_parameters,
                    );
                    Ok(GeneratedValue::NoValue)
                }
                Some((impl_name, StatementAnnotation::Value { .. })) => {
                    todo!()
                    // ???
                }
                _ => {
                    bail!("unknown callable item");
                }
            }
        };

        let call_result = do_visit_call_statement();
        self.push_value(
            node,
            call_result.map_err(|e| {
                Diagnostic::codegen_error(format!("Cannot generate call statement: {e}"), node.get_location())
            }),
        );
    }
}

// index helper methods
impl<'ink, 'a> ExpressionVisitor<'ink, 'a> {
    pub fn generate_expression(&mut self, expression: &AstNode) -> Result<GeneratedValue<'ink>, Diagnostic> {
        self.visit(expression);
        self.pop_value(expression)
    }

    pub fn generate_r_value(&mut self, expression: &AstNode) -> Result<BasicValueEnum<'ink>, Diagnostic> {
        let gv = self.generate_expression(expression)?;
        Ok(self.as_r_value(gv))
    }

    /// returns the data type associated to the given statement using the following strategy:
    /// - 1st try: fetch the type associated via the `self.annotations`
    /// - 2nd try: fetch the type associated with the given `default_type_name`
    /// - else return an `Err`
    pub fn get_type_hint_for(&self, statement: &AstNode) -> Result<&DataType, Diagnostic> {
        self.annotations
            .get_type_hint(statement, self.index)
            .or_else(|| self.annotations.get_type(statement, self.index))
            .and_then(|it| self.index.find_effective_type(it))
            .ok_or_else(|| {
                Diagnostic::codegen_error(format!("no type hint available for {statement:#?}"), statement)
            })
    }

    fn make_bool(&self, v: IntValue<'ink>) -> Result<IntValue<'ink>> {
        self.make_bool_with_name(v, "")
    }

    fn make_bool_with_name(&self, v: IntValue<'ink>, name: &str) -> Result<IntValue<'ink>> {
        if v.get_type().get_bit_width() > 1 {
            let zero = v.get_type().const_zero();
            Ok(self.llvm.builder.build_int_compare(IntPredicate::NE, v, zero, name))
        } else {
            Ok(v)
        }
    }
}
// Generation of Literals
impl<'ink, 'idx> ExpressionVisitor<'ink, 'idx> {
    pub fn push_value(&mut self, node: &AstNode, value: Result<GeneratedValue<'ink>, Diagnostic>) {
        self.result_stack.push(value);
    }

    pub fn pop_value(&mut self, a: &AstNode) -> Result<GeneratedValue<'ink>, Diagnostic> {
        let gv = self.result_stack.pop().unwrap_or_else(|| {
            Err(Diagnostic::codegen_error("No value on the stack".to_string(), SourceLocation::default()))
        })?;
        Ok(gv)
    }

    pub fn build_arithmetic_expression(&mut self, e: &BinaryExpression) -> Result<BasicValueEnum<'ink>> {
        let left_type = self.get_type_hint_for(&e.left)?.get_type_information();
        let right_type = self.get_type_hint_for(&e.right)?.get_type_information();

        if left_type.is_int() && right_type.is_int() {
            let build_int_compare =
                |predicate: IntPredicate, l: IntValue<'ink>, r: IntValue<'ink>| -> IntValue<'ink> {
                    self.llvm.builder.build_int_compare(predicate, l, r, "tmpVar")
                };

            if e.operator == Operator::Or || e.operator == Operator::And {
                // operators with short circuit evaluation so we must not eagerly evaluate the right side
                self.generate_bool_short_circuit_expression(&e.operator, &e.left, &e.right)
            } else if e.operator == Operator::Xor {
                // this is only its own if-branch to keep compatible with test-snapshots (the order of loads and icmp!)
                // todo: re-integrate into match below
                Ok(self
                    .llvm
                    .builder
                    .build_xor(
                        {
                            let l = self.generate_r_value(&e.left)?.into_int_value();
                            self.make_bool(l)?
                        },
                        {
                            let r = self.generate_r_value(&e.right)?.into_int_value();
                            self.make_bool(r)?
                        },
                        "",
                    )
                    .as_basic_value_enum())
            } else {
                let (l, r) = (
                    self.generate_r_value(&e.left)?.into_int_value(),
                    self.generate_r_value(&e.right)?.into_int_value(),
                );
                let v = match e.operator {
                    //arithmetic
                    Operator::Plus => self.llvm.builder.build_int_add(l, r, "tmpVar"),
                    Operator::Minus => self.llvm.builder.build_int_sub(l, r, "tmpVar"),
                    Operator::Multiplication => self.llvm.builder.build_int_mul(l, r, "tmpVar"),
                    Operator::Division => self.llvm.builder.build_int_signed_div(l, r, "tmpVar"),
                    //comparisons
                    Operator::Equal => build_int_compare(IntPredicate::EQ, l, r),
                    Operator::NotEqual => build_int_compare(IntPredicate::NE, l, r),
                    Operator::Less => build_int_compare(IntPredicate::SLT, l, r),
                    Operator::Greater => build_int_compare(IntPredicate::SGT, l, r),
                    Operator::LessOrEqual => build_int_compare(IntPredicate::SLE, l, r),
                    Operator::GreaterOrEqual => build_int_compare(IntPredicate::SGE, l, r),
                    // others
                    Operator::Modulo => self.llvm.builder.build_int_signed_rem(l, r, "tmpVar"),
                    _ => {
                        anyhow::bail!("Unsupported operator {:?} for int values", e.operator);
                    }
                };
                Ok(v.as_basic_value_enum())
            }
        } else if left_type.is_float() && right_type.is_float() {
            let build_float_compare = |predicate: FloatPredicate,
                                       l: FloatValue<'ink>,
                                       r: FloatValue<'ink>|
             -> BasicValueEnum<'ink> {
                self.llvm.builder.build_float_compare(predicate, l, r, "tmpVar").as_basic_value_enum()
            };
            let (l, r) = (
                self.generate_r_value(&e.left)?.into_float_value(),
                self.generate_r_value(&e.right)?.into_float_value(),
            );
            let v = match e.operator {
                Operator::Plus => self.llvm.builder.build_float_add(l, r, "tmpVar").as_basic_value_enum(),
                Operator::Minus => self.llvm.builder.build_float_sub(l, r, "tmpVar").as_basic_value_enum(),
                Operator::Multiplication => {
                    self.llvm.builder.build_float_mul(l, r, "tmpVar").as_basic_value_enum()
                }
                Operator::Division => self.llvm.builder.build_float_div(l, r, "tmpVar").as_basic_value_enum(),
                Operator::Equal => build_float_compare(FloatPredicate::OEQ, l, r),
                Operator::NotEqual => build_float_compare(FloatPredicate::ONE, l, r),
                Operator::Less => build_float_compare(FloatPredicate::OLT, l, r),
                Operator::Greater => build_float_compare(FloatPredicate::OGT, l, r),
                Operator::LessOrEqual => build_float_compare(FloatPredicate::OLE, l, r),
                Operator::GreaterOrEqual => build_float_compare(FloatPredicate::OGE, l, r),
                // others
                Operator::Modulo => self.llvm.builder.build_float_rem(l, r, "tmpVar").as_basic_value_enum(),
                _ => {
                    anyhow::bail!("Unsupported operator {:?} for float values", e.operator);
                }
            };
            Ok(v)
        } else {
            anyhow::bail!(
                "Unsupported types ({:?},{:?}) for operator {:?}",
                left_type.get_name(),
                right_type.get_name(),
                e.operator
            );
        }
    }

    /// generates a phi-expression (&& or || expression) with respect to short-circuit evaluation
    ///
    /// - `operator` AND / OR
    /// - `left` the left side of the expression as an i1 value
    /// - `right` the right side of an expression as an i1 value
    fn generate_bool_short_circuit_expression(
        &mut self,
        operator: &Operator,
        left: &AstNode,
        right: &AstNode,
    ) -> Result<BasicValueEnum<'ink>> {
        let builder = &self.llvm.builder;
        let l = self.generate_r_value(left)?.into_int_value();
        let lhs = self.make_bool(l)?;
        let function = self
            .llvm
            .builder
            .get_insert_block()
            .and_then(|it| it.get_parent())
            .ok_or_else(|| anyhow!("Cannot get function"))?;

        let right_branch = self.llvm.context.append_basic_block(function, "");
        let continue_branch = self.llvm.context.append_basic_block(function, "");

        let final_left_block = builder.get_insert_block().ok_or_else(|| anyhow!("Cannot get block"))?;
        //Compare left to 0

        match operator {
            Operator::Or => builder.build_conditional_branch(lhs, continue_branch, right_branch),
            Operator::And => builder.build_conditional_branch(lhs, right_branch, continue_branch),
            _ => {
                anyhow::bail!("Unsupported operator {operator:?} for short-circuit evaluation");
            }
        };

        builder.position_at_end(right_branch);

        let rhs = self.generate_r_value(right)?.into_int_value();
        let rhs = self.make_bool(rhs)?;
        let final_right_block = builder.get_insert_block().ok_or_else(|| anyhow!("Cannot get block"))?;
        builder.build_unconditional_branch(continue_branch);

        builder.position_at_end(continue_branch);
        //Generate phi
        let phi_value = builder.build_phi(lhs.get_type(), "");
        //assert
        phi_value.add_incoming(&[(&lhs, final_left_block), (&rhs, final_right_block)]);

        Ok(phi_value.as_basic_value())
    }
}

/// implementation block for function calls
impl<'ink, 'idx> ExpressionVisitor<'ink, 'idx> {
    fn generate_function_call(
        &mut self,
        fv: FunctionValue<'ink>,
        formal_parameters: &[&VariableIndexEntry],
        actual_parameters: &[&AstNode],
        ast_node: &AstNode,
    ) -> Result<GeneratedValue<'ink>> {
        let arguments = formal_parameters
            .iter()
            .zip(actual_parameters.iter())
            .map(|(formal, actual)| Argument::new(formal, actual))
            .collect::<Vec<_>>();

        let call = CallArguments::new(self.annotations, self.index, self.llvm, arguments);
        call.generate_function_call(fv, self, ast_node)

        //todo: check if we can have defualt parameters in functions

        // let args = self.generate_all(
        //     &actual_parameters.iter().cloned().zip(formal_parameters.iter().cloned()).collect_vec(),
        // )?;

        // let function_result = self.llvm.builder.build_call(fv, args.as_slice(), "call"); //todo we should use the function's name here?

        // // reutrn either the return value or a NoValue
        // Ok(function_result
        //     .try_as_basic_value()
        //     .left()
        //     .map(|it| GeneratedValue::RValue((it, ast_node.get_id())))
        //     .unwrap_or_else(|| GeneratedValue::NoValue))
    }

    fn generate_method_call(
        &mut self,
        method: &PouIndexEntry,
        fv: FunctionValue<'ink>,
        // formal_parameters: &[&VariableIndexEntry],
        actual_parameters: &[&AstNode],
        call_operator: &AstNode,
        ast_node: &AstNode,
    ) -> Result<GeneratedValue<'ink>> {
        //todo: cleanup this handling, who is this from within a method?
        //       unify implementatino for action method calling
        let instance = if let AstStatement::ReferenceExpr(ReferenceExpr { base: Some(base), .. }) =
            &call_operator.get_stmt_peeled()
        {
            self.generate_expression(base)?.as_pointer_value()?
        } else {
            self.llvm_index
                .find_loaded_associated_variable_value(
                    format!("{}.{}", method.get_container(), "__this").as_str(), //todo: use constant for this, maybe call it just "this"?
                )
                .or_else(
                    || {
                        self.llvm_index
                            .find_global_value(method.get_container())
                            .map(|it| it.as_pointer_value())
                    }, // works for pou and action
                )
                .expect("global value not found")
        };

        // fill in default parameters at the end
        let formal_parameters = self
            .index
            .get_pou_members(method.get_name())
            .iter()
            .filter(|e| e.is_parameter())
            .sorted_by_key(|p| p.get_location_in_parent()) //TODO: is this necessary
            .collect_vec();

        // see if we have explicit parameters
        let args = if actual_parameters.iter().any(|it| it.is_assignment() || it.is_output_assignment()) {
            let mut parameter_by_name = actual_parameters
                .iter()
                .map(|p| {
                    if let AstStatement::Assignment(Assignment { left, right: actual, .. })
                    | AstStatement::OutputAssignment(Assignment { left, right: actual, .. }) = p.get_stmt()
                    {
                        if let Some(AstStatement::Identifier(name)) =
                            left.as_ref().get_identifier().map(|n| n.get_stmt())
                        {
                            return Ok((name.as_str(), actual));
                        }
                    }
                    Err(anyhow!(format!("Expected assignment for {p:#?}"),))
                })
                .collect::<Result<HashMap<_, _>>>()?;

            // now we should either find this parameter, or a default value
            let parameters = formal_parameters
                .iter()
                .map(|ap| {
                    if let Some(actual) = parameter_by_name.remove(ap.get_name()) {
                        return Ok(actual.as_ref());
                    } else {
                        self.index
                            .get_const_expressions()
                            .maybe_get_constant_statement(&ap.initial_value)
                            .ok_or_else(|| anyhow!("No default value for parameter {}", ap.get_name()))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            parameters
        } else {
            let mut args = Vec::from(actual_parameters);
            // let mut args = self.generate_all(actual_parameters)?;

            if args.len() < formal_parameters.len() {
                let default_parameters = formal_parameters
                    .iter()
                    .skip(args.len())
                    .flat_map(|e| {
                        self.index.get_const_expressions().maybe_get_constant_statement(&e.initial_value)
                    })
                    .collect_vec();
                args.extend(default_parameters);
            }

            // assert_eq!(args.len(), formal_parameters.len());

            args
        };

        let mut args =
            self.generate_all(&args.into_iter().zip(formal_parameters.into_iter()).collect_vec())?;

        // insert the instance pointer as first argument
        args.insert(0, instance.into());
        let function_result = self.llvm.builder.build_call(fv, args.as_slice(), "call"); //todo we should use the function's name here?

        // reutrn either the return value or a NoValue
        Ok(function_result
            .try_as_basic_value()
            .left()
            .map(|it| GeneratedValue::RValue((it, ast_node.get_id())))
            .unwrap_or_else(|| GeneratedValue::NoValue))
    }

    /// generates a call to a program or function block
    fn generate_member_call(
        &mut self,
        fv: FunctionValue<'ink>,
        instance: &PointerValue<'ink>,
        prg: &PouIndexEntry,
        actual_parameters: &[&AstNode],
    ) -> Result<()> {
        

        //if this is a method call we need to pass the instance pointer as first argument
        // if prg.is_method() {
        //     arguments.insert(0, Argument::new(formal, actual)); //todo: introduce this
        // }

        let arguments = call_builder::build_arguments_list(self, prg.get_name(), actual_parameters)?;
        call_builder::program_generate_in_arguments(self, instance, arguments.as_slice())?;
        call_builder::program_build_call(self, fv, instance, prg.get_name());
        call_builder::program_generate_out_parameters(self, instance, arguments.as_slice())?;


        // let call = CallArguments::new(self.annotations, self.index, self.llvm, arguments);
        // let _ = call.generate_program_call(fv, instance, self)?;

        Ok(())
    }

    fn generate_all(
        &mut self,
        actual_parameters: &[(&AstNode, &VariableIndexEntry)],
    ) -> Result<Vec<inkwell::values::BasicMetadataValueEnum<'ink>>> {
        let args = actual_parameters
            .iter()
            .map(|(value, e)| {
                dbg!(self.annotations.get(value));
                dbg!(self.annotations.get_hint(value));

                self.visit(value);
                let v = self.pop_value(value);
                if e.is_inout() || e.is_output() {
                    v.and_then(|it| {
                        it.as_pointer_value().map(|it| it.into()).map_err(|e| {
                            Diagnostic::codegen_error(
                                format!("Cannot generate inout parameter: {e}"),
                                value.get_location(),
                            )
                        })
                    }) // TODO: use anyhow error
                } else {
                    v.map(|v| self.as_r_value(v).into())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(args)
    }
}

impl<'ink, 'idx> ExpressionVisitor<'ink, 'idx> {
    /// Generates a reference to a global, unscoped variable
    pub fn generate_reference_access(
        &mut self,
        reference: &AstNode,
        annotation: &StatementAnnotation,
        context: Option<PointerValue<'ink>>,
    ) -> Result<GeneratedValue<'ink>> {
        let ast_id = reference.get_id();
        let access = match (annotation, context) {
            // GLOBAL VARIABLE
            // a Variable without a context (could be a global or a member)
            (StatementAnnotation::Variable { qualified_name, constant: false, .. }, None) => self
                .find_local_llvm_variable(&qualified_name.as_str(), ast_id)
                .or_else(|| self.find_global_llvm_variable(qualified_name.as_str(), ast_id)),

            (StatementAnnotation::Variable { qualified_name, constant: true, .. }, None) => {
                let constant_expression = self
                    .index
                    .find_fully_qualified_variable(&qualified_name)
                    .and_then(|it: &VariableIndexEntry| it.initial_value)
                    .and_then(|id| self.index.get_const_expressions().get_constant_statement(&id))
                    .ok_or_else(|| anyhow!("Cannot find constant {:#?}", qualified_name))?;
                Some(GeneratedValue::RValue((self.generate_r_value(constant_expression)?, ast_id)))
            }
            // MEMBER VARIABLE
            // a Variable with a context (certainly a member of a struct, fb or program)
            (StatementAnnotation::Variable { qualified_name, .. }, Some(parent)) => {
                // generate a gep in parent
                let location_in_parent = self
                    .index
                    .find_fully_qualified_variable(&qualified_name)
                    .ok_or_else(|| anyhow!("Cannot find variable {:#?}", qualified_name))?
                    .get_location_in_parent();

                let name = self.get_load_name(ast_id).unwrap_or("");
                let gep =
                    self.llvm.builder.build_struct_gep(parent, location_in_parent, name).map_err(|e| {
                        anyhow!("Cannot generate gep for variable {qualified_name:#?}: {e:?}")
                    })?;
                Some(GeneratedValue::LValue((gep, ast_id)))
            }

            // PROGRAM REFERENCE
            // a reference to a program, this wants a pointer to the instance struct of that program
            (StatementAnnotation::Program { qualified_name }, _) => {
                // an identifier that resolves to a program
                // e.g. variable := PLC_PRG.x;
                //                  ^^^^^^^
                let PouIndexEntry::Program { instance_variable, .. } =
                    self.index
                        .find_pou(&qualified_name)
                        .ok_or_else(|| anyhow!("Cannot find program {:#?}", qualified_name))?
                else {
                    return Err(anyhow!("Expected program but got {:#?}", annotation));
                };

                self.find_global_llvm_variable(instance_variable.get_qualified_name(), ast_id)
            }

            _ => return Err(anyhow!("Expected member_variable but got {:#?}", annotation)),
        };

        access.map(|it| self.auto_deref_if_necessary(it)).ok_or_else(|| {
            anyhow!(
                "Cannot generate access for identifier {:#?}",
                self.annotations.get_identifier_name(reference).unwrap_or("<unknown>")
            )
        })
    }

    fn auto_deref_if_necessary(&self, v: GeneratedValue<'ink>) -> GeneratedValue<'ink> {
        if let GeneratedValue::LValue((_, id)) = v {
            // check if we need to deref
            if self.annotations.get_with_id(id).is_some_and(|opt| opt.is_auto_deref()) {
                return GeneratedValue::LValue((
                    self.load_value_with_identifier(v, Some("deref")).0.into_pointer_value(),
                    id,
                ));
            }
        }
        v
    }

    fn find_local_llvm_variable(&self, llvm_qualified_name: &str, id: AstId) -> Option<GeneratedValue<'ink>> {
        self.llvm_index
            .find_loaded_associated_variable_value(llvm_qualified_name)
            .map(|v| GeneratedValue::LValue((v, id)))
    }

    fn find_global_llvm_variable(
        &self,
        llvm_qualified_name: &str,
        id: AstId,
    ) -> Option<GeneratedValue<'ink>> {
        self.llvm_index
            .find_global_value(&llvm_qualified_name)
            .map(|v| GeneratedValue::LValue((v.as_pointer_value(), id)))
    }

    pub fn as_r_value(&self, v: GeneratedValue<'ink>) -> BasicValueEnum<'ink> {
        self.as_r_value_with_name(v, None)
    }

    pub fn as_r_value_with_name(&self, v: GeneratedValue<'ink>, name: Option<&str>) -> BasicValueEnum<'ink> {
        let (r_value, id) = self.load_value_with_identifier(v, name);

        //see if we need to cast here
        let Some(target_type) = self
            .annotations
            .get_hint_with_id(id)
            .and_then(|hint| self.annotations.get_type_for_annotation(self.index, hint))
        else {
            // no type-hint -> we can return the value as is
            return r_value.as_basic_value_enum();
        };
        let actual_type = self
            .annotations
            .get_with_id(id)
            .and_then(|hint| self.annotations.get_type_for_annotation(self.index, hint))
            .unwrap_or_else(|| self.index.get_void_type());

        crate::codegen::llvm_typesystem::cast(
            self.llvm,
            self.index,
            self.llvm_index,
            target_type,
            actual_type,
            r_value.as_basic_value_enum(),
            self.annotations.get_with_id(id),
        )
    }

    //TODO: clean up mess with load_value_with_identifier vs. as_r_value_with_name
    // this function only loads but does not cast the value if necessary
    fn load_value_with_identifier(
        &self,
        v: GeneratedValue<'ink>,
        name: Option<&str>,
    ) -> (BasicValueEnum<'ink>, usize) {
        let (r_value, id) = match v {
            GeneratedValue::RValue((v, id)) => (v, id),
            GeneratedValue::LValue((v, id)) => {
                if let Some(name) = name {
                    // if we have a name, we can use it
                    (self.llvm.builder.build_load(v, name), id)
                } else {
                    // if we don't have a name, we need to generate one
                    (
                        self.llvm.builder.build_load(
                            v,
                            self.get_load_name(id)
                                .map(|name| format!("load_{}", name))
                                .as_deref()
                                .unwrap_or(""),
                        ),
                        id,
                    )
                }
            }
            GeneratedValue::NoValue => panic!("No value"),
        };
        (r_value, id)
    }
}

