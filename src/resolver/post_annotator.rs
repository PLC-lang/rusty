use plc_ast::{
    ast::{
        flatten_expression_list, AstFactory, AstNode, AstStatement, CallStatement, Operator, ReferenceAccess,
    },
    control_statements::AstControlStatement,
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::{Index, PouIndexEntry},
    name_resolver::NameResolver,
    typesystem::BOOL_TYPE,
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
        if let Some(function_name) =
            crate::typesystem::get_equals_function_name_for(
                left_type.get_type_information().get_name(),
                operator,
            )
        {
            let call = AstFactory::create_call_to(
                function_name.to_string(),
                vec![left.clone(), right.clone()],
                self.id_provider.next_id(),
                left.get_id(),
                location,
            );

            if let AstStatement::CallStatement(CallStatement { operator, ..}) = call.get_stmt() {
                self.annotations.annotate(
                    &operator,
                    StatementAnnotation::Function {
                        return_type: BOOL_TYPE.to_string(),
                        qualified_name: function_name.to_string(),
                        call_name: None,
                    },
                );
                self.annotations.copy_annotation(&operator, &call)
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
        stmt.walk(self);

        let l_type = self.annotations.get_type_or_void(&stmt.left, self.index);
        let r_type = self.annotations.get_type_or_void(&stmt.right, self.index);
        let stmt_type = self.annotations.get_type_or_void(node, self.index);

        let (l_arithmetic, l_numerical, l_pointer) =
            (l_type.is_arithmetic(), l_type.is_numerical(), l_type.is_pointer());
        let (r_arithmetic, r_numerical, r_pointer) =
            (r_type.is_arithmetic(), r_type.is_numerical(), r_type.is_pointer());

        if stmt.operator.is_arithmetic_operator() && stmt_type.is_arithmetic() && l_arithmetic && r_arithmetic
        {
            // hint left and right with the resulting type
            let type_name = stmt_type.get_name().to_string();
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
    }

    fn visit_assignment(&mut self, stmt: &plc_ast::ast::Assignment, _node: &plc_ast::ast::AstNode) {
        stmt.walk(self);
        if let Some(l_type) = self.annotations.get_type(&stmt.left, self.index) {
            self.annotations
                .annotate_type_hint(&stmt.right, StatementAnnotation::value(l_type.get_name().to_string()));
        }
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
}
