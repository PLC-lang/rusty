use crate::ast::SourceRange;
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
                SourceRange::new(32..33,Some(2),Some(21),Some(2),Some(22))
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
        &Diagnostic::unexpected_token_found("StartKeyword", "SOME", SourceRange::new(0..4, Some(1), Some(0),Some(1),Some(4))),
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
        &Diagnostic::unexpected_token_found("KeywordAssignment", "ALPHA", SourceRange::new(36..41,Some(3),Some(15),Some(3),Some(20))),
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
        &Diagnostic::unexpected_token_found("KeywordTo", "BRAVO", SourceRange::new(41..46,Some(3),Some(20),Some(3),Some(25))),
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
        &Diagnostic::unexpected_token_found("KeywordThen", "CHARLIE", SourceRange::new(38..45,Some(3),Some(17),Some(3),Some(24))),
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
        &Diagnostic::unexpected_token_found("KeywordOf", "DELTA", SourceRange::new(48..53,Some(3),Some(27),Some(3),Some(32))),
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
            SourceRange::new(46..46,Some(5),Some(5),Some(5),Some(5))
        )]
    );
}
