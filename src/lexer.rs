// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use core::ops::Range;
use logos::Filter;
use logos::Lexer;
use logos::Logos;

use crate::ast::{NewLines, SourceRange};
use crate::Diagnostic;

#[cfg(test)]
mod tests;

pub struct ParseSession<'a> {
    lexer: Lexer<'a, Token>,
    file_path: String,
    pub token: Token,
    pub new_lines: NewLines,
    pub diagnostics: Vec<Diagnostic>,
    pub closing_keywords: Vec<Vec<Token>>,
    pub last_token: Token,
    pub parse_progress: usize,
}

impl<'a> ParseSession<'a> {
    pub fn new(l: Lexer<'a, Token>, file_path: &str, new_lines: NewLines) -> ParseSession<'a> {
        let mut lexer = ParseSession {
            lexer: l,
            file_path: file_path.into(),
            token: Token::KeywordBy,
            new_lines,
            diagnostics: vec![],
            closing_keywords: vec![],
            last_token: Token::End,
            parse_progress: 0,
        };
        lexer.advance();
        lexer
    }

    pub fn slice_region(&self, range: Range<usize>) -> &str {
        &self.lexer.source()[range]
    }

    pub fn get_new_lines(&self) -> &NewLines {
        &self.new_lines
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn advance(&mut self) {
        self.last_token =
            std::mem::replace(&mut self.token, self.lexer.next().unwrap_or(Token::End));
        self.parse_progress = self.parse_progress + 1;
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn location(&self) -> SourceRange {
        SourceRange::new(self.get_file_path(), self.range())
    }

    pub fn range(&self) -> Range<usize> {
        self.lexer.span()
    }

    pub fn get_current_line_nr(&self) -> usize {
        self.new_lines.get_line_of(self.range().start).unwrap_or(1)
    }

    pub fn get_location_information(&self) -> String {
        let line_index = self.new_lines.get_line_of(self.range().start);

        let location = line_index.map_or_else(
            || self.range(),
            |it| {
                let new_line_offset = self.new_lines.get_offest_of_line(it);
                let current_range = self.range();
                (current_range.start - new_line_offset)..(current_range.end - new_line_offset)
            },
        );
        format!(
            "line: {line:?} offset: {location:?}",
            line = line_index.map_or_else(|| 1, |line_index| line_index),
            location = location
        )
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

    //pops from the stack until it sees end_token
    pub fn recover_until_close(&mut self) {
        let mut hit = self
            .closing_keywords
            .iter()
            .rposition(|it| it.contains(&self.token));

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
                    self.closing_keywords.last().and_then(|it| it.first()).unwrap_or(&Token::End) //only show first expected token
                ),
                format!("'{}'", self.slice_region(range.clone())),
                SourceRange::new(start.get_file_path(), range),
            ));
        }

        if let Some(hit) = hit {
            if self.closing_keywords.len() > hit + 1 {
                let closing = self.closing_keywords.pop().unwrap();
                self.accept_diagnostic(Diagnostic::missing_token(
                    format!("{:?}", closing),
                    self.location(),
                ));
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

#[derive(Debug, PartialEq, Logos)]
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

impl Token {
    pub fn ends_implementation(&self) -> bool {
        matches!(
            self,
            &Token::KeywordEndFunction
                | &Token::KeywordEndProgram
                | &Token::KeywordEndFunctionBlock
                | &Token::KeywordEndAction
        )
    }
}

pub fn lex<'src>(file_path: &'src str, source: &'src str) -> ParseSession<'src> {
    ParseSession::new(Token::lexer(source), file_path, NewLines::new(source))
}
