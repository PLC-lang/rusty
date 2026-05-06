use std::fmt;

use logos::Logos;

use plc_ast::ast::{DirectAccessType, HardwareAccessType};

#[derive(Debug, PartialEq, Eq, Logos, Clone, Copy)]
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

    #[token("@CONSTANT")]
    #[token("{constant}")]
    PropertyConstant,

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

    #[token("IMPLEMENTS", ignore(case))]
    KeywordImplements,

    #[token("INTERFACE", ignore(case))]
    KeywordInterface,

    #[token("END_INTERFACE", ignore(case))]
    #[token("ENDINTERFACE", ignore(case))]
    KeywordEndInterface,

    #[token("VAR_INPUT", ignore(case))]
    #[token("VARINPUT", ignore(case))]
    KeywordVarInput,

    #[token("VAR_OUTPUT", ignore(case))]
    #[token("VAROUTPUT", ignore(case))]
    KeywordVarOutput,

    #[token("VAR", ignore(case))]
    KeywordVar,

    #[token("VAR_CONFIG", ignore(case))]
    KeywordVarConfig,

    #[token("ABSTRACT", ignore(case))]
    KeywordAbstract,

    #[token("FINAL", ignore(case))]
    KeywordFinal,

    #[token("METHOD", ignore(case))]
    KeywordMethod,

    #[token("END_METHOD", ignore(case))]
    #[token("ENDMETHOD", ignore(case))]
    KeywordEndMethod,

    #[token("SUPER", ignore(case))]
    KeywordSuper,

    #[token("THIS", ignore(case))]
    KeywordThis,

    #[token("PROPERTY_GET", ignore(case))]
    #[token("PROPERTYGET", ignore(case))]
    KeywordPropertyGet,

    #[token("PROPERTY_SET", ignore(case))]
    #[token("PROPERTYSET", ignore(case))]
    KeywordPropertySet,

    #[token("END_PROPERTY", ignore(case))]
    #[token("ENDPROPERTY", ignore(case))]
    KeywordEndProperty,

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

    #[token("VAR_EXTERNAL", ignore(case))]
    KeywordVarExternal,

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

    #[token("REF=", ignore(case))]
    KeywordReferenceAssignment,

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

    #[token("__FPOINTER", ignore(case))]
    KeywordFunctionPointer,

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

    #[token("AND_THEN", ignore(case))]
    OperatorAndThen,

    #[token("AND", ignore(case))]
    OperatorAnd,

    #[token("OR_ELSE", ignore(case))]
    OperatorOrElse,

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

impl Token {
    pub fn display_list(tokens: &[Token]) -> String {
        match tokens {
            [] => "end of file".into(),
            [token] => token.to_string(),
            tokens => tokens.iter().map(ToString::to_string).collect::<Vec<_>>().join(" or "),
        }
    }

    /// Returns true if the current token represents any `VAR(_*)` keyword
    pub fn is_var(&self) -> bool {
        matches!(
            self,
            Token::KeywordVar
                | Token::KeywordVarInput
                | Token::KeywordVarOutput
                | Token::KeywordVarInOut
                | Token::KeywordVarTemp
        )
    }

