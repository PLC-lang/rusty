use crate::{
    ast::{self, CompilationUnit},
    index::{self, Index},
    lexer::lex,
};

use super::{AnnotationMap, TypeAnnotator};

#[cfg(test)]
mod const_resolver_tests;
#[cfg(test)]
mod resolve_control_statments;
#[cfg(test)]
mod resolve_expressions_tests;
#[cfg(test)]
mod resolve_literals_tests;

fn parse(src: &str) -> (CompilationUnit, Index) {
    let (mut unit, _) = crate::parser::parse(lex(src));

    ast::pre_process(&mut unit);
    let index = index::visitor::visit(&unit);
    (unit, index)
}

fn annotate(parse_result: &CompilationUnit, index: &Index) -> AnnotationMap {
    TypeAnnotator::visit_unit(index, parse_result)
}
