// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use core::ops::Range;
use logos::Filter;
use logos::Lexer;
use logos::Logos;

use crate::ast::SourceRange;
use crate::Diagnostic;

#[cfg(test)]
mod tests;

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
}

impl<'a> ParseSession<'a> {
    pub fn new(l: Lexer<'a, Token>) -> ParseSession<'a> {
        let mut lexer = ParseSession {
            lexer: l,
            token: Token::KeywordBy,
            diagnostics: vec![],
            closing_keywords: vec![],
            last_token: Token::End,
            last_range: 0..0,
            parse_progress: 0,
        };
        lexer.advance();
        lexer
    }

    /// consumes an optional token and returns true if it was consumed.
    pub fn allow(&mut self, token: &Token) -> bool {
        if self.token == *token {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn consume_or_report(&mut self, token: Token) {
        if !self.allow(&token) {
            self.accept_diagnostic(Diagnostic::missing_token(
                format!("{:?}", token),
                self.location(),
            ));
        }
    }

    pub fn slice_region(&self, range: Range<usize>) -> &str {
        &self.lexer.source()[range]
    }

    pub fn advance(&mut self) {
        self.last_range = self.range();
        self.last_token =
            std::mem::replace(&mut self.token, self.lexer.next().unwrap_or(Token::End));
        self.parse_progress += 1;
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn location(&self) -> SourceRange {
        SourceRange::new(self.range())
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
                    format!("{:?}", expected_token[0]),
                    format!("'{}'", self.slice()),
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
        self.closing_keywords
            .iter()
            .rposition(|it| it.contains(token))
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
            hit = self
                .closing_keywords
                .iter()
                .rposition(|it| it.contains(&self.token));
        }

        //Did we recover in the while loop above?
        if start.get_end() != self.location().get_end() {
            let range = start.get_start()..end;
            self.accept_diagnostic(Diagnostic::unexpected_token_found(
                format!(
                    "{:?}",
                    self.closing_keywords
                        .last()
                        .and_then(|it| it.first())
                        .unwrap_or(&Token::End) //only show first expected token
                ),
                format!("'{}'", self.slice_region(range.clone())),
                SourceRange::new(range),
            ));
        }

        if let Some(hit) = hit {
            if self.closing_keywords.len() > hit + 1 {
                let closing = self.closing_keywords.last().unwrap();
                let expected_tokens = format!("{:?}", closing);
                self.accept_diagnostic(Diagnostic::missing_token(expected_tokens, self.location()));
            }
        }
    }
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

#[derive(Debug, PartialEq, Logos, Clone)]
pub enum Token {
    #[error]
    #[regex(r"\(\*", |lex| parse_comments(lex))]
    #[regex(r"/\*", |lex| parse_comments(lex))]
    #[regex(r"//.*", logos::skip)]
    #[regex(r"(?m)\r", logos::skip)]
    Error,

    #[token("@EXTERNAL")]
    PropertyExternal,

    #[token("PROGRAM")]
    KeywordProgram,

    #[token("VAR_INPUT")]
    KeywordVarInput,

    #[token("VAR_OUTPUT")]
    KeywordVarOutput,

    #[token("VAR")]
    KeywordVar,

    #[token("VAR_GLOBAL")]
    KeywordVarGlobal,

    #[token("VAR_IN_OUT")]
    KeywordVarInOut,

    #[token("END_VAR")]
    KeywordEndVar,

    #[token("END_PROGRAM")]
    KeywordEndProgram,

    #[token("FUNCTION")]
    KeywordFunction,

    #[token("END_FUNCTION")]
    KeywordEndFunction,

    #[token("FUNCTION_BLOCK")]
    KeywordFunctionBlock,

    #[token("END_FUNCTION_BLOCK")]
    KeywordEndFunctionBlock,

    #[token("TYPE")]
    KeywordType,

    #[token("STRUCT")]
    KeywordStruct,

    #[token("END_TYPE")]
    KeywordEndType,

    #[token("END_STRUCT")]
    KeywordEndStruct,

    #[token("ACTIONS")]
    KeywordActions,

    #[token("ACTION")]
    KeywordAction,

    #[token("END_ACTION")]
    KeywordEndAction,

    #[token("END_ACTIONS")]
    KeywordEndActions,

    #[token(":")]
    KeywordColon,

    #[token(";")]
    KeywordSemicolon,

    #[token(":=")]
    KeywordAssignment,

    #[token("=>")]
    KeywordOutputAssignment,

    #[token("(")]
    KeywordParensOpen,

    #[token(")")]
    KeywordParensClose,

    #[token("[")]
    KeywordSquareParensOpen,

    #[token("]")]
    KeywordSquareParensClose,

    #[token(",")]
    KeywordComma,

    #[token("...")]
    KeywordDotDotDot,

    #[token("..")]
    KeywordDotDot,

    #[token(".")]
    KeywordDot,

    //Control Structures
    #[token("IF")]
    KeywordIf,

    #[token("THEN")]
    KeywordThen,

    #[token("ELSIF")]
    KeywordElseIf,

    #[token("ELSE")]
    KeywordElse,

    #[token("END_IF")]
    KeywordEndIf,

    #[token("FOR")]
    KeywordFor,

    #[token("TO")]
    KeywordTo,

    #[token("BY")]
    KeywordBy,

    #[token("DO")]
    KeywordDo,

    #[token("END_FOR")]
    KeywordEndFor,

    #[token("WHILE")]
    KeywordWhile,

    #[token("END_WHILE")]
    KeywordEndWhile,

    #[token("REPEAT")]
    KeywordRepeat,

    #[token("UNTIL")]
    KeywordUntil,

    #[token("END_REPEAT")]
    KeywordEndRepeat,

    #[token("CASE")]
    KeywordCase,

    #[token("ARRAY")]
    KeywordArray,

    #[token("STRING")]
    KeywordString,

    #[token("WSTRING")]
    KeywordWideString,

    #[token("OF")]
    KeywordOf,

    #[token("END_CASE")]
    KeywordEndCase,

    //Operators
    #[token("+")]
    OperatorPlus,

    #[token("-")]
    OperatorMinus,

    #[token("*")]
    OperatorMultiplication,

    #[token("/")]
    OperatorDivision,

    #[token("=")]
    OperatorEqual,

    #[token("<>")]
    OperatorNotEqual,

    #[token("<")]
    OperatorLess,

    #[token(">")]
    OperatorGreater,

    #[token("<=")]
    OperatorLessOrEqual,

    #[token(">=")]
    OperatorGreaterOrEqual,

    #[token("MOD")]
    OperatorModulo,

    #[token("AND")]
    OperatorAnd,

    #[token("OR")]
    OperatorOr,

    #[token("XOR")]
    OperatorXor,

    #[token("NOT")]
    OperatorNot,

    //Identifiers
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    Identifier,

    //Literals
    #[regex(r"[0-9]+")]
    LiteralInteger,

    #[regex("[eE][+-]?[0-9]+")]
    LiteralExponent,

    #[token("TRUE")]
    LiteralTrue,

    #[token("FALSE")]
    LiteralFalse,

    #[regex("D(ATE)?#\\d+-\\d+-\\d+")]
    LiteralDate,

    #[regex("(DATE_AND_TIME|DT)#\\d+-\\d+-\\d+-\\d+:\\d+:\\d+(\\.\\d+)?")]
    LiteralDateAndTime,

    #[regex("(TIME_OF_DAY|TOD)#\\d+:\\d+:\\d+(\\.\\d+)?")]
    LiteralTimeOfDay,

    #[regex("T(IME)?#-?(\\d+(\\.\\d+)?(d|h|ms|m|s|us|ns))+")]
    LiteralTime,

    #[regex("'((\\$.)|[^$'])*'")]
    LiteralString,

    #[regex("\"((\\$.)|[^$\"])*\"")]
    LiteralWideString,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    End,
}

pub fn lex(source: &str) -> ParseSession {
    ParseSession::new(Token::lexer(source))
}
