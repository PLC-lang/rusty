// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn pointers_in_function_return() {
    let result = codegen(
        r#"FUNCTION func : REF_TO INT
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn structs_in_function_return() {
    let result = codegen(
        r#"
        TYPE myStruct : STRUCT
            x : INT;
            END_STRUCT
        END_TYPE
        FUNCTION func : myStruct
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn unary_expressions_can_be_real() {
    let result = codegen(
        r#"
            PROGRAM prg
            VAR
                a,b : REAL;
            END_VAR
                b := -2.0;
                a := -b;
            END_PROGRAM
        "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn type_mix_in_call() {
    let result = codegen(
        "
        FUNCTION foo : INT
        VAR_INPUT
            in : INT;
        END_VAR
        END_FUNCTION
        FUNCTION baz : INT
            foo(1.5);
        END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}
