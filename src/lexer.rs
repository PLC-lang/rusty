// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use core::ops::Range;
use logos::{Filter, Lexer, Logos};
use plc_ast::ast::{AstId, DirectAccessType, HardwareAccessType};
use plc_ast::provider::IdProvider;
use plc_diagnostics::diagnostics::Diagnostic;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
pub use tokens::Token;

#[cfg(test)]
mod tests;
mod tokens;

/// Categories of source-text bytes that the regular `Token` lexer skips.
/// Surfaced separately by [`lex_with_trivia`] for cursor-aware LSP walks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriviaKind {
    Whitespace,
    LineComment,
    /// Covers both `(* ... *)` (ST) and `/* ... */` (C-style); they nest within
    /// their own delimiters but not across each other, matching `parse_comments`.
    BlockComment,
    /// Unknown `{ ... }` pragma. Recognised pragmas (e.g. `{external}`) stay as
    /// real tokens; this catches the catch-all `\{` arm that parse_pragma skips.
    Pragma,
}

/// A single lexer-produced span: either a real `Token` or skipped trivia.
/// `Range<usize>` is a byte range into the original source.
#[derive(Debug, Clone)]
pub enum LspToken {
    Token(Token, Range<usize>),
    Trivia(TriviaKind, Range<usize>),
}

impl LspToken {
    pub fn range(&self) -> &Range<usize> {
        match self {
            LspToken::Token(_, r) | LspToken::Trivia(_, r) => r,
        }
    }
}

/// Lex `source` into a contiguous stream of real tokens plus the trivia
/// between them. Strategy: run the regular `Token::lexer` and, in each gap
/// between adjacent real-token spans, classify the skipped bytes as
/// whitespace / line comment / block comment / pragma. The output covers
/// `0..source.len()` with no gaps, which is the invariant downstream
/// binary-search walks rely on.
pub fn lex_with_trivia(source: &str) -> Vec<LspToken> {
    let mut out = Vec::new();
    let mut lexer = Token::lexer(source);
    let mut cursor = 0usize;
    while let Some(tok) = lexer.next() {
        let span = lexer.span();
        if cursor < span.start {
            fill_trivia(source, cursor, span.start, &mut out);
        }
        out.push(LspToken::Token(tok, span.clone()));
        cursor = span.end;
    }
    if cursor < source.len() {
        fill_trivia(source, cursor, source.len(), &mut out);
    }
    out
}

