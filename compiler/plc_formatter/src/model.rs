//! The token+trivia stream the layout engine and printer operate on.

use std::ops::Range;

use plc_lexer::Token;

// Trivia
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trivia {
    LineComment(String),
    BlockComment(String),
    /// An unrecognized `{ ... }` block. Recognized pragmas (`{external}`, `{ref}`, ...) are real
    /// tokens and never appear here.
    Pragma(String),
}

impl Trivia {
    pub fn text(&self) -> &str {
        match self {
            Trivia::LineComment(s) | Trivia::BlockComment(s) | Trivia::Pragma(s) => s,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeadingTrivia {
    /// Blank source lines between the previous item and this comment.
    pub blank_lines_before: u32,
    pub comment: Trivia,
}

// Token + trivia model
#[derive(Debug, Clone)]
pub struct LexedToken {
    pub token: Token,
    pub text: String,
    pub range: Range<usize>,
    /// Comments on the same source line, immediately after this token.
    pub trailing_trivia: Vec<Trivia>,
    /// Comments on their own line(s), between the previous token and this one.
    pub leading_trivia: Vec<LeadingTrivia>,
    /// Blank source lines immediately before this token (after any leading comments).
    pub blank_lines_before: u32,
    /// Whether at least one newline appeared anywhere in the source between the previous
    /// token and this one. Used outside of structurally-forced break points, where the
    /// MVP printer preserves the user's existing line breaks rather than reflowing them.
    pub had_newline_before: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub tokens: Vec<LexedToken>,
    /// Comments after the last real token, before end-of-file. Any blank lines at the very end
    /// of the file (with no trailing comment) are intentionally trimmed by the printer, which
    /// always normalizes end-of-file to a single trailing newline.
    pub trailing_trivia: Vec<LeadingTrivia>,
}
