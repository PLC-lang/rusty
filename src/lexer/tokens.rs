use logos::Logos;

use plc_ast::ast::{DirectAccessType, HardwareAccessType};

#[derive(Debug, PartialEq, Eq, Logos, Clone)]
pub enum Token {
    #[error]
    #[regex(r"\(\*", |lex| super::parse_comments(lex))]
    #[regex(r"/\*", |lex| super::parse_comments(lex))]
    #[regex(r"\{", |lex| super::parse_pragma(lex))]
    #[regex(r"//.*", logos::skip)]
    #[regex(r"(?m)\r", logos::skip)]
    Error,

    #[token("@EXTERNAL")]
    #[token("{external}")]
    PropertyExternal,

    #[token("{ref}")]
    PropertyByRef,

    #[token("{sized}")]
    PropertySized,

    #[token("PROGRAM", ignore(case))]
    KeywordProgram,

    #[token("CLASS", ignore(case))]
    KeywordClass,

    #[token("END_CLASS", ignore(case))]
    #[token("ENDCLASS", ignore(case))]
    KeywordEndClass,

    #[token("EXTENDS", ignore(case))]
    KeywordExtends,

    #[token("VAR_INPUT", ignore(case))]
    #[token("VARINPUT", ignore(case))]
    KeywordVarInput,

    #[token("VAR_OUTPUT", ignore(case))]
    #[token("VAROUTPUT", ignore(case))]
    KeywordVarOutput,

    #[token("VAR", ignore(case))]
    KeywordVar,

    #[token("ABSTRACT", ignore(case))]
    KeywordAbstract,

    #[token("FINAL", ignore(case))]
    KeywordFinal,

    #[token("METHOD", ignore(case))]
    KeywordMethod,

    #[token("CONSTANT", ignore(case))]
    KeywordConstant,

    #[token("RETAIN", ignore(case))]
    KeywordRetain,

    #[token("NON_RETAIN", ignore(case))]
    #[token("NONRETAIN", ignore(case))]
    KeywordNonRetain,

    #[token("VAR_TEMP", ignore(case))]
    #[token("VARTEMP", ignore(case))]
    KeywordVarTemp,

    #[token("END_METHOD", ignore(case))]
    #[token("ENDMETHOD", ignore(case))]
    KeywordEndMethod,

    #[token("PUBLIC", ignore(case))]
    KeywordAccessPublic,

    #[token("PRIVATE", ignore(case))]
    KeywordAccessPrivate,

    #[token("INTERNAL", ignore(case))]
    KeywordAccessInternal,

    #[token("PROTECTED", ignore(case))]
    KeywordAccessProtected,

    #[token("OVERRIDE", ignore(case))]
    KeywordOverride,

    #[token("VAR_GLOBAL", ignore(case))]
    #[token("VARGLOBAL", ignore(case))]
    KeywordVarGlobal,

    #[token("VAR_IN_OUT", ignore(case))]
    #[token("VARINOUT", ignore(case))]
    KeywordVarInOut,

    #[token("END_VAR", ignore(case))]
    #[token("ENDVAR", ignore(case))]
    KeywordEndVar,

    #[token("END_PROGRAM", ignore(case))]
    #[token("ENDPROGRAM", ignore(case))]
    KeywordEndProgram,

    #[token("FUNCTION", ignore(case))]
    KeywordFunction,

    #[token("END_FUNCTION", ignore(case))]
    #[token("ENDFUNCTION", ignore(case))]
    KeywordEndFunction,

    #[token("FUNCTION_BLOCK", ignore(case))]
    #[token("FUNCTIONBLOCK", ignore(case))]
    KeywordFunctionBlock,

    #[token("END_FUNCTION_BLOCK", ignore(case))]
    #[token("ENDFUNCTIONBLOCK", ignore(case))]
    KeywordEndFunctionBlock,

    #[token("TYPE", ignore(case))]
    KeywordType,

    #[token("STRUCT", ignore(case))]
    KeywordStruct,

    #[token("END_TYPE", ignore(case))]
    #[token("ENDTYPE", ignore(case))]
    KeywordEndType,

    #[token("END_STRUCT", ignore(case))]
    #[token("ENDSTRUCT", ignore(case))]
    KeywordEndStruct,

    #[token("ACTIONS", ignore(case))]
    KeywordActions,

    #[token("ACTION", ignore(case))]
    KeywordAction,

    #[token("END_ACTION", ignore(case))]
    #[token("ENDACTION", ignore(case))]
    KeywordEndAction,

    #[token("END_ACTIONS", ignore(case))]
    #[token("ENDACTIONS", ignore(case))]
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
    #[token("IF", ignore(case))]
    KeywordIf,

