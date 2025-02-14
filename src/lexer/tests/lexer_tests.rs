// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder

use plc_ast::ast::{DirectAccessType, HardwareAccessType};
use pretty_assertions::{assert_eq, assert_ne};

use crate::lexer::{lex, Token::*};

#[test]
fn generic_properties() {
    let lexer = lex("@EXTERNAL");
    assert_eq!(lexer.token, PropertyExternal);
}

#[test]
fn windows_and_linux_line_separators_ignored() {
    let mut lexer = lex("PROGRAM\r\nEND_PROGRAM");
    assert_eq!(lexer.token, KeywordProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndProgram, "Token : {}", lexer.slice());
}

#[test]
fn comments_are_ignored_by_the_lexer() {
    let mut lexer = lex(r"
        PROGRAM (* Some Content *) END_PROGRAM
                                   /*
                                    * FUNCTION */
        (* Nested (*) Comment *) *)
        /* Nested /* Comment */ */
        //END_FUNCTION FUNCTION_BLOCK
        END_FUNCTION_BLOCK
        ");
    assert_eq!(lexer.token, KeywordProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndFunctionBlock, "Token : {}", lexer.slice());
    lexer.advance();
}

#[test]
fn undefined_pragmas_are_ignored_by_the_lexer() {
    let mut lexer = lex(r"
        PROGRAM { Some Content } END_PROGRAM
                                   {
                                    FUNCTION }
        {END_FUNCTION FUNCTION_BLOCK}
        END_FUNCTION_BLOCK
        ");
    assert_eq!(lexer.token, KeywordProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndFunctionBlock, "Token : {}", lexer.slice());
    lexer.advance();
}

#[test]
fn registered_pragmas_parsed() {
    let mut lexer = lex(r"
        {external}{ref}{sized}{not_registerd}
        ");
    assert_eq!(lexer.token, PropertyExternal, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, PropertyByRef, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, PropertySized, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, End);
}

#[test]
fn comments_are_not_ignored_in_strings() {
    let mut lexer = lex(r#"
        'PROGRAM (* Some Content *) END_PROGRAM
                                   /*
                                    * FUNCTION */
        (* Nested (*) Comment *) *)
        /* Nested /* Comment */ */
        //END_FUNCTION FUNCTION_BLOCK
        END_FUNCTION_BLOCK'
        "#);
    assert_eq!(lexer.token, LiteralString, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, End, "Token : {}", lexer.slice());
}

#[test]
fn string_delimiter_dont_leak_out_of_comments() {
    let mut lexer = lex(r#"
        '(* Some Content *)'
        (* ' *) 'xx' // '
        ' abc // '
        "#);
    assert_eq!(lexer.token, LiteralString, "Token : {}", lexer.slice());
    assert_eq!(lexer.slice(), "'(* Some Content *)'");
    lexer.advance();
    assert_eq!(lexer.token, LiteralString, "Token : {}", lexer.slice());
    assert_eq!(lexer.slice(), "'xx'");
    lexer.advance();
    assert_eq!(lexer.token, LiteralString, "Token : {}", lexer.slice());
    assert_eq!(lexer.slice(), "' abc // '");
}

#[test]
fn unicode_chars_in_comments() {
    let mut lexer = lex(r"
        PROGRAM (* Some Content *) END_PROGRAM
                                   /*
                                    * FUNCTION */
        (* Nested //2 char utf-8 -> 💖ß (*) //2 char utf-16 --> 💣 Comment *) *)
        /* Nested /* Comment */ */
        //END_FUNCTION FUNCTION_BLOCK
        END_FUNCTION_BLOCK
        ");
    assert_eq!(lexer.token, KeywordProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndProgram, "Token : {}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndFunctionBlock, "Token : {}", lexer.slice());
    lexer.advance();
}

#[test]
fn pou_tokens() {
    let mut lexer = lex("PROGRAM END_PROGRAM FUNCTION END_FUNCTION FUNCTION_BLOCK END_FUNCTION_BLOCK");
    assert_eq!(lexer.token, KeywordProgram);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndProgram);
    lexer.advance();
    assert_eq!(lexer.token, KeywordFunction);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndFunction);
    lexer.advance();
    assert_eq!(lexer.token, KeywordFunctionBlock);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndFunctionBlock);
    lexer.advance();
}

#[test]
fn action_tokens() {
    let mut lexer = lex("ACTIONS ACTION END_ACTION END_ACTIONS");
    assert_eq!(lexer.token, KeywordActions);
    lexer.advance();
    assert_eq!(lexer.token, KeywordAction);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndAction);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndActions);
}

#[test]
fn var_tokens() {
    let mut lexer = lex("VAR VAR_INPUT VAR_OUTPUT VAR_GLOBAL VAR_IN_OUT END_VAR");
    assert_eq!(lexer.token, KeywordVar);
    lexer.advance();
    assert_eq!(lexer.token, KeywordVarInput);
    lexer.advance();
    assert_eq!(lexer.token, KeywordVarOutput);
    lexer.advance();
    assert_eq!(lexer.token, KeywordVarGlobal);
    lexer.advance();
    assert_eq!(lexer.token, KeywordVarInOut);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndVar);
}

#[test]
fn hello_is_an_identifier() {
    let mut lexer = lex("hello a12 _a12");
    assert_eq!(lexer.token, Identifier, "{}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, Identifier, "{}", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, Identifier, "{}", lexer.slice());
    lexer.advance();
}

#[test]
fn an_identifier_cannot_start_with_a_number() {
    let lexer = lex("2g12");
    assert_ne!(lexer.token, Identifier);
}

#[test]
fn punctuations() {
    let lexer = lex(":");
    assert_eq!(lexer.token, KeywordColon, "{}", lexer.slice());
    let lexer = lex(";");
    assert_eq!(lexer.token, KeywordSemicolon, "{}", lexer.slice());
}

#[test]
fn parens() {
    let mut lexer = lex("( )");
    assert_eq!(lexer.token, KeywordParensOpen);
    lexer.advance();
    assert_eq!(lexer.token, KeywordParensClose);
}

#[test]
fn a_assignment_is_keyword_assignment() {
    let mut lexer = lex(":= =>");
    assert_eq!(lexer.token, KeywordAssignment);
    lexer.advance();
    assert_eq!(lexer.token, KeywordOutputAssignment);
}

#[test]
fn comma() {
    let lexer = lex(",");
    assert_eq!(lexer.token, KeywordComma);
}

#[test]
fn operator_test() {
    let mut lexer = lex("+ - * ** / MOD = <> < > <= >=");
    assert_eq!(lexer.token, OperatorPlus);
    lexer.advance();
    assert_eq!(lexer.token, OperatorMinus);
    lexer.advance();
    assert_eq!(lexer.token, OperatorMultiplication);
    lexer.advance();
    assert_eq!(lexer.token, OperatorExponent);
    lexer.advance();
    assert_eq!(lexer.token, OperatorDivision);
    lexer.advance();
    assert_eq!(lexer.token, OperatorModulo);
    lexer.advance();
    assert_eq!(lexer.token, OperatorEqual);
    lexer.advance();
    assert_eq!(lexer.token, OperatorNotEqual);
    lexer.advance();
    assert_eq!(lexer.token, OperatorLess);
    lexer.advance();
    assert_eq!(lexer.token, OperatorGreater);
    lexer.advance();
    assert_eq!(lexer.token, OperatorLessOrEqual);
    lexer.advance();
    assert_eq!(lexer.token, OperatorGreaterOrEqual);
}

#[test]
fn boolean_expression_test() {
    let mut lexer = lex("AND XOR OR NOT");
    assert_eq!(lexer.token, OperatorAnd);
    lexer.advance();
    assert_eq!(lexer.token, OperatorXor);
    lexer.advance();
    assert_eq!(lexer.token, OperatorOr);
    lexer.advance();
    assert_eq!(lexer.token, OperatorNot);
}

#[test]
fn int_literals_test() {
    let mut lexer = lex("1 2 3 0123 321 43_000 43__000 12_00E5 12e5");

    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "1");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "2");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "3");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "0123");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "321");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "43_000");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "43");
    assert_eq!(lexer.token, Identifier);
    assert_eq!(lexer.slice_and_advance(), "__000");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "12_00E5");
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!(lexer.slice_and_advance(), "12e5");
}

