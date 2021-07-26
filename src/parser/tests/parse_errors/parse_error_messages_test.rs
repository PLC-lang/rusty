use crate::parser::parse;
use crate::Diagnostic;
use pretty_assertions::*;

#[test]
fn test_unexpected_token_error_message() {
    let source = "PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ";
    let lexer = super::super::lex(source);
    let (_, diagnostics) = parse(lexer);

    assert_eq!(
        format!("{:?}", diagnostics),
        format!(
            "{:?}",
            vec![Diagnostic::unexpected_token_found(
                "KeywordEndVar".into(),
                "';'".into(),
                (32..33).into()
            ),]
        )
    );
}

#[test]
fn test_unexpected_token_error_message2() {
    let lexer = super::super::lex(
        "SOME PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ",
    );
    let parse_result = parse(lexer);
    assert_eq!(
        &Diagnostic::syntax_error(
            "Unexpected token: expected StartKeyword but found SOME".into(),
            (0..4).into()
        ),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn for_with_unexpected_token_1() {
    let lexer = super::super::lex(
        "
        PROGRAM exp 
        FOR z ALPHA x TO y DO
            x;
            y;
        END_FOR
        END_PROGRAM
        ",
    );
    let parse_result = parse(lexer);
    assert_eq!(
        &Diagnostic::syntax_error(
            "Unexpected token: expected KeywordAssignment but found ALPHA".into(),
            (36..41).into()
        ),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn for_with_unexpected_token_2() {
    let lexer = super::super::lex(
        "
        PROGRAM exp 
        FOR z := x BRAVO y DO
            x;
            y;
        END_FOR
        END_PROGRAM
        ",
    );
    let parse_result = parse(lexer);
    assert_eq!(
        &Diagnostic::syntax_error(
            "Unexpected token: expected KeywordTo but found BRAVO".into(),
            (41..46).into()
        ),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn case_with_unexpected_token() {
    let lexer = super::super::lex(
        "
        PROGRAM exp 
        CASE StateMachine DELTA
        1: x;
        END_CASE
        END_PROGRAM
        ",
    );
    let parse_result = parse(lexer);
    assert_eq!(
        &Diagnostic::syntax_error(
            "Unexpected token: expected KeywordOf but found DELTA".into(),
            (48..53).into()
        ),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn test_unclosed_body_error_message() {
    let lexer = super::super::lex(
        "
            
            PROGRAM My_PRG

    ",
    );
    let (_, diagnostics) = parse(lexer);

    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndProgram".into(),
            "''".into(),
            (46..46).into()
        )]
    );
}
