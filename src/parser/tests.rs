use crate::ast::{SourceRange, Statement};

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
mod control_parser_tests;
mod expressions_parser_tests;
mod parse_errors;
mod parser_tests;

pub fn lex(source: &str) -> crate::lexer::ParseSession {
    crate::lexer::lex("", source)
}

/// helper function to create references
pub fn ref_to(name: &str) -> Statement {
    Statement::Reference {
        location: SourceRange::undefined(),
        name: name.to_string(),
    }
}
