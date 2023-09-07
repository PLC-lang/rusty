use plc_ast::{
    ast::{AstFactory, AstStatement, ReferenceAccess},
    literals::AstLiteral,
};
use plc_source::source_location::SourceLocation;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
mod class_parser_tests;
mod container_parser_tests;
mod control_parser_tests;
mod expressions_parser_tests;
mod function_parser_tests;
mod initializer_parser_tests;
mod misc_parser_tests;
mod parse_errors;
mod parse_generics;
mod program_parser_tests;
mod statement_parser_tests;
mod type_parser_tests;
mod variable_parser_tests;

/// helper function to create references
pub fn ref_to(name: &str) -> AstStatement {
    AstStatement::ReferenceExpr {
        access: ReferenceAccess::Member(Box::new(AstFactory::create_identifier(
            name,
            &SourceLocation::undefined(),
            0,
        ))),
        base: None,
        id: 0,
        location: SourceLocation::undefined(),
    }
}

/// helper function to create literal ints
pub fn literal_int(value: i128) -> AstStatement {
    AstStatement::new_literal(AstLiteral::new_integer(value), 0, SourceLocation::undefined())
}

/// helper function to create empty statements
pub fn empty_stmt() -> AstStatement {
    AstStatement::EmptyStatement { location: SourceLocation::undefined(), id: 0 }
}
