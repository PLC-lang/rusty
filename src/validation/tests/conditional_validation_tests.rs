use crate::test_utils::tests::parse_and_validate_buffered;
use insta::assert_snapshot;

#[test]
fn undefined_reference_in_conditional() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE ErrorCode :
        STRUCT
            Lib     : UINT;     (**< Error library code *)
            Reason  : UINT;     (**< Error reason code *)
            Func    : UDINT;    (**< Error function code *)
        END_STRUCT
        END_TYPE

        PROGRAM mainProg
        VAR
            Error: ErrorCode;
        END_VAR

        IF Error.Reason <> ERROR_OK THEN // unresolved reference in conditional

        END_IF
        END_PROGRAM
        ",
    );

    assert_snapshot!(diagnostics, @"
    error[E048]: Could not resolve reference to ERROR_OK
       ┌─ <internal>:15:28
       │
    15 │         IF Error.Reason <> ERROR_OK THEN // unresolved reference in conditional
       │                            ^^^^^^^^ Could not resolve reference to ERROR_OK
    ");
}

#[test]
fn undefined_reference_with_nested_binary_expression_in_conditional() {
    let diagnostics = parse_and_validate_buffered(
        "
        TYPE myEnum : (
            GO,
            STOP
        );
        END_TYPE

        FUNCTION main : DINT
        VAR
            count: DINT;
        END_VAR
            WHILE (state = GO OR GO = GO) DO // must be a nested binary expression to reproduce, hence the `GO = GO`
                count := count +1;
            END_WHILE;
        END_FUNCTION
        ",
    );

    assert_snapshot!(diagnostics, @"
    error[E048]: Could not resolve reference to state
       ┌─ <internal>:12:20
       │
    12 │             WHILE (state = GO OR GO = GO) DO // must be a nested binary expression to reproduce, hence the `GO = GO`
       │                    ^^^^^ Could not resolve reference to state
    ");
}
