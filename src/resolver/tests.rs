use crate::{ast::{self, CompilationUnit}, index::{self, Index}, lexer::lex};

use super::{AnnotationMap, TypeAnnotator};


#[cfg(test)]
mod resolve_literals_tests;
mod resolve_expressions_tests;


fn parse(src: &str) -> (CompilationUnit, Index) {
    let mut unit = crate::parser::parse(lex(src)).unwrap().0;
    ast::pre_process(&mut unit);
    let index = index::visitor::visit(&unit);
    (unit, index)
}

fn annotate(parse_result: &CompilationUnit, index: Index) -> AnnotationMap {
    let mut annotator = TypeAnnotator::new(&index);
    annotator.visit_unit(parse_result)
}