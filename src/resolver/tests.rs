use crate::{
    ast::{self, CompilationUnit},
    index::{self, Index},
    lexer::{lex_with_ids, IdProvider},
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
    let ids = IdProvider::default();
    let (mut unit, _) = crate::parser::parse(lex_with_ids(src, ids.clone()));

    ast::pre_process(&mut unit);
    let index = index::visitor::visit(&unit, ids.clone());
    (unit, index)
}

fn annotate(parse_result: &CompilationUnit, index: &Index) -> AnnotationMap {
    TypeAnnotator::visit_unit(index, parse_result)
}