#[test]
fn real_literals_test() {
    let mut lexer = lex("1.234 0.9E10");

    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDot);
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDot);
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
}

#[test]
fn date_literals_test() {
    let mut lexer = lex(r#"
        DATE#1984-10-01 D#1-1-1
        DATE#1946 D#2001.10.04
        DATE#1946-4 D#-1-1-1
        "#);
    for _ in 1..=2 {
        assert_eq!(lexer.token, LiteralDate);
        lexer.advance();
    }

    for _ in 1..=4 {
        assert_ne!(lexer.token, LiteralDate);
        lexer.advance();
    }
}

#[test]
fn long_date_literals_test() {
    let mut lexer = lex(r#"
        LDATE#1984-10-01 LDATE#1-1-1
        LD#1-1-1
        INT#1
        "#);
    for _ in 1..=3 {
        assert_eq!(lexer.token, LiteralDate);
        lexer.advance();
    }
    assert_ne!(lexer.token, LiteralDate);
}

#[test]
fn date_and_time_literals_test() {
    let mut lexer = lex(
        "DATE_AND_TIME#1984-10-01-20:15:12 DT#1-1-1-1:1:1 DT#1-1-1-1:1:1.123 DATE_AND_TIME#2000-01-01-20:15",
    );
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
}

#[test]
fn long_date_and_time_literals_test() {
    let mut lexer = lex("
    LDT#1984-10-01-20:15:12 LDT#1-1-1-1:1:1 LDT#1984-10-01-20:15 LDT#1984-10-01-20:15:12.123");
    for _ in 1..=4 {
        assert_eq!(lexer.token, LiteralDateAndTime);
        lexer.advance();
    }
}

#[test]
fn time_of_day_literals_test() {
    let mut lexer = lex("TIME_OF_DAY#20:15:12 TOD#1:1:1 TOD#1:1:1.123 TIME_OF_DAY#12:13 TOD#10:20");
    for _ in 1..=5 {
        assert_eq!(lexer.token, LiteralTimeOfDay);
        lexer.advance();
    }
}

#[test]
fn long_time_of_day_literals_test() {
    let mut lexer = lex("LTOD#20:15:12 LTOD#1:1:1");
    for _ in 1..=2 {
        assert_eq!(lexer.token, LiteralTimeOfDay);
        lexer.advance();
    }
}

#[test]
fn time_literals_test() {
    let mut lexer = lex(r#"
    T#12d T#13h TIME#14m TIME#15s T#16ms
    T#12d10ms T#12h10m TIME#12m4s3ns
    TIME#4d6h8M7s12ms04US2ns
    "#);
    for _ in 1..9 {
        assert_eq!(
            lexer.token,
            LiteralTime,
            "{} at {:?} is no Time Literal",
            lexer.slice(),
            lexer.location()
        );
        lexer.advance();
    }
}

#[test]
fn ltime_literals_test() {
    let mut lexer = lex(r#"
    LTIME#12d
    LTIME#10M4S
    LT#10d
    DINT#10
    "#);
    for _ in 1..=3 {
        assert_eq!(
            lexer.token,
            LiteralTime,
            "{} at {:?} is no Time Literal",
            lexer.slice(),
            lexer.location()
        );
        lexer.advance();
    }
    assert_ne!(lexer.token, LiteralTime);
}

#[test]
fn a_full_program_generates_correct_token_sequence() {
    let mut lexer = lex(r"
        PROGRAM hello
        VAR
          a : INT;
        END_VAR
        END_PROGRAM
        ");

    assert_eq!(lexer.token, KeywordProgram);
    lexer.advance();
    assert_eq!(lexer.token, Identifier);
    lexer.advance();
    assert_eq!(lexer.token, KeywordVar);
    lexer.advance();
    assert_eq!(lexer.token, Identifier);
    lexer.advance();
    assert_eq!(lexer.token, KeywordColon);
    lexer.advance();
    assert_eq!(lexer.token, Identifier);
    lexer.advance();
    assert_eq!(lexer.token, KeywordSemicolon);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndVar);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndProgram);
}

#[test]
fn boolean_literals() {
    let mut lexer = lex(r" TRUE FALSE");
    assert_eq!(lexer.token, LiteralTrue);
    lexer.advance();
    assert_eq!(lexer.token, LiteralFalse);
}

#[test]
fn if_expression() {
    let mut lexer = lex(r"
        IF THEN ELSIF ELSE END_IF
        ");

    assert_eq!(lexer.token, KeywordIf);
    lexer.advance();
    assert_eq!(lexer.token, KeywordThen);
    lexer.advance();
    assert_eq!(lexer.token, KeywordElseIf);
    lexer.advance();
    assert_eq!(lexer.token, KeywordElse);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndIf);
}

#[test]
fn for_statement() {
    let mut lexer = lex(r"
        FOR TO BY DO END_FOR
        ");

    assert_eq!(lexer.token, KeywordFor);
    lexer.advance();
    assert_eq!(lexer.token, KeywordTo);
    lexer.advance();
    assert_eq!(lexer.token, KeywordBy);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDo);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndFor);
}

#[test]
fn while_statement() {
    let mut lexer = lex(r"
        WHILE DO END_WHILE
        ");

    assert_eq!(lexer.token, KeywordWhile);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDo);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndWhile);
}

#[test]
fn repeat_statement() {
    let mut lexer = lex(r"
        REPEAT UNTIL END_REPEAT
        ");

    assert_eq!(lexer.token, KeywordRepeat);
    lexer.advance();
    assert_eq!(lexer.token, KeywordUntil);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndRepeat);
}

#[test]
fn case_statement() {
    let mut lexer = lex(r"
        CASE OF ELSE END_CASE
        ");

    assert_eq!(lexer.token, KeywordCase);
    lexer.advance();
    assert_eq!(lexer.token, KeywordOf);
    lexer.advance();
    assert_eq!(lexer.token, KeywordElse);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndCase);
}

#[test]
fn dot_statements() {
    let mut lexer = lex(r"... .. .");

    assert_eq!(lexer.token, KeywordDotDotDot);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDotDot);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDot);
    lexer.advance();
}

