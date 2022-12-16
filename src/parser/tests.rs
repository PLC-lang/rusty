use crate::ast::{AstStatement, SourceRange};

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
    AstStatement::Reference { location: SourceRange::undefined(), name: name.to_string(), id: 0 }
}

/// helper function to create literal ints
pub fn literal_int(value: i128) -> AstStatement {
    AstStatement::LiteralInteger { value, location: SourceRange::undefined(), id: 0 }
}

/// helper function to create empty statements
pub fn empty_stmt() -> AstStatement {
    AstStatement::EmptyStatement { location: SourceRange::undefined(), id: 0 }
}
