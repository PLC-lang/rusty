// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::Validator;
use crate::{
    ast,
    index::{self, Index},
    lexer::lex,
    parser::parse,
    resolver::TypeAnnotator,
    Diagnostic,
};

mod literals_validation_tests;
mod reference_resolve_tests;

mod bitaccess_validation_test;

pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
    let mut idx = Index::new();
    let (mut ast, _) = parse(lex(src));
    ast::pre_process(&mut ast);
    idx.import(index::visitor::visit(&ast));

    let annotations = TypeAnnotator::visit_unit(&idx, &ast);

    let mut validator = Validator::new();
    validator.visit_unit(&annotations, &idx, &ast);
    validator.diagnostics()
}
