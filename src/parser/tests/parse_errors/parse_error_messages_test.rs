use crate::test_utils::tests::parse;
use crate::Diagnostic;
use pretty_assertions::*;

#[test]
fn test_unexpected_token_error_message() {
    let source = "PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ";
    let (_, diagnostics) = parse(source);

    assert_eq!(
        format!("{:?}", diagnostics),
        format!(
            "{:?}",
            vec![Diagnostic::unexpected_token_found(
                "KeywordEndVar",
                "';'",
                (32..33).into()
            ),]
        )
    );
}

#[test]
fn test_unexpected_token_error_message2() {
    let src = "SOME PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ";
    let parse_result = parse(src);
    assert_eq!(
        &Diagnostic::unexpected_token_found("StartKeyword", "SOME", (0..4).into()),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn for_with_unexpected_token_1() {
    let src = "
        PROGRAM exp 
        FOR z ALPHA x TO y DO
            x;
            y;
        END_FOR
        END_PROGRAM
        ";
    let parse_result = parse(src);
    assert_eq!(
        &Diagnostic::unexpected_token_found("KeywordAssignment", "ALPHA", (36..41).into()),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn for_with_unexpected_token_2() {
    let src = "
        PROGRAM exp 
        FOR z := x BRAVO y DO
            x;
            y;
        END_FOR
        END_PROGRAM
        ";
    let parse_result = parse(src);
    assert_eq!(
        &Diagnostic::unexpected_token_found("KeywordTo", "BRAVO", (41..46).into()),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn if_then_with_unexpected_token() {
    let src = "
        PROGRAM exp 
        IF TRUE CHARLIE
            x;
        ELSE
            y;
        END_IF
        END_PROGRAM
        ";
    let parse_result = parse(src);

    assert_eq!(
        &Diagnostic::unexpected_token_found("KeywordThen", "CHARLIE", (38..45).into()),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn case_with_unexpected_token() {
    let src = "
        PROGRAM exp 
        CASE StateMachine DELTA
        1: x;
        END_CASE
        END_PROGRAM
        ";
    let parse_result = parse(src);
    assert_eq!(
        &Diagnostic::unexpected_token_found("KeywordOf", "DELTA", (48..53).into()),
        parse_result.1.first().unwrap()
    );
}

#[test]
fn test_unclosed_body_error_message() {
    let src = "
            
            PROGRAM My_PRG

    ";
    let (_, diagnostics) = parse(src);

    assert_eq!(
        diagnostics,
        vec![Diagnostic::unexpected_token_found(
            "KeywordEndProgram",
            "''",
            (46..46).into()
        )]
    );
}