#[test]
fn range_statements() {
    let mut lexer = lex(r"123..ABC");

    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDotDot);
    lexer.advance();
    assert_eq!(lexer.token, Identifier);
    lexer.advance();
}

#[test]
fn struct_enum_datatype() {
    let mut lexer = lex(r"TYPE STRUCT END_STRUCT END_TYPE");

    assert_eq!(lexer.token, KeywordType);
    lexer.advance();
    assert_eq!(lexer.token, KeywordStruct);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndStruct);
    lexer.advance();
    assert_eq!(lexer.token, KeywordEndType);
}

#[test]
fn array_parsing() {
    let mut lexer = lex(r"ARRAY OF x[5]");

    assert_eq!(lexer.token, KeywordArray);
    lexer.advance();
    assert_eq!(lexer.token, KeywordOf);
    lexer.advance();
    assert_eq!(lexer.token, Identifier);
    lexer.advance();
    assert_eq!(lexer.token, KeywordSquareParensOpen);
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, KeywordSquareParensClose);
    lexer.advance();
}

#[test]
fn string_parsing() {
    let mut lexer = lex(r"STRING 'AB C' 'AB$$' 'AB$''");

    assert_eq!(lexer.token, KeywordString);
    assert_eq!("STRING", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralString);
    assert_eq!("'AB C'", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralString);
    assert_eq!("'AB$$'", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralString);
    assert_eq!("'AB$''", lexer.slice());
    lexer.advance();
}

