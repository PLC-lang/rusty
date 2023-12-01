use crate::{diagnostics::Diagnostic, test_utils::tests::codegen_without_unwrap};
use pretty_assertions::assert_eq;

/*
SWITCH CASE duplicate integer tests

`module.verify()` can handle [x] following cases

+--------------------------------------------------------------------------------------------------------+
|                    | non const var ref  | literal integer    |       const        | binary expression  |
|--------------------------------------------------------------------------------------------------------|
| non const var ref  |                    |                    |                    |                    |
|--------------------------------------------------------------------------------------------------------|
| literal integer    |                    |         X          |         X          |         X          |
|--------------------------------------------------------------------------------------------------------|
|       const        |                    |         X          |         X          |         X          |
|--------------------------------------------------------------------------------------------------------|
| binary expression  |                    |         X          |         X          |         X          |
+--------------------------------------------------------------------------------------------------------+

For non const variable references we need our own validation tests -> statement_validation_tests.rs
*/

#[test]
fn switch_case_duplicate_integer_literal_integer() {
    let result = codegen_without_unwrap(
        r#"
        PROGRAM mainProg
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                2:
                    res := 1;
                2:
                    res := 2;
            END_CASE
        END_PROGRAM
        "#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::GeneralError {
                message: "Duplicate integer as switch case\n  switch i32 %load_input, label %else [\n    i32 2, label %case\n    i32 2, label %case1\n  ]\ni32 2\n".into(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            },
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn switch_case_duplicate_integer_literal_integer_and_const() {
    let result = codegen_without_unwrap(
        r#"
        VAR_GLOBAL CONSTANT
            GLOB : DINT := 2;
        END_VAR

        PROGRAM mainProg
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                2:
                    res := 1;
                GLOB:
                    res := 2;
            END_CASE
        END_PROGRAM
        "#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::GeneralError {
                message: "Duplicate integer as switch case\n  switch i32 %load_input, label %else [\n    i32 2, label %case\n    i32 2, label %case1\n  ]\ni32 2\n".into(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            },
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn switch_case_duplicate_integer_literal_integer_and_binary_expression() {
    let result = codegen_without_unwrap(
        r#"
        PROGRAM mainProg
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                2:
                    res := 1;
                1*2:
                    res := 2;
            END_CASE
        END_PROGRAM
        "#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::GeneralError {
                message: "Duplicate integer as switch case\n  switch i32 %load_input, label %else [\n    i32 2, label %case\n    i32 2, label %case1\n  ]\ni32 2\n".into(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            },
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn switch_case_duplicate_integer_const() {
    let result = codegen_without_unwrap(
        r#"
        VAR_GLOBAL CONSTANT
            GLOB : DINT := 2;
        END_VAR

        TYPE myType: ( BASE := GLOB ); END_TYPE

        PROGRAM mainProg
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                GLOB:
                    res := 1;
                BASE:
                    res := 2;
            END_CASE
        END_PROGRAM
        "#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::GeneralError {
                message: "Duplicate integer as switch case\n  switch i32 %load_input, label %else [\n    i32 2, label %case\n    i32 2, label %case1\n  ]\ni32 2\n".into(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            },
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn switch_case_duplicate_integer_const_and_binary_expression() {
    let result = codegen_without_unwrap(
        r#"
        VAR_GLOBAL CONSTANT
            GLOB : DINT := 2;
        END_VAR

        PROGRAM mainProg
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                GLOB:
                    res := 1;
                1*2:
                    res := 2;
            END_CASE
        END_PROGRAM
        "#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::GeneralError {
                message: "Duplicate integer as switch case\n  switch i32 %load_input, label %else [\n    i32 2, label %case\n    i32 2, label %case1\n  ]\ni32 2\n".into(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            },
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}

#[test]
fn switch_case_duplicate_integer_binary_expression() {
    let result = codegen_without_unwrap(
        r#"
        PROGRAM mainProg
        VAR
            input, res : DINT;
        END_VAR
            CASE input OF
                1*2:
                    res := 1;
                1+1:
                    res := 2;
            END_CASE
        END_PROGRAM
        "#,
    );
    if let Err(msg) = result {
        assert_eq!(
            Diagnostic::GeneralError {
                message: "Duplicate integer as switch case\n  switch i32 %load_input, label %else [\n    i32 2, label %case\n    i32 2, label %case1\n  ]\ni32 2\n".into(),
                err_no: crate::diagnostics::ErrNo::codegen__general,
            },
            msg
        )
    } else {
        panic!("expected code-gen error but got none")
    }
}
