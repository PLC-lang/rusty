use plc_ast::{
    ast::{AstNode, CompilationUnit, Operator}, visitor::{AstVisitor, Walker}
};

use crate::{
    index::Index,
    typesystem::{get_bigger_type, BOOL_TYPE},
};

use super::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};

/// The inferring type annotator is used to infer the types of variables and expressions
/// based on their exising declaration-based type annotations.
///
/// It is able to infer following situations:
/// - The type of a binary expression based on the types of its operands
///     - e.g. `a + b` where `a` is of type `INT` and `b` is of type `DINT` will result in a type `DINT`
/// - The type of literals, when used on the right hand side of an assignment
///     - e.g. `a := 5` will annotate 5 with the a's type
/// - The elements of an array-literal, according to the inner type of the array
///     - e.g. `a := (1, 2, 3)` will annotate 1, 2 and 3 with the inner array-type of `a`
pub struct InferringTypeAnnotator<'i> {
    /// The annotations map. Records annotations for nodes
    annotations: AnnotationMapImpl,
    /// the index to lookup names
    index: &'i Index,
}

impl InferringTypeAnnotator<'_> {
    pub fn new(index: &Index) -> InferringTypeAnnotator {
        InferringTypeAnnotator { annotations: AnnotationMapImpl::new(), index }
    }

    pub fn into_annotations(self) -> AnnotationMapImpl {
        self.annotations
    }
}

impl InferringTypeAnnotator<'_> {
    pub fn visit_unit(
        unit: &CompilationUnit,
        index: &Index,
        annotations: AnnotationMapImpl,
    ) -> AnnotationMapImpl {
        let mut annotator = InferringTypeAnnotator { annotations, index };
        annotator.visit_compilation_unit(unit);
        annotator.annotations
    }
}

impl<'i> AstVisitor for InferringTypeAnnotator<'i> {
    fn visit_paren_expression(&mut self, inner: &AstNode, node: &AstNode) {
        inner.walk(self);
        self.annotations.copy_annotation(inner, node);
    }

    fn visit_binary_expression(&mut self, stmt: &plc_ast::ast::BinaryExpression, node: &AstNode) {
        stmt.walk(self);
        match stmt.operator {
            Operator::Plus | Operator::Minus | Operator::Multiplication | Operator::Division |
            Operator::Less | Operator::LessOrEqual | Operator::Equal | Operator::NotEqual |
            Operator::Greater | Operator::GreaterOrEqual
             => {
                // if let Some(expected_type) = self.annotations.get_type_hint(node, &self.index) {
                let types = self
                    .annotations
                    .get_type(&stmt.left, &self.index)
                    .zip(self.annotations.get_type(&stmt.right, &self.index));

                let target_type = types.map(|(left_type, right_type)| {
                    get_bigger_type(
                        left_type.get_type_information(),
                        right_type.get_type_information(),
                        &self.index,
                    )
                    .get_name()
                    .to_string()
                });

                if let Some(target_type) = target_type {
                    self.annotations.annotate_type_hint(&stmt.left, StatementAnnotation::value(target_type.clone()));
                    self.annotations.annotate_type_hint(&stmt.right, StatementAnnotation::value(target_type.clone()));

                    if !stmt.operator.is_comparison_operator() {
                        self.annotations.annotate(node, StatementAnnotation::value(target_type));
                    }
                }

                if stmt.operator.is_comparison_operator() {
                    self.annotations.annotate(node, StatementAnnotation::value(BOOL_TYPE));
                }
                // }
            }
            Operator::Exponentiation => todo!(),
            
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use plc_ast::ast::CompilationUnit;
    use plc_diagnostics::diagnostics::Diagnostic;

    use crate::{
        index::Index,
        resolver::{
            annotation_printer::AnnotationPrinter, declared_type_annotator::DeclaredTypeAnnotator,
            inferring_type_annotator::InferringTypeAnnotator, AnnotationMapImpl,
        },
        test_utils::tests::index_safe,
    };

    fn annotate(src: &str, allow_diagnostics: Vec<&str>) -> (CompilationUnit, AnnotationMapImpl, Index) {
        let (unit, index, diagnostics) = index_safe(src);

        let diagnostics: Vec<Diagnostic> = diagnostics
            .into_iter()
            .filter(|d| !allow_diagnostics.iter().any(|it| d.get_error_code().eq_ignore_ascii_case(it)))
            .collect();

        assert_eq!(diagnostics, vec![]); // make sure there are no

        let a = DeclaredTypeAnnotator::visit_unit(&unit, &index);
        let a = InferringTypeAnnotator::visit_unit(&unit, &index, a);
        (unit, a, index)
    }

    macro_rules! snapshot {
        ($src:expr) => {
            let (unit, annotations, _) = annotate($src, vec![]);
            insta::assert_snapshot!(AnnotationPrinter::print($src, &annotations, &unit));
        };
        ($src:expr, $allow_diagnostics:expr) => {
            let (unit, annotations, _) = annotate($src, $allow_diagnostics);
            insta::assert_snapshot!(AnnotationPrinter::print($src, &annotations, &unit));
        };
    }

    #[test]
    fn compare_operations() {
        snapshot!(
            "
        PROGRAM PRG
            VAR a : INT; b : BYTE; END_VAR
            a < b;
            a <= b;
            a = b;
            a <> b;
            a > b;
            a >= b;
            (a = b) = (b = a);
        END_PROGRAM"
        );
    }

    #[test]
    fn binary_operations() {
        snapshot!(
            "
        PROGRAM PRG
            VAR a : INT; b : BYTE; END_VAR
            a + b;
            a - b;
            a * b;
            a / b;
            (a + b) - (b + b);
        END_PROGRAM"
        );
    }

}
