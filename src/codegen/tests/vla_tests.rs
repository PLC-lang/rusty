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
fn multi_dim_vla() {
    let res = codegen(
        r#"
        FUNCTION foo
        VAR_INPUT
            vla : ARRAY[*, *] OF INT;
        END_VAR
        END_FUNCTION

        FUNCTION bar
        VAR
            arr : ARRAY[0..1, 6..12] OF INT;
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
        FUNCTION foo
        VAR_INPUT
            vla : ARRAY[*] OF INT;
        END_VAR
        VAR_TEMP
            i : INT;
        END_VAR
            i := vla[0];
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
