use crate::test_utils::tests::codegen;

#[test]
fn simple() {
    let result = codegen(
        r#"
        VAR_GLOBAL
            s: STRING := 'hello world!';
            ps: REF_TO STRING := REF(s);
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"
    "###);
}

#[test]
fn init_fn_test() {
    let result = codegen(
        r#"
        PROGRAM PLC_PRG
        VAR
            s: STRING;
            to_init: REF_TO STRING := REF(s);
        END_VAR    
        END_PROGRAM

        FUNCTION_BLOCK foo
        VAR
            s: STRING;
            to_init: REF_TO STRING := REF(s);
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

#[test]
fn dependencies() {
    let result = codegen(
        r#"
        // __PLC_PRG_init => has dependency on __foo_init
        PROGRAM PLC_PRG
        VAR
            fb: foo;
        END_VAR    
        END_PROGRAM

        // __foo_init => has dependency on __bar_init
        FUNCTION_BLOCK foo
        VAR
            fb: bar;
        END_VAR    
        END_FUNCTION_BLOCK

        // __bar_init => has dependency on __global_ps_init => globals which are not in scope of another POU should be initialized first!
        FUNCTION_BLOCK bar
        VAR
            ps2: REF_TO STRING := ps;
        END_VAR
        END_FUNCTION_BLOCK

        VAR_GLOBAL
            s: STRING;
            ps: REF_TO STRING := REF(s);

            // // ... cyclic dependency? ignore for now, will probably need to be validated
            // fb_global: bar;
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, r###""###);
}
