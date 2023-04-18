use crate::test_utils::tests::codegen;

#[test]
fn internal_vla_struct_is_generated_for_call_statements() {
    let res = codegen(
        r#"
        FUNCTION foo
        VAR_INPUT
            vla : ARRAY[*] OF INT;
        END_VAR
        END_FUNCTION

        FUNCTION bar
        VAR
            arr : ARRAY[0..1] OF INT;
        END_VAR
            foo(arr);
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(res);
}

#[test]
fn vla_read_access() {
    let res = codegen(
        r#"
        FUNCTION foo : INT
        VAR_INPUT
            vla : ARRAY[*] OF INT;
        END_VAR
            FOO := vla[0];
        END_FUNCTION

        FUNCTION main : DINT
        VAR
            arr : ARRAY[0..1] OF INT;
        END_VAR
            foo(arr);
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(res);
}

#[test]
fn global_variable_passed_to_function_as_vla() {
    let res = codegen(
        r#"
        VAR_GLOBAL
            arr : ARRAY[0..1] OF INT;
        END_VAR

        FUNCTION foo : INT
        VAR_INPUT
            vla : ARRAY[*] OF INT;
        END_VAR
            vla[0] := 10;
        END_FUNCTION

        FUNCTION main : DINT
            foo(arr);
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(res);
}
