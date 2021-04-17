// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
mod control_parser_tests;
mod expressions_parser_tests;
mod parser_tests;

pub fn lex(source: &str) -> crate::lexer::RustyLexer {
    crate::lexer::lex("", source)
}
