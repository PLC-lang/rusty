// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::Validator;
use crate::{Diagnostic, ast, index::{self, Index}, lexer::{IdProvider, lex, lex_with_ids}, parser::parse, resolver::{self, TypeAnnotator}};

mod bitaccess_validation_test;
mod literals_validation_tests;
mod reference_resolve_tests;
mod variable_validation_tests;

mod statement_validation_tests;

pub fn parse_and_validate(src: &str) -> Vec<Diagnostic> {
    let mut idx = Index::new();
    let ids = IdProvider::new();
    let (mut ast, _) = parse(lex_with_ids(src, ids.clone()));
    ast::pre_process(&mut ast);
    idx.import(index::visitor::visit(&ast, ids));

    let (idx, _) = resolver::const_evaluator::evaluate_constants(idx);

    let annotations = TypeAnnotator::visit_unit(&idx, &ast);

    let mut validator = Validator::new();
    validator.visit_unit(&annotations, &idx, &ast);
    validator.diagnostics()
}
