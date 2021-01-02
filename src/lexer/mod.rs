/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use logos::Lexer;
use logos::Logos;
use core::ops::Range;

#[cfg(test)]
mod tests;

pub struct RustyLexer<'a> {
    lexer: Lexer<'a, Token>,
    pub token: Token,
    pub new_lines: Vec<usize>,
}

impl<'a> RustyLexer<'a> {

    pub fn new(l: Lexer<'a, Token>, new_lines: Vec<usize>) -> RustyLexer<'a> {
        let mut lexer = RustyLexer{
            lexer: l,
            token: Token::KeywordBy,
            new_lines
        };
        lexer.advance();
        lexer
    }

    pub fn get_new_lines(&self) -> &Vec<usize> {
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

    /// binary search the first element which is bigger than the given index
    fn index_of_line_offset(&self, offset: usize) -> Option<usize> {

        if offset == 0 { return Some(1); }

        let mut start  = 0;
        let mut end   = self.new_lines.len() - 1;
        let mut result: usize = 0;
        while  start <= end {
            let mid = (start + end) / 2;

            if self.new_lines[mid] <= offset {
                start = mid + 1; //move to the right
            } else {
                result = mid;
                end = mid - 1;
            }
        }

        return if self.new_lines[result] > offset {
            Some(result)
        } else {
            None
        }
    }

    pub fn get_current_line_nr(&self) -> usize {
        self.index_of_line_offset(self.range().start).unwrap_or(0)
    }

    pub fn get_location_information(&self) -> String {
        let line_index = self.index_of_line_offset(self.range().start);

        let location = line_index.map_or_else(
            || self.range(), 
            |it| {
                let new_line_offset = self.new_lines[it-1];
                let current_range = self.range();
                (current_range.start - new_line_offset) .. (current_range.end - new_line_offset)
            });
        format!("line: {line:?} offset: {location:?}",
                line = line_index.map_or_else(|| 1, |line_index| line_index),
                location = location)
    }
}



#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[error]
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

    #[regex (r"[a-zA-Z_][a-zA-Z_0-9]*")]
    Identifier,

    //Literals

    #[regex (r"[0-9]+")]
    LiteralInteger,

    #[regex ("[eE][+-]?[0-9]+")]
    LiteralExponent,

    #[token("TRUE")]
    LiteralTrue,

    #[token("FALSE")]
    LiteralFalse,

    #[regex ("'((\\$.)|[^$'])*'")]
    LiteralString,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    End,
}


pub fn lex(source: &str) -> RustyLexer {
    RustyLexer::new(Token::lexer(source), analyze_new_lines(source))
}

fn analyze_new_lines(source: &str) -> Vec<usize>{
    let mut new_lines = Vec::new();
    new_lines.push(0);
    for (offset, c) in source.char_indices() {
        if c == '\n' {
            new_lines.push(offset);
        }
    }
    new_lines
}

