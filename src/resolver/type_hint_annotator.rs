use plc_ast::{ast::{AstStatement, ReferenceAccess, ReferenceExpr}, visitor::{AstVisitor, Walker}};

use crate::index::Index;

use super::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};

pub struct TypeHintAnnotator<'i> {
    index: &'i Index,
    pub annotations: AnnotationMapImpl,
}

impl<'i> TypeHintAnnotator<'i> {
    pub fn new(index: &'i Index, annotations: AnnotationMapImpl) -> TypeHintAnnotator<'i> {
        Self { index, annotations }
    }
}

impl AstVisitor for TypeHintAnnotator<'_> {
    fn visit_binary_expression(
        &mut self,
        stmt: &plc_ast::ast::BinaryExpression,
        node: &plc_ast::ast::AstNode,
    ) {
        stmt.walk(self);

        let l_type = self.annotations.get_type_or_void(&stmt.left, self.index);
        let r_type = self.annotations.get_type_or_void(&stmt.right, self.index);
        let stmt_type = self.annotations.get_type_or_void(node, self.index);

        if stmt.operator.is_arithmetic_operator()
            && stmt_type.is_arithmetic()
            && l_type.is_arithmetic()
            && r_type.is_arithmetic()
        {
            // hint left and right with the resulting type
            let type_name = stmt_type.get_name().to_string();
            self.annotations.annotate_type_hint(&stmt.left, StatementAnnotation::value(type_name.clone()));
            self.annotations.annotate_type_hint(&stmt.right, StatementAnnotation::value(type_name));
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
            if matches!(target.get_stmt(), AstStatement::Literal{..}) {
                self.annotations.copy_annotation(node, &target);
                self.annotations.clear_type_hint(&target);
            }
        }
    }

}
