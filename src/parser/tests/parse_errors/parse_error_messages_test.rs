use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn test_unexpected_token_error_message() {
    let source = "PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ";
    let diagnostics = parse_and_validate_buffered(source);
    assert_snapshot!(diagnostics);
}

#[test]
fn test_unexpected_token_error_message2() {
    let src = "SOME PROGRAM prg
                VAR ;
                END_VAR
            END_PROGRAM
    ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
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
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
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
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
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
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
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
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}

#[test]
fn test_unclosed_body_error_message() {
    let src = "
            
            PROGRAM My_PRG

    ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics);
}
