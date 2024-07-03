use crate::test_utils::tests::codegen;

#[test]
fn init_fn_test() {
    let result = codegen(
        r#"
        PROGRAM PLC_PRG
        VAR
            s: STRING;
            hard_to_init_innit: REF_TO STRING := REF(s);
        END_VAR    
        END_PROGRAM

        FUNCTION_BLOCK foo
        VAR
            s: STRING;
            hard_to_init_innit: REF_TO STRING := REF(s);
        END_VAR    
        END_FUNCTION_BLOCK

        VAR_GLOBAL 
            s: STRING;
            ps: REF_TO STRING := REF(s);
            bar: foo;
        END_VAR

        "#,
    );

    insta::assert_snapshot!(result, r###""###);
}