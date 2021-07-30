use logos::Logos;

#[derive(Debug, PartialEq, Logos, Clone)]
pub enum Token {
    #[error]
    #[regex(r"\(\*", |lex| super::parse_comments(lex))]
    #[regex(r"/\*", |lex| super::parse_comments(lex))]
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
    #[regex(r"16#[0-9a-fA-F]+(_[0-9a-fA-F]+)*")]
    LiteralIntegerHex,

    #[regex(r"8#[0-7]+(_[0-7]+)*")]
    LiteralIntegerOct,

    #[regex(r"2#[0-1]+(_[0-1]+)*")]
    LiteralIntegerBin,

    #[regex(r"[0-9]+(_[0-9]+)*")]
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
