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

    #[token = "VAR_GLOBAL"]
    KeywordVarGlobal, 

    #[token = "END_VAR"]
    KeywordEndVar,

    #[token = "END_PROGRAM"]
    KeywordEndProgram,

    #[token = "FUNCTION"]
    KeywordFunction,

    #[token = "END_FUNCTION"]
    KeywordEndFunction,
 
    #[token = "FUNCTION_BLOCK"]
    KeywordFunctionBlock,

    #[token = "END_FUNCTION_BLOCK"]
    KeywordEndFunctionBlock,

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

    #[token = ","] 
    KeywordComma,

    #[token = ".."]
    KeywordDotDot,

    #[token ="."]
    KeywordDot,

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

    #[token = "WHILE"]
    KeywordWhile,

    #[token = "END_WHILE"]
    KeywordEndWhile,

    #[token = "REPEAT"]
    KeywordRepeat,

    #[token = "UNTIL"]
    KeywordUntil,

    #[token = "END_REPEAT"]
    KeywordEndRepeat,

    #[token = "CASE"]
    KeywordCase,
    
    #[token = "OF"]
    KeywordOf,
    
    #[token = "END_CASE"]
    KeywordEndCase,

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

    #[regex = r"[0-9]+"]
    LiteralInteger,

    #[regex = "[eE][+-]?[0-9]+"]
    LiteralExponent,

    #[token = "TRUE"]
    LiteralTrue,

    #[token = "FALSE"]
    LiteralFalse,

}

pub fn lex(source: &str) -> Lexer<Token, &str> {
    Token::lexer(source)
}