    #[token("THEN", ignore(case))]
    KeywordThen,

    #[token("ELSIF", ignore(case))]
    KeywordElseIf,

    #[token("ELSE", ignore(case))]
    KeywordElse,

    #[token("END_IF", ignore(case))]
    #[token("ENDIF", ignore(case))]
    KeywordEndIf,

    #[token("FOR", ignore(case))]
    KeywordFor,

    #[token("TO", ignore(case))]
    KeywordTo,

    #[token("BY", ignore(case))]
    KeywordBy,

    #[token("DO", ignore(case))]
    KeywordDo,

    #[token("END_FOR", ignore(case))]
    #[token("ENDFOR", ignore(case))]
    KeywordEndFor,

    #[token("WHILE", ignore(case))]
    KeywordWhile,

    #[token("END_WHILE", ignore(case))]
    #[token("ENDWHILE", ignore(case))]
    KeywordEndWhile,

    #[token("REPEAT", ignore(case))]
    KeywordRepeat,

    #[token("UNTIL", ignore(case))]
    KeywordUntil,

    #[token("END_REPEAT", ignore(case))]
    #[token("ENDREPEAT", ignore(case))]
    KeywordEndRepeat,

    #[token("CASE", ignore(case))]
    KeywordCase,

    #[token("RETURN", ignore(case))]
    KeywordReturn,

    #[token("EXIT", ignore(case))]
    KeywordExit,

    #[token("CONTINUE", ignore(case))]
    KeywordContinue,

    #[token("POINTER", ignore(case))]
    KeywordPointer,

    #[token("REF_TO", ignore(case))]
    #[token("REFTO", ignore(case))]
    KeywordRef,

    #[token("REFERENCE TO", ignore(case))]
    KeywordReferenceTo,

    #[token("ARRAY", ignore(case))]
    KeywordArray,

    #[token("STRING", ignore(case))]
    KeywordString,

    #[token("WSTRING", ignore(case))]
    KeywordWideString,

    #[token("OF", ignore(case))]
    KeywordOf,

    #[token("AT", ignore(case))]
    KeywordAt,

    #[token("END_CASE", ignore(case))]
    #[token("ENDCASE", ignore(case))]
    KeywordEndCase,

    //Operators
    #[token("+")]
    OperatorPlus,

    #[token("-")]
    OperatorMinus,

    #[token("*")]
    OperatorMultiplication,

    #[token("**")]
    OperatorExponent,

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

    #[token("&")]
    OperatorAmp,

    #[token("^")]
    OperatorDeref,

    #[token("MOD", ignore(case))]
    OperatorModulo,

    #[token("AND", ignore(case))]
    OperatorAnd,

    #[token("OR", ignore(case))]
    OperatorOr,

    #[token("XOR", ignore(case))]
    OperatorXor,

    #[token("NOT", ignore(case))]
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

    #[regex(r"[0-9]+(_[0-9]+)*([eE][+-]?[0-9]+)?")]
    LiteralInteger,

    #[token("NULL", ignore(case))]
    LiteralNull,

    #[token("TRUE", ignore(case))]
    LiteralTrue,

    #[token("FALSE", ignore(case))]
    LiteralFalse,

    #[regex("(LDATE|DATE|LD|D)#\\d+-\\d+-\\d+", ignore(case))]
    LiteralDate,

    #[regex(
        "(DATE_AND_TIME|DT|LDATE_AND_TIME|LDT)#\\d+-\\d+-\\d+-\\d+:\\d+(:\\d+(\\.\\d+)?)?",
        ignore(case)
    )]
    LiteralDateAndTime,

    #[regex("(TIME_OF_DAY|LTIME_OF_DAY|TOD|LTOD)#\\d+:\\d+(:\\d+(\\.\\d+)?)?", ignore(case))]
    LiteralTimeOfDay,

    #[regex("(LTIME|LT|TIME|T)#-?(\\d+(\\.\\d+)?(d|h|ms|m|s|us|ns))+", ignore(case))]
    LiteralTime,

    #[regex("%(B|b|D|d|W|w|L|l|X|x)", super::parse_access_type)]
    DirectAccess(DirectAccessType),

    #[regex(r"%(I|i|Q|q|M|m|G|g)(B|b|D|d|W|w|L|l|X|x|\*)", super::parse_hardware_access_type)]
    HardwareAccess((HardwareAccessType, DirectAccessType)),

    #[regex("'((\\$.)|[^$'])*'")]
    LiteralString,

    #[regex("\"((\\$.)|[^$\"])*\"")]
    LiteralWideString,

    #[regex("[a-zA-Z_][a-zA-Z_0-9]*#")]
    TypeCastPrefix,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    End,
}
