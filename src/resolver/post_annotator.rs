use plc_ast::{
    ast::{
        flatten_expression_list, AstFactory, AstNode, AstStatement, CallStatement, Operator, ReferenceAccess,
        TypeNature,
    },
    control_statements::AstControlStatement,
    literals::{Array, AstLiteral},
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    name_resolver::LiteralsAnnotator,
    typesystem::{get_bigger_type, DataType, DataTypeInformation, BOOL_TYPE, VOID_TYPE},
};

use super::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};

pub struct PostAnnotator<'i> {
    index: &'i Index,
    pub annotations: AnnotationMapImpl,
    id_provider: IdProvider,
}

impl<'i> PostAnnotator<'i> {
    pub fn new(
        index: &'i Index,
        annotations: AnnotationMapImpl,
        id_provider: IdProvider,
    ) -> PostAnnotator<'i> {
        Self { index, annotations, id_provider }
    }

    // tries to call one of the EQUAL_XXX, LESS_XXX, ... functions for the
    // given type (of left). The stmt's operator has to be a comparison-operator
    fn replace_compare_statement_with_custom_call(
        &mut self,
        left: &AstNode,
        op: &Operator,
        right: &AstNode,
        original_statement: &AstNode,
    ) {
        let location = &original_statement.get_location();
        let replacement_ast = match &op {
            // a <> b expression is handled as Not(Equal(a,b))
            Operator::NotEqual => AstFactory::create_not_expression(
                self.create_typed_compare_call_statement(left, &Operator::Equal, right, location),
                location.clone(),
                self.id_provider.next_id(),
            ),
            // a <= b expression is handled as a = b OR a < b
            Operator::LessOrEqual => AstFactory::create_or_expression(
                self.create_typed_compare_call_statement(left, &Operator::Equal, right, location),
                self.create_typed_compare_call_statement(left, &Operator::Less, right, location),
            ),
            // a >= b expression is handled as a = b OR a > b
            Operator::GreaterOrEqual => AstFactory::create_or_expression(
                self.create_typed_compare_call_statement(left, &Operator::Equal, right, location),
                self.create_typed_compare_call_statement(left, &Operator::Greater, right, location),
            ),
            _ => self.create_typed_compare_call_statement(left, op, right, location),
        };
        // self.visit_statement(&ctx, &call_statement);
        // self.update_expected_types(self.index.get_type_or_panic(typesystem::BOOL_TYPE), &call_statement);

        self.annotations.annotate(
            &original_statement,
            StatementAnnotation::ReplacementAst { statement: replacement_ast },
        );
        // self.update_expected_types(self.index.get_type_or_panic(typesystem::BOOL_TYPE), statement);
    }

    /// creates a call statement to the operator's corresponding custom compare function
    fn create_typed_compare_call_statement(
        &mut self,
        left: &AstNode,
        operator: &Operator,
        right: &AstNode,
        location: &SourceLocation,
    ) -> AstNode {
        let left_type = self
            .annotations
            .get_type_hint(left, self.index)
            .unwrap_or_else(|| self.annotations.get_type_or_void(left, self.index));
        if let Some(function_name) = crate::typesystem::get_equals_function_name_for(
            left_type.get_type_information().get_name(),
            operator,
        ) {
            let call = AstFactory::create_call_to(
                function_name.to_string(),
                vec![left.clone(), right.clone()],
                self.id_provider.next_id(),
                left.get_id(),
                location,
            );

            if let AstStatement::CallStatement(CallStatement { operator, .. }) = call.get_stmt() {
                self.annotations.annotate(
                    &operator,
                    StatementAnnotation::Function {
                        return_type: BOOL_TYPE.to_string(),
                        qualified_name: function_name.to_string(),
                        call_name: None,
                    },
                );
                self.annotations.copy_annotation(&operator, &call);
            }
            self.annotations.annotate_type_hint(&call, StatementAnnotation::value(BOOL_TYPE));
            call
        } else {
            AstFactory::create_empty_statement(location.clone(), self.id_provider.next_id())
        }
    }
}