#[test]
fn type_cast_prefixes_parsing() {
    let mut lexer = lex("
    INT#123
    BOOL#TRUE
    ");

    assert_eq!(lexer.token, TypeCastPrefix);
    assert_eq!("INT#", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    assert_eq!("123", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, TypeCastPrefix);
    assert_eq!("BOOL#", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralTrue);
    assert_eq!("TRUE", lexer.slice());
    lexer.advance();
}

#[test]
fn wide_string_parsing() {
    let mut lexer = lex(r#"
    WSTRING
    "AB C"
    "AB$$"
    "AB$""
    "#);

    assert_eq!(lexer.token, KeywordWideString);
    assert_eq!("WSTRING", lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralWideString);
    assert_eq!(r#""AB C""#, lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralWideString);
    assert_eq!(r#""AB$$""#, lexer.slice());
    lexer.advance();
    assert_eq!(lexer.token, LiteralWideString);
    assert_eq!(r#""AB$"""#, lexer.slice());
    lexer.advance();
}

#[test]
fn pointers_and_references_keyword() {
    let mut lexer = lex(r#"
    POINTER TO x
    REF_TO x
    REFTO x
    &x
    x^
    NULL
    "#);

    assert_eq!(lexer.token, KeywordPointer);
    lexer.advance();
    assert_eq!(lexer.token, KeywordTo);
    lexer.advance();
    assert_eq!(lexer.slice(), "x");
    lexer.advance();
    assert_eq!(lexer.token, KeywordRef);
    lexer.advance();
    assert_eq!(lexer.slice(), "x");
    lexer.advance();
    assert_eq!(lexer.token, KeywordRef);
    lexer.advance();
    assert_eq!(lexer.slice(), "x");
    lexer.advance();
    assert_eq!(lexer.token, OperatorAmp);
    lexer.advance();
    assert_eq!(lexer.slice(), "x");
    lexer.advance();
    assert_eq!(lexer.slice(), "x");
    lexer.advance();
    assert_eq!(lexer.token, OperatorDeref);
    lexer.advance();
    assert_eq!(lexer.token, LiteralNull);
    lexer.advance();
}

#[test]
fn direct_access_test() {
    let mut lexer = lex(r"
        %X1 %x1 %B1 %b1
        %W1 %w1 %D1 %d1
        %L1 %l1 %X1_1
    ");

    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Bit));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Bit));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Byte));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Byte));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Word));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Word));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::DWord));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::DWord));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::LWord));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::LWord));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, DirectAccess(DirectAccessType::Bit));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, End);
}

