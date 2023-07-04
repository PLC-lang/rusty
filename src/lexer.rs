// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use core::ops::Range;
use logos::Filter;
use logos::Lexer;
use logos::Logos;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
pub use tokens::Token;

use crate::ast::AstId;
use crate::ast::DirectAccessType;
use crate::ast::HardwareAccessType;
use crate::ast::SourceRange;
use crate::ast::SourceRangeFactory;
use crate::Diagnostic;

#[cfg(test)]
mod tests;
mod tokens;

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
    id_provider: IdProvider,
    pub source_range_factory: SourceRangeFactory,
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
        source_range_factory: SourceRangeFactory,
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

    /// this function will be removed soon:
    pub fn expect(&self, token: Token) -> Result<(), Diagnostic> {
        if self.token != token {
            Err(Diagnostic::unexpected_token_found(
                format!("{token:?}").as_str(),
                self.slice(),
                self.location(),
            ))
        } else {
            Ok(())
        }
    }

    /// Tries to consume the given token, returning false if it failed.
    pub fn try_consume(&mut self, token: &Token) -> bool {
        if self.token == *token {
            self.advance();
            return true;
        }

        false
    }

    pub fn consume_or_report(&mut self, token: Token) {
        if !self.try_consume(&token) {
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
            | Token::KeywordEndClass => {
                if !self.slice().to_string().contains('_') {
                    self.accept_diagnostic(Diagnostic::ImprovementSuggestion {
                        message: format!("the words in {} should be separated by a '_'", self.slice()),
                        range: vec![self.location()],
                    });
                }
            }
            _ => {}
        }
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn location(&self) -> SourceRange {
        self.source_range_factory.create_range(self.range())
    }

    pub fn last_location(&self) -> SourceRange {
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
        token == &Token::End || self.get_close_region_level(token).is_some()
    }

    pub fn recover_until_close(&mut self) {
        let mut hit = self.get_close_region_level(&self.token);
        let start = self.location();
        let mut end = self.location().get_end();
        while self.token != Token::End && hit.is_none() {
            end = self.location().get_end();
            self.advance();
            hit = self.closing_keywords.iter().rposition(|it| it.contains(&self.token));
        }

        //Did we recover in the while loop above?
        if start.get_end() != self.location().get_end() {
            let range = start.get_start()..end;
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
            'x' => Some(crate::ast::DirectAccessType::Bit),
            'b' => Some(crate::ast::DirectAccessType::Byte),
            'w' => Some(crate::ast::DirectAccessType::Word),
            'd' => Some(crate::ast::DirectAccessType::DWord),
            'l' => Some(crate::ast::DirectAccessType::LWord),
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
            'i' => Some(crate::ast::HardwareAccessType::Input),
            'q' => Some(crate::ast::HardwareAccessType::Output),
            'm' => Some(crate::ast::HardwareAccessType::Memory),
            'g' => Some(crate::ast::HardwareAccessType::Global),
            _ => None,
        })
        .expect("Unknown access type - tokenizer/grammar incomplete?");
    //Find the size from position 2
    let access = lexer
        .slice()
        .chars()
        .nth(2)
        .and_then(|c| match c.to_ascii_lowercase() {
            'x' => Some(crate::ast::DirectAccessType::Bit),
            'b' => Some(crate::ast::DirectAccessType::Byte),
            'w' => Some(crate::ast::DirectAccessType::Word),
            'd' => Some(crate::ast::DirectAccessType::DWord),
            'l' => Some(crate::ast::DirectAccessType::LWord),
            '*' => Some(crate::ast::DirectAccessType::Template),
            _ => None,
        })
        .expect("Unknown access type - tokenizer/grammar incomplete?");

    Some((hardware_type, access))
}

#[derive(Clone)]
//TODO: This belongs to the AST
pub struct IdProvider {
    current_id: Arc<AtomicUsize>,
}

impl IdProvider {
    pub fn next_id(&mut self) -> AstId {
        self.current_id.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for IdProvider {
    fn default() -> Self {
        IdProvider { current_id: Arc::new(AtomicUsize::new(1)) }
    }
}

#[cfg(test)]
pub fn lex(source: &str) -> ParseSession {
    ParseSession::new(Token::lexer(source), IdProvider::default(), SourceRangeFactory::internal())
}

pub fn lex_with_ids(
    source: &str,
    id_provider: IdProvider,
    location_factory: SourceRangeFactory,
) -> ParseSession {
    ParseSession::new(Token::lexer(source), id_provider, location_factory)
}

#[cfg(test)]
mod id_tests {
    use super::IdProvider;

    #[test]
    fn id_provider_generates_unique_ids_over_clones() {
        let mut id1 = IdProvider::default();

        assert_eq!(id1.next_id(), 1);

        let mut id2 = id1.clone();
        assert_eq!(id2.next_id(), 2);

        assert_eq!(id1.next_id(), 3);

        assert_eq!(id2.next_id(), 4);
    }
}
