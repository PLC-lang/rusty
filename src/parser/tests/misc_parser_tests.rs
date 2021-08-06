// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::{
    ast::*,
    parser::{parse, tests::lex},
};
use pretty_assertions::*;

#[test]
fn empty_returns_empty_compilation_unit() {
    let (result, ..) = parse(lex(""));
    assert_eq!(result.units.len(), 0);
}

#[test]
fn programs_can_be_external() {
    let lexer = lex("@EXTERNAL PROGRAM foo END_PROGRAM");
    let parse_result = parse(lexer).0;
    let implementation = &parse_result.implementations[0];
    assert_eq!(LinkageType::External, implementation.linkage);
}
