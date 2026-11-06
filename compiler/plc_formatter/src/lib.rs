//! A lexer-based formatter for Structured Text source files, configurable via a
//! `.plcformat.yaml` file (see [`config::FormatConfig`]).
//!
//! The formatter never builds an AST: it drives `plc_lexer`'s `Token` lexer directly to get the
//! real token stream, recovers comments/blank-lines from the gaps between tokens (`trivia`),
//! derives indentation from a block-nesting stack machine (`layout`), and re-serializes the
//! result (`printer`). See `compiler/plc_formatter/src/layout.rs` for why a stack machine is
//! enough and no parse tree is needed.

pub mod cli;
pub mod config;
mod layout;
mod lex;
mod model;
mod printer;
mod trivia;

pub use config::FormatConfig;
pub use model::Document;

use std::path::Path;

use plc_diagnostics::diagnostics::Diagnostic;

pub struct FormatOutcome {
    pub formatted: String,
    pub changed: bool,
}

/// Formats `source`, or returns a [`Diagnostic`] if it cannot be formatted safely (a lex error,
/// or a structurally unbalanced block the layout engine cannot indent).
pub fn format_source(source: &str, config: &FormatConfig) -> Result<String, Diagnostic> {
    let raw_tokens = lex::lex_tokens(source).map_err(|err| {
        Diagnostic::new(format!("cannot format: invalid token at byte offset {}", err.offset))
            .with_error_code("E089")
    })?;
    let doc = trivia::build_document(source, raw_tokens);
    let decisions = layout::compute_layout(&doc.tokens).map_err(|err| match err {
        layout::LayoutError::UnmatchedCloser { index } => Diagnostic::new(format!(
            "cannot format: unbalanced block (unexpected closing keyword/bracket at token {index})"
        ))
        .with_error_code("E089"),
        layout::LayoutError::UnmatchedOpener => {
            Diagnostic::new("cannot format: unbalanced block (a block was never closed)".to_string())
                .with_error_code("E089")
        }
    })?;
    Ok(printer::print(&doc, &decisions, config))
}

pub fn format_file(path: &Path, config: &FormatConfig) -> Result<FormatOutcome, Diagnostic> {
    let original = std::fs::read_to_string(path).map_err(|err| {
        Diagnostic::new(format!("could not read {}: {err}", path.display())).with_error_code("E002")
    })?;
    let formatted = format_source(&original, config)?;
    Ok(FormatOutcome { changed: formatted != original, formatted })
}
