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

#[test]
fn test_incomplete_var_config_block() {
    let src = "

            VAR_CONFIG
                // instance1;
                instance2.bar AT;
                instance3.bar AT %IX3.1;
                instance4.bar AT %IX3.1 : BOOL
                instance5.bar : BOOL;
                AT %IX3.1
                %IX3.1
            END_VAR


    ";
    let diagnostics = parse_and_validate_buffered(src);
    assert_snapshot!(diagnostics, @r###"
    error[E006]: Missing expected Token hardware access
      ┌─ <internal>:5:33
      │
    5 │                 instance2.bar AT;
      │                                 ^ Missing expected Token hardware access

    error[E006]: Missing expected Token KeywordColon
      ┌─ <internal>:6:40
      │
    6 │                 instance3.bar AT %IX3.1;
      │                                        ^ Missing expected Token KeywordColon

    error[E007]: Unexpected token: expected DataTypeDefinition but found KeywordSemicolon
      ┌─ <internal>:6:40
      │
    6 │                 instance3.bar AT %IX3.1;
      │                                        ^ Unexpected token: expected DataTypeDefinition but found KeywordSemicolon

    error[E007]: Unexpected token: expected KeywordSemicolon but found 'instance5.bar : BOOL'
      ┌─ <internal>:8:17
      │
    8 │                 instance5.bar : BOOL;
      │                 ^^^^^^^^^^^^^^^^^^^^ Unexpected token: expected KeywordSemicolon but found 'instance5.bar : BOOL'

    error[E007]: Unexpected token: expected KeywordEndVar but found 'AT %IX3.1
                    %IX3.1'
       ┌─ <internal>:9:17
       │  
     9 │ ╭                 AT %IX3.1
    10 │ │                 %IX3.1
       │ ╰──────────────────────^ Unexpected token: expected KeywordEndVar but found 'AT %IX3.1
                    %IX3.1'

    "###);
}
