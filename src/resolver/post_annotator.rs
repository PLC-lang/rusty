use std::collections::HashMap;

use plc_ast::{
    ast::{
        flatten_expression_list, AstFactory, AstNode, AstStatement, CallStatement, Operator, ReferenceAccess,
    },
    control_statements::AstControlStatement,
    literals::{Array, AstLiteral},
    provider::IdProvider,
    visitor::{AstVisitor, Walker},
};
use plc_source::source_location::SourceLocation;

use crate::{
    index::{Index, PouIndexEntry},
    name_resolver::LiteralsAnnotator,
    typesystem::{
        get_bigger_type, DataType, DataTypeInformation, InternalType, StructSource, BOOL_TYPE, VOID_TYPE,
    },
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

    /// if the given node is a vla, this function returns an necessary hint-annotation for the node
    fn get_vla_hint_if_necessary(&mut self, node: &AstNode) -> Option<StatementAnnotation> {
        if let Some(StatementAnnotation::Variable { resulting_type, qualified_name, argument_type, .. }) =
            self.annotations.get(node)
        {
            if let Some(DataTypeInformation::Struct {
                source: StructSource::Internal(InternalType::VariableLengthArray { .. }),
                members,
                ..
            }) = self.index.find_effective_type_info(resulting_type)
            {
                if let Some(DataTypeInformation::Pointer { inner_type_name, .. }) =
                    self.index.find_effective_type_info(
                        members.get(0).map(|it| it.get_type_name()).unwrap_or(VOID_TYPE),
                    )
                {
                    let hint_annotation = StatementAnnotation::Variable {
                        resulting_type: inner_type_name.to_owned(),
                        qualified_name: qualified_name.to_string(),
                        constant: false,
                        argument_type: argument_type.clone(),
                        is_auto_deref: false,
                    };
                    return Some(hint_annotation);
                }
            }
        }
        None
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
        if stmt.operator.is_arithmetic_operator()
            && (stmt_type.is_arithmetic())
            && l_arithmetic
            && r_arithmetic
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

        // if this was a vla, we update a typehint
        if let Some(vla_annotation) = self.get_vla_hint_if_necessary(node) {
            self.annotations.annotate_type_hint(node, vla_annotation)
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
        let num_type = self.annotations.get_type(node, self.index);
        let hint_type = self.annotations.get_type_hint(node, self.index);

        if let Some((num_type, hint_type)) = num_type.zip(hint_type) {
            if num_type != hint_type
                && (num_type.is_numerical() || num_type.is_bool())
                && num_type.is_compatible_with_type(hint_type)
            {
                //TODO: make a clean decision whether literals should be
                // hinted or not

                // promote the hint to the type-annotation
                self.annotations.annotate(node, StatementAnnotation::value(hint_type.get_name()));
                self.annotations.clear_type_hint(node)
            }
        }
    }

    fn visit_call_statement(&mut self, stmt: &CallStatement, _node: &AstNode) {
        stmt.walk(self);

        let mut cleard_nodes = Vec::new();
        {
            if let Some(PouIndexEntry::Function { generics, .. }) =
                self.annotations.get_call_name(&stmt.operator).and_then(|n| self.index.find_pou(n))
            {
                let mut resolved_generics: HashMap<&str, Option<&str>> =
                    generics.iter().map(|generic| (generic.name.as_str(), None)).collect();

                // re-check the parameteres for any unresolved generics
                let parameteres = stmt.parameters.as_ref().map(|it| it.get_as_list()).unwrap_or_default();
                for p in parameteres {
                    if let (Some(DataTypeInformation::Generic { generic_symbol, .. }), Some(found_type)) = (
                        self.annotations.get_type_hint(p, self.index).map(DataType::get_type_information),
                        self.annotations.get_type(p, self.index),
                    ) {
                        if let Some(None) = resolved_generics.get_mut(generic_symbol.as_str()) {
                            resolved_generics.insert(generic_symbol.as_str(), Some(found_type.get_name()));
                            cleard_nodes.push(p);
                        }
                    }
                }
                dbg!(resolved_generics);
            }
        }
        for ele in cleard_nodes {
            self.annotations.clear_type_hint(ele);
        }
    }
}