    /// Returns true if the current token can be used where an identifier is expected.
    ///
    /// `PROPERTY_GET` and `PROPERTY_SET` are soft keywords: they are parsed as property accessors in
    /// property declarations, but may still be used as identifiers elsewhere.
    pub fn is_identifier_like(&self) -> bool {
        matches!(self, Token::Identifier | Token::KeywordPropertyGet | Token::KeywordPropertySet)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Error => write!(f, "unknown token"),
            Token::PropertyExternal => write!(f, "`@EXTERNAL`"),
            Token::PropertyByRef => write!(f, "`{{ref}}`"),
            Token::PropertyConstant => write!(f, "`@CONSTANT`"),
            Token::PropertySized => write!(f, "`{{sized}}`"),
            Token::KeywordProgram => write!(f, "`PROGRAM`"),
            Token::KeywordClass => write!(f, "`CLASS`"),
            Token::KeywordEndClass => write!(f, "`END_CLASS`"),
            Token::KeywordExtends => write!(f, "`EXTENDS`"),
            Token::KeywordImplements => write!(f, "`IMPLEMENTS`"),
            Token::KeywordInterface => write!(f, "`INTERFACE`"),
            Token::KeywordEndInterface => write!(f, "`END_INTERFACE`"),
            Token::KeywordVarInput => write!(f, "`VAR_INPUT`"),
            Token::KeywordVarOutput => write!(f, "`VAR_OUTPUT`"),
            Token::KeywordVar => write!(f, "`VAR`"),
            Token::KeywordVarConfig => write!(f, "`VAR_CONFIG`"),
            Token::KeywordAbstract => write!(f, "`ABSTRACT`"),
            Token::KeywordFinal => write!(f, "`FINAL`"),
            Token::KeywordMethod => write!(f, "`METHOD`"),
            Token::KeywordEndMethod => write!(f, "`END_METHOD`"),
            Token::KeywordSuper => write!(f, "`SUPER`"),
            Token::KeywordThis => write!(f, "`THIS`"),
            Token::KeywordPropertyGet => write!(f, "`PROPERTY_GET`"),
            Token::KeywordPropertySet => write!(f, "`PROPERTY_SET`"),
            Token::KeywordEndProperty => write!(f, "`END_PROPERTY`"),
            Token::KeywordConstant => write!(f, "`CONSTANT`"),
            Token::KeywordRetain => write!(f, "`RETAIN`"),
            Token::KeywordNonRetain => write!(f, "`NON_RETAIN`"),
            Token::KeywordVarTemp => write!(f, "`VAR_TEMP`"),
            Token::KeywordAccessPublic => write!(f, "`PUBLIC`"),
            Token::KeywordAccessPrivate => write!(f, "`PRIVATE`"),
            Token::KeywordAccessInternal => write!(f, "`INTERNAL`"),
            Token::KeywordAccessProtected => write!(f, "`PROTECTED`"),
            Token::KeywordOverride => write!(f, "`OVERRIDE`"),
            Token::KeywordVarGlobal => write!(f, "`VAR_GLOBAL`"),
            Token::KeywordVarInOut => write!(f, "`VAR_IN_OUT`"),
            Token::KeywordVarExternal => write!(f, "`VAR_EXTERNAL`"),
            Token::KeywordEndVar => write!(f, "`END_VAR`"),
            Token::KeywordEndProgram => write!(f, "`END_PROGRAM`"),
            Token::KeywordFunction => write!(f, "`FUNCTION`"),
            Token::KeywordEndFunction => write!(f, "`END_FUNCTION`"),
            Token::KeywordFunctionBlock => write!(f, "`FUNCTION_BLOCK`"),
            Token::KeywordEndFunctionBlock => write!(f, "`END_FUNCTION_BLOCK`"),
            Token::KeywordType => write!(f, "`TYPE`"),
            Token::KeywordStruct => write!(f, "`STRUCT`"),
            Token::KeywordEndType => write!(f, "`END_TYPE`"),
            Token::KeywordEndStruct => write!(f, "`END_STRUCT`"),
            Token::KeywordActions => write!(f, "`ACTIONS`"),
            Token::KeywordAction => write!(f, "`ACTION`"),
            Token::KeywordEndAction => write!(f, "`END_ACTION`"),
            Token::KeywordEndActions => write!(f, "`END_ACTIONS`"),
            Token::KeywordColon => write!(f, "`:`"),
            Token::KeywordSemicolon => write!(f, "`;`"),
            Token::KeywordAssignment => write!(f, "`:=`"),
            Token::KeywordOutputAssignment => write!(f, "`=>`"),
            Token::KeywordReferenceAssignment => write!(f, "`REF=`"),
            Token::KeywordParensOpen => write!(f, "`(`"),
            Token::KeywordParensClose => write!(f, "`)`"),
            Token::KeywordSquareParensOpen => write!(f, "`[`"),
            Token::KeywordSquareParensClose => write!(f, "`]`"),
            Token::KeywordComma => write!(f, "`,`"),
            Token::KeywordDotDotDot => write!(f, "`...`"),
            Token::KeywordDotDot => write!(f, "`..`"),
            Token::KeywordDot => write!(f, "`.`"),
            Token::KeywordIf => write!(f, "`IF`"),
            Token::KeywordThen => write!(f, "`THEN`"),
            Token::KeywordElseIf => write!(f, "`ELSIF`"),
            Token::KeywordElse => write!(f, "`ELSE`"),
            Token::KeywordEndIf => write!(f, "`END_IF`"),
            Token::KeywordFor => write!(f, "`FOR`"),
            Token::KeywordTo => write!(f, "`TO`"),
            Token::KeywordBy => write!(f, "`BY`"),
            Token::KeywordDo => write!(f, "`DO`"),
            Token::KeywordEndFor => write!(f, "`END_FOR`"),
            Token::KeywordWhile => write!(f, "`WHILE`"),
            Token::KeywordEndWhile => write!(f, "`END_WHILE`"),
            Token::KeywordRepeat => write!(f, "`REPEAT`"),
            Token::KeywordUntil => write!(f, "`UNTIL`"),
            Token::KeywordEndRepeat => write!(f, "`END_REPEAT`"),
            Token::KeywordCase => write!(f, "`CASE`"),
            Token::KeywordReturn => write!(f, "`RETURN`"),
            Token::KeywordExit => write!(f, "`EXIT`"),
            Token::KeywordContinue => write!(f, "`CONTINUE`"),
            Token::KeywordPointer => write!(f, "`POINTER`"),
            Token::KeywordFunctionPointer => write!(f, "`__FPOINTER`"),
            Token::KeywordRef => write!(f, "`REF_TO`"),
            Token::KeywordReferenceTo => write!(f, "`REFERENCE TO`"),
            Token::KeywordArray => write!(f, "`ARRAY`"),
            Token::KeywordString => write!(f, "`STRING`"),
            Token::KeywordWideString => write!(f, "`WSTRING`"),
            Token::KeywordOf => write!(f, "`OF`"),
            Token::KeywordAt => write!(f, "`AT`"),
            Token::KeywordEndCase => write!(f, "`END_CASE`"),
            Token::OperatorPlus => write!(f, "`+`"),
            Token::OperatorMinus => write!(f, "`-`"),
            Token::OperatorMultiplication => write!(f, "`*`"),
            Token::OperatorExponent => write!(f, "`**`"),
            Token::OperatorDivision => write!(f, "`/`"),
            Token::OperatorEqual => write!(f, "`=`"),
            Token::OperatorNotEqual => write!(f, "`<>`"),
            Token::OperatorLess => write!(f, "`<`"),
            Token::OperatorGreater => write!(f, "`>`"),
            Token::OperatorLessOrEqual => write!(f, "`<=`"),
            Token::OperatorGreaterOrEqual => write!(f, "`>=`"),
            Token::OperatorAmp => write!(f, "`&`"),
            Token::OperatorDeref => write!(f, "`^`"),
            Token::OperatorModulo => write!(f, "`MOD`"),
            Token::OperatorAndThen => write!(f, "`AND_THEN`"),
            Token::OperatorAnd => write!(f, "`AND`"),
            Token::OperatorOrElse => write!(f, "`OR_ELSE`"),
            Token::OperatorOr => write!(f, "`OR`"),
            Token::OperatorXor => write!(f, "`XOR`"),
            Token::OperatorNot => write!(f, "`NOT`"),
            Token::Identifier => write!(f, "identifier"),
            Token::LiteralIntegerHex
            | Token::LiteralIntegerOct
            | Token::LiteralIntegerBin
            | Token::LiteralInteger => {
                write!(f, "integer literal")
            }
            Token::LiteralNull => write!(f, "`NULL`"),
            Token::LiteralTrue => write!(f, "`TRUE`"),
            Token::LiteralFalse => write!(f, "`FALSE`"),
            Token::LiteralDate => write!(f, "date literal"),
            Token::LiteralDateAndTime => write!(f, "date and time literal"),
            Token::LiteralTimeOfDay => write!(f, "time of day literal"),
            Token::LiteralTime => write!(f, "time literal"),
            Token::DirectAccess(_) => write!(f, "direct access"),
            Token::HardwareAccess(_) => write!(f, "hardware access"),
            Token::LiteralString => write!(f, "string literal"),
            Token::LiteralWideString => write!(f, "wide string literal"),
            Token::TypeCastPrefix => write!(f, "type cast prefix"),
            Token::End => write!(f, "end of file"),
        }
    }
}
