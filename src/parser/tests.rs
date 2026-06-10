// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use plc_ast::{
    ast::{AstFactory, AstNode},
    literals::AstLiteral,
};
use plc_source::source_location::SourceLocation;

mod class_parser_tests;
mod function_parser_tests;
mod interface_parser_tests;
mod parse_errors;

/// helper function to create references
pub fn ref_to(name: &str) -> AstNode {
    AstFactory::create_member_reference(
        AstFactory::create_identifier(name, SourceLocation::internal(), 0),
        None,
        0,
    )
}

/// helper function to create literal ints
pub fn literal_int(value: i128) -> AstNode {
    AstNode::new_literal(AstLiteral::new_integer(value), 0, SourceLocation::internal())
}
