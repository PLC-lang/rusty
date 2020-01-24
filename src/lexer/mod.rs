use logos::Lexer;
use logos::Logos;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
    #[end]
    End,
    #[error]
    Error,

    #[token = "PROGRAM"]
    KeywordProgram,

    #[token = "VAR"]
    KeywordVar,

    #[token = "END_VAR"]
    KeywordEndVar,

    #[token = "END_PROGRAM"]
    KeywordEndProgram,
    
    #[token = ":"]
    KeywordColon,

    #[token = ";"]
    KeywordSemicolon,

    #[token = ":="]
    KeywordAssignment,

    #[token = "("]
    KeywordParensOpen,

    #[token = ")"]
    KeywordParensClose,

    //Control Structures
    #[token = "IF"]
    KeywordIf,

    #[token = "THEN"]
    KeywordThen,

    #[token = "ELSIF"]
    KeywordElseIf,

    #[token = "ELSE"]
    KeywordElse,

    #[token = "END_IF"]
    KeywordEndIf,

    #[token = "FOR"]
    KeywordFor,
    
    #[token = "TO"]
    KeywordTo,

    #[token = "BY"]
    KeywordBy,
 
    #[token = "DO"]
    KeywordDo,
 
    #[token = "END_FOR"]
    KeywordEndFor,

    //Operators
    #[token = "+"]
    OperatorPlus,

    #[token = "-"]
    OperatorMinus,

    #[token = "*"]
    OperatorMultiplication,

    #[token = "/"]
    OperatorDivision,

    #[token = "="]
    OperatorEqual,

    #[token = "<>"]
    OperatorNotEqual,

    #[token = "<"]
    OperatorLess,

    #[token = ">"]
    OperatorGreater,

    #[token = "<="]
    OperatorLessOrEqual,

    #[token = ">="]
    OperatorGreaterOrEqual,

    #[token = "MOD"]
    OperatorModulo,

    #[token = "AND"]
    OperatorAnd,

    #[token = "OR"]
    OperatorOr,

    #[token = "XOR"]
    OperatorXor,

    #[token = "NOT"]
    OperatorNot,

    //Identifiers

    #[regex = r"[a-zA-Z_][a-zA-Z_0-9]*"]
    Identifier,

    //Literals

    #[regex = r"[0-9]+(\.(0-9)+)?"]
    LiteralNumber,

    #[token = "TRUE"]
    LiteralTrue,

    #[token = "FALSE"]
    LiteralFalse,
}

pub fn lex(source: &str) -> Lexer<Token, &str> {
    Token::lexer(source)
}