#[test]
fn hardware_access_test() {
    let mut lexer = lex("AT %I* %Q* %M* %IX1.1 %IB2.2 %QW5 %MD7 %IL6 %GX8");
    assert_eq!(lexer.token, KeywordAt);
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Input, DirectAccessType::Template)));
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Output, DirectAccessType::Template)));
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Memory, DirectAccessType::Template)));
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Input, DirectAccessType::Bit)));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDot);
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Input, DirectAccessType::Byte)));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDot);
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Output, DirectAccessType::Word)));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Memory, DirectAccessType::DWord)));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Input, DirectAccessType::LWord)));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, HardwareAccess((HardwareAccessType::Global, DirectAccessType::Bit)));
    lexer.advance();
    assert_eq!(lexer.token, LiteralInteger);
    lexer.advance();
    assert_eq!(lexer.token, End);
}

#[test]
fn multi_named_keywords_without_underscore_test() {
    let mut lexer = lex("VARINPUT VARGLOBAL VARINOUT REFTO ENDVAR ENDPROGRAM ENDFUNCTION ENDCASE
        VARRETAIN VARTEMP VAROUTPUT FUNCTIONBLOCK ENDFUNCTIONBLOCK ENDSTRUCT ENDACTION
        ENDACTIONS ENDIF ENDFOR ENDREPEAT");

    while lexer.token != End {
        lexer.advance();
    }

    assert_eq!(lexer.diagnostics.len(), 18);

    let d1 = lexer.diagnostics.first().unwrap();
    let d2 = lexer.diagnostics.last().unwrap();

    assert_eq!(d1.get_message(), "the words in VARINPUT should be separated by a `_`");
    assert_eq!(d1.get_location().to_range().unwrap(), (0..8));

    assert_eq!(d2.get_message(), "the words in ENDREPEAT should be separated by a `_`");
    assert_eq!(d2.get_location().to_range().unwrap(), (191..200));
}

#[test]
fn lowercase_keywords_accepted() {
    let mut result = lex(r###"
        program class end_class endclass var_input varinput var_output
        varoutput var abstract final method constant retain non_retain
        nonretain var_temp vartemp end_method endmethod
        public private internal protected override
        var_global varglobal var_in_out varinout end_var endvar
        end_program endprogram end_function endfunction end_function_block endfunctionblock
        type struct end_type endtype end_struct endstruct
        actions action end_action endaction end_actions endactions
        if then elsif else endif end_if
        for to by do end_for endfor
        while end_while endwhile repeat until endrepeat end_repeat
        case return exit continue array string wstring
        of endcase end_case mod and or xor not true false
        d#1-2-3 date#1-2-3 dt#1-2-3-1:2:3 date_and_time#1-2-3-1:2:3 tod#1:2:3 time_of_day#1:2:3 time#1s t#1s null refto pointer ref_to
        "###);

    while result.token != End {
        if result.token == Identifier || result.token == Error {
            panic!("Unextected token {} : {:?}", result.slice(), result.token);
        }
        result.advance();
    }
}

#[test]
fn property_related_keywords() {
    let mut lexer = lex(r"
        PROPERTY END_PROPERTY GET END_GET SET END_SET
        ENDPROPERTY ENDGET ENDSET
    ");

    assert!(lexer.try_consume(KeywordProperty));
    assert!(lexer.try_consume(KeywordEndProperty));
    assert!(lexer.try_consume(KeywordGet));
    assert!(lexer.try_consume(KeywordEndGet));
    assert!(lexer.try_consume(KeywordSet));
    assert!(lexer.try_consume(KeywordEndSet));

    assert!(lexer.try_consume(KeywordEndProperty));
    assert!(lexer.try_consume(KeywordEndGet));
    assert!(lexer.try_consume(KeywordEndSet));
}
