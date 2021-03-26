/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use core::ops::Range;
use logos::Filter;
use logos::Lexer;
use logos::Logos;

use crate::ast::NewLines;

#[cfg(test)]
mod tests;

pub struct RustyLexer<'a> {
    lexer: Lexer<'a, Token>,
    pub token: Token,
    pub new_lines: NewLines,
}

impl<'a> RustyLexer<'a> {
    pub fn new(l: Lexer<'a, Token>, new_lines: NewLines) -> RustyLexer<'a> {
        let mut lexer = RustyLexer {
            lexer: l,
            token: Token::KeywordBy,
            new_lines,
        };
        lexer.advance();
        lexer
    }

    pub fn get_new_lines(&self) -> &NewLines {
        &self.new_lines
    }

    pub fn advance(&mut self) {
        self.token = self.lexer.next().unwrap_or(Token::End);
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
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

    #[regex("'((\\$.)|[^$'])*'")]
    LiteralString,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    End,
}

pub fn lex(source: &str) -> RustyLexer {
    RustyLexer::new(Token::lexer(source), NewLines::new(source))
}