impl AstVisitor for PostAnnotator<'_> {
    fn visit_binary_expression(
        &mut self,
        stmt: &plc_ast::ast::BinaryExpression,
        node: &plc_ast::ast::AstNode,
    ) {

        let l_type = self.annotations.get_type_or_void(&stmt.left, self.index);
        let r_type = self.annotations.get_type_or_void(&stmt.right, self.index);
        let stmt_type = self.annotations.get_type_or_void(node, self.index);

        let (l_arithmetic, l_numerical, l_pointer) =
            (l_type.is_arithmetic(), l_type.is_numerical(), l_type.is_pointer());
        let (r_arithmetic, r_numerical, r_pointer) =
            (r_type.is_arithmetic(), r_type.is_numerical(), r_type.is_pointer());

        // TODO: make this 1 if
        if stmt.operator.is_arithmetic_operator() && (stmt_type.is_arithmetic()) && l_arithmetic && r_arithmetic
        {
            // upscale left & right to the same type
            let type_name = stmt_type.get_name().to_string();
            self.annotations.annotate_type_hint(&stmt.left, StatementAnnotation::value(type_name.clone()));
            self.annotations.annotate_type_hint(&stmt.right, StatementAnnotation::value(type_name));
        } else if stmt.operator.is_comparison_operator() && l_arithmetic && r_arithmetic {
            // upscale left and right ot the same type
            let type_name = get_bigger_type(l_type, r_type, self.index).get_name().to_string();
            self.annotations.annotate_type_hint(&stmt.left, StatementAnnotation::value(type_name.clone()));
            self.annotations.annotate_type_hint(&stmt.right, StatementAnnotation::value(type_name));
        }

        if stmt.operator.is_comparison_operator()
            // one of the operants is not numerical or pointer?
            && ((!l_numerical && !l_pointer)
                || !r_numerical && !r_pointer)
        {
            // replace the comparison operation with a call
            self.replace_compare_statement_with_custom_call(&stmt.left, &stmt.operator, &stmt.right, node);
        }

        stmt.walk(self);
    }

    fn visit_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
        self.visit(&stmt.left);
        // eagerly hint the right side, so future traversal can react appropriately
        if let Some(l_type) = self.annotations.get_type(&stmt.left, self.index) {
            self.annotations
                .annotate_type_hint(&stmt.right, StatementAnnotation::value(l_type.get_name().to_string()));
        }
        self.visit(&stmt.right);
    }

    fn visit_reference_expr(&mut self, stmt: &plc_ast::ast::ReferenceExpr, node: &plc_ast::ast::AstNode) {
        stmt.walk(self);

        // a cast-statement on a literal should directly annotate the literal correctly (avoid casts)
        if let ReferenceAccess::Cast(target) = &stmt.access {
            if matches!(target.get_stmt(), AstStatement::Literal { .. }) {
                self.annotations.copy_annotation(node, target);
                self.annotations.clear_type_hint(&target);
            }
        }
    }

    fn visit_control_statement(
        &mut self,
        stmt: &plc_ast::control_statements::AstControlStatement,
        _node: &plc_ast::ast::AstNode,
    ) {
        stmt.walk(self);

        // type-hint all the case-conditions with the selector's type
        if let AstControlStatement::Case(case) = stmt {
            if let Some(selector_type) =
                self.annotations.get_type_name(&case.selector).map(StatementAnnotation::value)
            {
                for c in case.case_blocks.iter().flat_map(|cb| flatten_expression_list(&cb.condition)) {
                    self.annotations.annotate_type_hint(c, selector_type.clone());
                }
            }
        }
    }

    fn visit_paren_expression(&mut self, inner: &AstNode, node: &AstNode) {
        inner.walk(self);

        if let Some(expected_type_name) =
            self.annotations.get_type_hint_or_type(node, self.index).map(|t| t.get_name())
        {
            let expected_type = self
                .index
                .find_effective_type_by_name(expected_type_name)
                .unwrap_or_else(|| self.index.get_void_type());

            // a parenthesized expression that should be a struct -> struct-literal
            if expected_type.is_struct() {
                let mut literal_annotator = LiteralsAnnotator::new(
                    expected_type.get_type_information(),
                    self.index,
                    &mut self.annotations,
                );
                literal_annotator.visit(node);
            }
        }
    }

    //TODO: why does this cause several tests to fail?
    // fn visit_call_statement(&mut self, stmt: &CallStatement, node: &AstNode) {
    //     // this is a bold optimization to only look for replacement ASTs in calls!!! :-/
    //     if let Some(StatementAnnotation::ReplacementAst { statement: replacement}) = self.annotations.take(node) {
    //         replacement.walk(self);
    //         // copy the top level annotation back to the original one
    //         self.annotations.copy_annotation(&replacement, node);
    //         //re-attached replacement ast
    //         self.annotations.annotate(node, StatementAnnotation::ReplacementAst { statement: replacement });
    //     }else{
    //         stmt.walk(self);

    //     }
    // }

    fn visit_literal(&mut self, stmt: &plc_ast::literals::AstLiteral, node: &AstNode) {
        stmt.walk(self);

        if let AstLiteral::Array(Array { elements: Some(elements) }) = &stmt {
            if let Some(DataTypeInformation::Array { name, inner_type_name, .. }) =
                self.annotations.get_type_hint_or_type(node, self.index).map(|it| it.get_type_information())
            {
                for e in elements.get_as_list() {
                    self.annotations
                        .annotate_type_hint(e, StatementAnnotation::value(inner_type_name.clone()));
                    self.visit(e);
                }
                self.annotations.annotate(node, StatementAnnotation::value(name));
                self.annotations.clear_type_hint(node);
            }
        }

        // upscale a literal if it looks streight forward
        let num_type =
            self.annotations.get_type(node, self.index);
        let hint_type = self.annotations.get_type_hint(node, self.index);

        let num_name = num_type.map(|t| t.get_name());
        let hint_name = hint_type.map(|t| t.get_name());
        if let Some((num_type, hint_type)) = num_type.zip(hint_type) {
            if num_type.is_numerical() && hint_type.is_real() {
                // promote the hint to the type-annotation
                self.annotations.annotate(node, StatementAnnotation::value(hint_type.get_name()));
                self.annotations.clear_type_hint(node)
            }
        }

        // if let Some((num_type, hint_type)) = num_type.zip(hint_type) {
        //     if num_type.is_numerical() && hint_type.is_numerical() && hint_type.is_compatible_with_type(num_type) {
        //         // promote the hint to the type-annotation
        //         self.annotations.annotate(node, StatementAnnotation::value(hint_type.get_name()));
        //         self.annotations.clear_type_hint(node)
        //     }
        // }

        // let hint_name = self.annotations.get_type_hint(node, self.index).map(|t| t.get_name());
        // let hint_num_type = hint_name
        //     .and_then(|type_name| self.index.find_type(type_name))
        //     .map(|it| it.is_numerical() || it.is_compatible_with_type(other))
        //     .unwrap_or(false);

        // if real_num_type && hint_num_type {
        //     if let Some(type_name) = hint_name {
        //         self.annotations.annotate_type_hint(node, StatementAnnotation::value(type_name));
        //         self.annotations.clear_type_hint(node);
        //     }
        // }
    }
}