/// Classify bytes in `source[from..to]` (a gap between two real tokens)
/// into trivia variants.
fn fill_trivia(source: &str, from: usize, to: usize, out: &mut Vec<LspToken>) {
    let bytes = source.as_bytes();
    let mut i = from;
    while i < to {
        let start = i;
        let b = bytes[i];
        let b1 = bytes.get(i + 1).copied();

        if b == b'/' && b1 == Some(b'/') {
            // Line comment: `//` to newline (or EOF)
            while i < to && bytes[i] != b'\n' {
                i += 1;
            }
            out.push(LspToken::Trivia(TriviaKind::LineComment, start..i));
        } else if (b == b'(' && b1 == Some(b'*')) || (b == b'/' && b1 == Some(b'*')) {
            // Block comment. Track nesting in the style that opened it.
            let (open_a, open_b, close_a, close_b) =
                if b == b'(' { (b'(', b'*', b'*', b')') } else { (b'/', b'*', b'*', b'/') };
            i += 2;
            let mut depth = 1u32;
            while i < to && depth > 0 {
                if i + 1 < to && bytes[i] == open_a && bytes[i + 1] == open_b {
                    depth += 1;
                    i += 2;
                } else if i + 1 < to && bytes[i] == close_a && bytes[i + 1] == close_b {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            out.push(LspToken::Trivia(TriviaKind::BlockComment, start..i));
        } else if b == b'{' {
            // Catch-all pragma: skip to `}` (inclusive) or EOF.
            while i < to && bytes[i] != b'}' {
                i += 1;
            }
            if i < to {
                i += 1;
            }
            out.push(LspToken::Trivia(TriviaKind::Pragma, start..i));
        } else if b.is_ascii_whitespace() {
            while i < to && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push(LspToken::Trivia(TriviaKind::Whitespace, start..i));
        } else {
            // Anything else in a gap is unexpected; treat as whitespace one byte
            // at a time so binary search over spans stays well-formed.
            i += 1;
            out.push(LspToken::Trivia(TriviaKind::Whitespace, start..i));
        }
    }
}

pub struct ParseSession<'a> {
    lexer: Lexer<'a, Token>,
    pub token: Token,
    pub diagnostics: Vec<Diagnostic>,
    pub closing_keywords: Vec<Vec<Token>>,
    /// the token parsed before the current one stored in `token`
    pub last_token: Token,
    /// the range of the `last_token`
    pub last_range: Range<usize>,
    pub parse_progress: usize,
    pub id_provider: IdProvider,
    pub source_range_factory: SourceLocationFactory,
    pub scope: Option<String>,
}

#[macro_export]
macro_rules! expect_token {
    ($lexer:expr, $token:expr, $return_value:expr) => {
        if $lexer.token != $token {
            $lexer.accept_diagnostic(Diagnostic::unexpected_token_found(
                format!("{:?}", $token).as_str(),
                $lexer.slice(),
                $lexer.location(),
            ));
            return $return_value;
        }
    };
}

impl<'a> ParseSession<'a> {
    pub fn new(
        l: Lexer<'a, Token>,
        id_provider: IdProvider,
        source_range_factory: SourceLocationFactory,
    ) -> ParseSession<'a> {
        let mut lexer = ParseSession {
            lexer: l,
            token: Token::KeywordBy,
            diagnostics: vec![],
            closing_keywords: vec![],
            last_token: Token::End,
            last_range: 0..0,
            parse_progress: 0,
            id_provider,
            scope: None,
            source_range_factory,
        };
        lexer.advance();
        lexer
    }

    pub fn get_src(&self) -> &str {
        self.lexer.source()
    }

    pub fn next_id(&mut self) -> AstId {
        self.id_provider.next_id()
    }

    /// Tries to consume the given token, returning false if it failed.
    pub fn try_consume(&mut self, token: Token) -> bool {
        if self.token == token {
            self.advance();
            return true;
        }

        false
    }

    pub fn try_consume_or_report(&mut self, token: Token) {
        if !self.try_consume(token) {
            self.accept_diagnostic(Diagnostic::missing_token(format!("{token:?}").as_str(), self.location()));
        }
    }

    pub fn slice_and_advance(&mut self) -> String {
        let slice = self.slice().to_string();
        self.advance();
        slice
    }

    pub fn is_end_of_stream(&self) -> bool {
        self.token == Token::End || self.token == Token::Error
    }

    pub fn slice_region(&self, range: Range<usize>) -> &str {
        &self.lexer.source()[range]
    }

    pub fn advance(&mut self) {
        self.last_range = self.range();
        self.last_token = std::mem::replace(&mut self.token, self.lexer.next().unwrap_or(Token::End));
        self.parse_progress += 1;

        match self.token {
            Token::KeywordVarInput
            | Token::KeywordVarOutput
            | Token::KeywordVarGlobal
            | Token::KeywordVarInOut
            | Token::KeywordRef
            | Token::KeywordVarTemp
            | Token::KeywordNonRetain
            | Token::KeywordEndVar
            | Token::KeywordEndProgram
            | Token::KeywordEndFunction
            | Token::KeywordEndCase
            | Token::KeywordFunctionBlock
            | Token::KeywordEndFunctionBlock
            | Token::KeywordEndStruct
            | Token::KeywordEndAction
            | Token::KeywordEndActions
            | Token::KeywordEndIf
            | Token::KeywordEndFor
            | Token::KeywordEndRepeat
            | Token::KeywordEndMethod
            | Token::KeywordEndClass
                if !self.slice().to_string().contains('_') =>
            {
                self.accept_diagnostic(
                    Diagnostic::new(format!("the words in {} should be separated by a `_`", self.slice()))
                        .with_error_code("E013")
                        .with_location(self.location()),
                );
            }
            _ => {}
        }
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn location(&self) -> SourceLocation {
        self.source_range_factory.create_range(self.range())
    }

    pub fn last_location(&self) -> SourceLocation {
        self.source_range_factory.create_range(self.last_range.clone())
    }

    pub fn range(&self) -> Range<usize> {
        self.lexer.span()
    }

    pub fn accept_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn enter_region(&mut self, end_token: Vec<Token>) {
        self.closing_keywords.push(end_token);
    }

    pub fn close_region(&mut self) {
        if let Some(expected_token) = self.closing_keywords.pop() {
            if !expected_token.contains(&self.token) {
                self.accept_diagnostic(Diagnostic::unexpected_token_found(
                    format!("{:?}", expected_token[0]).as_str(),
                    format!("'{}'", self.slice()).as_str(),
                    self.location(),
                ));
            } else {
                self.advance();
            }
        }
    }

    /// returns the level (which corresponds to the position on the `closing_keywords` stack)
    /// returns `None` if this token does not close an open region
    fn get_close_region_level(&self, token: &Token) -> Option<usize> {
        self.closing_keywords.iter().rposition(|it| it.contains(token))
    }

    /// returns true if the given token closes an open region
    pub fn closes_open_region(&self, token: &Token) -> bool {
        token == &Token::End || self.get_close_region_level(token).is_some() || is_top_level_sync_token(token)
    }

    pub fn recover_until_close(&mut self) {
        let mut hit = self.get_close_region_level(&self.token);
        let start = self.range();
        let mut end = self.range().end;
        while self.token != Token::End && hit.is_none() && !is_top_level_sync_token(&self.token) {
            end = self.range().end;
            self.advance();
            hit = self.closing_keywords.iter().rposition(|it| it.contains(&self.token));
        }

        //Did we recover in the while loop above?
        if start.end != self.range().end {
            let range = start.start..end;
            self.accept_diagnostic(Diagnostic::unexpected_token_found(
                format!(
                    "{:?}",
                    self.closing_keywords.last().and_then(|it| it.first()).unwrap_or(&Token::End) //only show first expected token
                )
                .as_str(),
                format!("'{}'", self.slice_region(range.clone())).as_str(),
                self.source_range_factory.create_range(range),
            ));
        }

        if let Some(hit) = hit {
            if self.closing_keywords.len() > hit + 1 {
                let closing = self
                    .closing_keywords
                    .last()
                    .expect("parse-recovery has no closing-keyword to recover from."); //illegal state! invalid use of parser-recovery?
                let expected_tokens = format!("{closing:?}");
                self.accept_diagnostic(Diagnostic::missing_token(expected_tokens.as_str(), self.location()));
            }
        }
    }
}

/// Returns true if `token` is a "synchronisation" keyword — one that, when
/// encountered inside a body or nested region where it shouldn't appear,
/// signals that the previous declaration was unclosed and the parser should
/// bail out of the current region. The top-level dispatcher then picks up
/// the new declaration cleanly. Without this, the parser would treat the
/// keyword as a stray identifier-like token and swallow the next declaration
/// as garbage (the H28/H29 pattern surfaced by the lenient_completion_probe).
///
/// Includes top-level POU keywords, `ACTION` / `ACTIONS` (both can appear at
/// top level), and `METHOD` / `PROPERTY_GET` / `PROPERTY_SET` (sibling
/// declarations inside an FB / CLASS body).
fn is_top_level_sync_token(token: &Token) -> bool {
    matches!(
        token,
        Token::KeywordFunction
            | Token::KeywordFunctionBlock
            | Token::KeywordProgram
            | Token::KeywordClass
            | Token::KeywordType
            | Token::KeywordVarGlobal
            | Token::KeywordInterface
            | Token::KeywordAction
            | Token::KeywordActions
            | Token::KeywordMethod
            | Token::KeywordPropertyGet
            | Token::KeywordPropertySet
    )
}

fn parse_pragma(lexer: &mut Lexer<Token>) -> Filter<()> {
    let remainder = lexer.remainder();
    let chars = remainder.chars();
    let mut traversed = 0;
    for c in chars {
        traversed += c.len_utf8();
        if c == '}' {
            lexer.bump(traversed);
            return Filter::Skip;
        }
    }
    Filter::Emit(())
}

fn parse_comments(lexer: &mut Lexer<Token>) -> Filter<()> {
    let (open, close) = get_closing_tag(lexer.slice());
    let remainder = lexer.remainder();
    let mut unclosed = 1;
    let chars = remainder.chars();

    let mut prev = ' ';
    let mut traversed = 0;
    for c in chars {
        if c == '*' && prev == open {
            unclosed += 1;
            //Make sure the next action does not consume the star
            prev = ' ';
        } else if c == close && prev == '*' {
            unclosed -= 1;
            prev = c;
        } else {
            prev = c;
        }
        traversed += c.len_utf8();
        if unclosed == 0 {
            lexer.bump(traversed);
            //This is a well formed comment, treat it as whitespace
            return Filter::Skip;
        }
    }
    Filter::Emit(())
}

fn get_closing_tag(open_tag: &str) -> (char, char) {
    match open_tag {
        "(*" => ('(', ')'),
        "/*" => ('/', '/'),
        _ => unreachable!(),
    }
}
fn parse_access_type(lexer: &mut Lexer<Token>) -> Option<DirectAccessType> {
    //Percent is at position 0
    //Find the size from position 1
    let access = lexer
        .slice()
        .chars()
        .nth(1)
        .and_then(|c| match c.to_ascii_lowercase() {
            'x' => Some(DirectAccessType::Bit),
            'b' => Some(DirectAccessType::Byte),
            'w' => Some(DirectAccessType::Word),
            'd' => Some(DirectAccessType::DWord),
            'l' => Some(DirectAccessType::LWord),
            _ => None,
        })
        .expect("Unknown access type - tokenizer/grammar incomplete?");

    Some(access)
}

fn parse_hardware_access_type(lexer: &mut Lexer<Token>) -> Option<(HardwareAccessType, DirectAccessType)> {
    //Percent is at position 0
    let hardware_type = lexer
        .slice()
        .chars()
        .nth(1)
        .and_then(|c| match c.to_ascii_lowercase() {
            'i' => Some(HardwareAccessType::Input),
            'q' => Some(HardwareAccessType::Output),
            'm' => Some(HardwareAccessType::Memory),
            'g' => Some(HardwareAccessType::Global),
            _ => None,
        })
        .expect("Unknown access type - tokenizer/grammar incomplete?");
    //Find the size from position 2
    let access = lexer
        .slice()
        .chars()
        .nth(2)
        .and_then(|c| match c.to_ascii_lowercase() {
            'x' => Some(DirectAccessType::Bit),
            'b' => Some(DirectAccessType::Byte),
            'w' => Some(DirectAccessType::Word),
            'd' => Some(DirectAccessType::DWord),
            'l' => Some(DirectAccessType::LWord),
            '*' => Some(DirectAccessType::Template),
            _ => None,
        })
        .expect("Unknown access type - tokenizer/grammar incomplete?");

    Some((hardware_type, access))
}

#[cfg(test)]
pub fn lex(source: &str) -> ParseSession<'_> {
    ParseSession::new(Token::lexer(source), IdProvider::default(), SourceLocationFactory::internal(source))
}

pub fn lex_with_ids(
    source: &str,
    id_provider: IdProvider,
    location_factory: SourceLocationFactory,
) -> ParseSession<'_> {
    ParseSession::new(Token::lexer(source), id_provider, location_factory)
}
