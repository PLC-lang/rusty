// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use pretty_assertions::{assert_eq, assert_ne};

use crate::lexer::{RustyLexer, Token::*};

fn lex(source: &str) -> RustyLexer {
    crate::lexer::lex("", source)
}

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
    assert_eq!(
        lexer.token,
        KeywordEndFunctionBlock,
        "Token : {}",
        lexer.slice()
    );
    lexer.advance();
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
    assert_eq!(
        lexer.token,
        KeywordEndFunctionBlock,
        "Token : {}",
        lexer.slice()
    );
    lexer.advance();
}

#[test]
fn pou_tokens() {
    let mut lexer =
        lex("PROGRAM END_PROGRAM FUNCTION END_FUNCTION FUNCTION_BLOCK END_FUNCTION_BLOCK");
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
    let mut lexer = lex("+ - * / MOD = <> < > <= >=");
    assert_eq!(lexer.token, OperatorPlus);
    lexer.advance();
    assert_eq!(lexer.token, OperatorMinus);
    lexer.advance();
    assert_eq!(lexer.token, OperatorMultiplication);
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
    let mut lexer = lex("1 2 3 0123 321");

    for x in 0..5 {
        print!("{}", x);
        assert_eq!(lexer.token, LiteralInteger);
        lexer.advance();
    }
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
    assert_eq!(lexer.token, LiteralExponent);
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
fn date_and_time_literals_test() {
    let mut lexer = lex("DATE_AND_TIME#1984-10-01-20:15:12 DT#1-1-1-1:1:1 DT#1-1-1-1:1:1.123");
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
    assert_eq!(lexer.token, LiteralDateAndTime);
    lexer.advance();
}

#[test]
fn time_of_day_literals_test() {
    let mut lexer = lex("TIME_OF_DAY#20:15:12 TOD#1:1:1 TOD#1:1:1.123");
    assert_eq!(lexer.token, LiteralTimeOfDay);
    lexer.advance();
    assert_eq!(lexer.token, LiteralTimeOfDay);
    lexer.advance();
    assert_eq!(lexer.token, LiteralTimeOfDay);
    lexer.advance();
}

#[test]
fn time_literals_test() {
    let mut lexer = lex(r#"
    T#12d T#13h TIME#14m TIME#15s T#16ms
    T#12d10ms T#12h10m TIME#12m4s3ns
    TIME#4d6h8m7s12ms04us2ns
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
    let mut lexer = lex(r".. .");

    assert_eq!(lexer.token, KeywordDotDot);
    lexer.advance();
    assert_eq!(lexer.token, KeywordDot);
    lexer.advance();
}

#[test]
fn range_statements() {
    let mut lexer = lex(r"123..ABC");

    println!("{:?}", lexer.token);
    lexer.advance();
    println!("{:?}", lexer.token);
    lexer.advance();
    println!("{:?}", lexer.token);
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
