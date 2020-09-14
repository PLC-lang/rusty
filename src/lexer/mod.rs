/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use logos::Lexer;
use logos::Logos;
use core::ops::Range;

#[cfg(test)]
mod tests;

pub struct RustyLexer<'a> {
    lexer: Lexer<'a, Token>,
    pub token: Token,
}

impl<'a> RustyLexer<'a> {

    pub fn new(l: Lexer<'a, Token>) -> RustyLexer<'a> {
        let mut lexer = RustyLexer{
            lexer: l,
            token: Token::KeywordBy,
        };
        lexer.advance();
        lexer
    }    

    pub fn advance(&mut self) {
        self.token = self.lexer.next().unwrap_or(Token::End);
    }

    pub fn slice(&self) -> &str {
        self.lexer.slice()
    }

    pub fn range(&self) -> Range<usize>{
        self.lexer.span()
    }
}



#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[error]
    Error,
    

    #[token("PROGRAM")]
    KeywordProgram,

    #[token("VAR_INPUT")]
    KeywordVarInput,
    
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
    RustyLexer::new(Token::lexer(source))
}

