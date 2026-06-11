// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

//! Lexer and parser for IEC 61131-3 Structured Text.
//!
//! This crate turns Structured Text source into the AST defined in [`plc_ast`],
//! reporting syntax problems as [`plc_diagnostics`] diagnostics.

pub mod lexer;
pub mod parser;

#[cfg(test)]
mod test_utils;
