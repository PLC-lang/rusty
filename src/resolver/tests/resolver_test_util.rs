use plc_ast::ast::AstId;
use plc_ast::provider::IdProvider;
use plc_ast::visitor::{AstVisitor, Walker};
use plc_source::SourceCode;
use std::collections::HashSet;
use std::ops::Range;

use crate::index::Index;
use crate::resolver::{AnnotationMap, AnnotationMapImpl, StatementAnnotation};
use crate::{resolver::TypeAnnotator, test_utils::tests::index_with_ids};

pub fn extract_markers(src: &str) -> (String, Vec<Range<usize>>) {
    let mut clean_src = String::new();
    let mut markers = Vec::new();

    let mut open_markers = Vec::new();

    for (i, c) in src.chars().enumerate() {
        match c {
            '{' => {
                open_markers.push(clean_src.len());
            }
            '}' => {
                if let Some(start) = open_markers.pop() {
                    let end = clean_src.len();
                    markers.push(start..end);
                } else {
                    panic!("Unmatched closing marker '}}' in source code at index {i}.");
                }
            }
            _ => {
                clean_src.push(c);
            }
        }
    }

    if open_markers.len() > 0 {
        panic!("Unmatched opening marker '{{' in source code at indexes {open_markers:?}.");
    }

    (clean_src, markers)
}

fn display_annotation(annotation: &StatementAnnotation, annotation_map: &AnnotationMapImpl, index: &Index) -> String {
    if let Some(ty) = annotation_map.get_type_for_annotation(index, annotation) {
        format!("{}", ty.get_name())
    } else {
        "<Unknown Type>".to_string()
    }
}



pub fn test_resolve<T: Into<SourceCode>>(src: T) -> String {
    let id_provider = IdProvider::default();

    let (clean_src, markers) = extract_markers(src.into().source.as_str());
    dbg!(&markers);

    let (unit, index) = index_with_ids(clean_src.as_str(), id_provider.clone());
    let (annotations, ..) = TypeAnnotator::visit_unit(&index, &unit, id_provider);

    let mut collector = ExpressionCollector::new(&markers);
    collector.visit_compilation_unit(&unit);

    collector
        .expressions
        .iter()
        .map(|(r, ast_id)| {
            let ty = annotations.get_with_id(*ast_id);
            let hint = annotations.get_hint_with_id(*ast_id);
            format!(
                "{}\n      [{} | {}]",
                &clean_src[r.start..r.end],
                ty.map(|t| display_annotation(t, &annotations, &index)).unwrap_or("<unknown>".to_string()),
                hint.map(|t| display_annotation(t, &annotations, &index)).unwrap_or("-".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub struct ExpressionCollector {
    pub expressions: Vec<(Range<usize>, AstId)>,
    pub ordered_markers: HashSet<Range<usize>>,
}

impl ExpressionCollector {
    pub fn new(markers: &[Range<usize>]) -> Self {
        Self { expressions: Vec::new(), ordered_markers: markers.iter().cloned().collect() }
    }
}

impl AstVisitor for ExpressionCollector {
    fn visit(&mut self, node: &plc_ast::ast::AstNode) {
        let span = dbg!(node).get_location().to_range();
        let current_range = span.map(|it| it.clone()).unwrap_or_else(|| (0..0));

        if self.ordered_markers.remove(&current_range) {
            self.expressions.push((current_range, node.get_id()))
        }

        node.walk(self);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_collector() {
        let src = "
        FUNCTION foo
            VAR
                x : INT;
                y : DINT;
            END_VAR
           {{x} + {y}}  // { ... } surrounded expressions to be evaluated
        END_FUNCTION
        ";

        let resolves = test_resolve(src);
        assert_snapshot!(resolves, @r"
        x + y
              [DINT | -]
        x
              [INT | DINT]
        y
              [DINT | -]
        ");
    }
}
