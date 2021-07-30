use crate::ast::{SourceRange, Statement};

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
mod container_parser_tests;
mod control_parser_tests;
mod expressions_parser_tests;
mod function_parser_tests;
mod initializer_parser_tests;
mod misc_parser_tests;
mod parse_errors;
mod program_parser_tests;
mod statement_parser_tests;
mod type_parser_tests;
mod variable_parser_tests;

pub fn lex(source: &str) -> crate::lexer::ParseSession {
    crate::lexer::lex(source)
}

/// helper function to create references
pub fn ref_to(name: &str) -> Statement {
    Statement::Reference {
        location: SourceRange::undefined(),
        name: name.to_string(),
        id: 0,
    }
}

/// helper function to create empty statements
pub fn empty_stmt() -> Statement {
    Statement::EmptyStatement {
        location: SourceRange::undefined(),
        id: 0,
    }
}
