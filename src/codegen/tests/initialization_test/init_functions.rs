use insta::assert_snapshot;

use crate::test_utils::tests::codegen;

#[test]
#[ignore = "VAR_GLOBAL blocks not yet supported"]
fn simple_global() {
    let result = codegen(
        r#"
        VAR_GLOBAL
            s: STRING := 'hello world!';
            ps: REF_TO STRING := REF(s);
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###""###);
}

#[test]
fn init_functions_generated_for_programs() {
    let result = codegen(
        r#"
        PROGRAM PLC_PRG
        VAR
            to_init: REF_TO STRING := REF(s);
        END_VAR    
        END_PROGRAM

        VAR_GLOBAL 
            s: STRING;
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###""###);
}

#[test]
fn init_functions_generated_for_function_blocks() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR
            to_init: REF_TO STRING := REF(s);
        END_VAR    
        END_PROGRAM

        VAR_GLOBAL 
            s: STRING;
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###""###);
}


#[test]
fn nested_initializer_pous() {
    let result = codegen(
        r#"
        VAR_GLOBAL 
            str : STRING := 'hello';
        END_VAR

        FUNCTION_BLOCK foo
        VAR 
            str_ref : REF_TO STRING := REF(str);
            b: bar;
        END_VAR
            b.print();
            b();
        END_FUNCTION_BLOCK

        ACTION foo.print
            // do something
        END_ACTION

        FUNCTION_BLOCK bar
        VAR 
            str_ref : REF_TO STRING := REF(str);
            b: baz;
        END_VAR
            b.print();
        END_FUNCTION_BLOCK

        ACTION bar.print
            // do something
        END_ACTION

        FUNCTION_BLOCK baz
        VAR 
            str_ref : REF_TO STRING := REF(str);
        END_VAR
        END_FUNCTION_BLOCK

        ACTION baz.print
            // do something
        END_ACTION

        PROGRAM mainProg
        VAR
            other_ref_to_global: REF_TO STRING := REF(str);
            f: foo;
        END_VAR
            // do something   
        END_PROGRAM

        PROGRAM sideProg
        VAR
            other_ref_to_global: REF_TO STRING := REF(str);
            f: foo;
        END_VAR
            f();
            f.print();
        END_PROGRAM

        PROGRAM aliasProg 
        VAR
            s2 : REFERENCE TO STRING := str;
            s AT str : STRING;
        END_VAR
            // do something
        END_PROGRAM
        
        FUNCTION main : DINT
            __init();
            mainProg();
            sideProg();
            aliasProg();
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %PLC_PRG = type { %foo }
    %foo = type { %bar }
    %bar = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer, section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, section "var-$RUSTY$PLC_PRG_instance:r1r1r1ps8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r1r1ps8u81"
    @__bar__init = unnamed_addr constant %bar zeroinitializer, section "var-$RUSTY$__bar__init:r1ps8u81"

    define void @PLC_PRG(%PLC_PRG* %0) section "fn-$RUSTY$PLC_PRG:v" {
    entry:
      %fb = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %fb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @bar(%bar* %0) section "fn-$RUSTY$bar:v" {
    entry:
      %ps2 = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void
    }
    "###);
}

#[test]
fn edge_case() {
    let res = codegen(
        r#"
        VAR_GLOBAL 
            str : STRING;
        END_VAR

        PROGRAM prg
        VAR
            a : DATE := D#2001-02-29; // feb29 on non-leap year should not pass 
            b : REF_TO STRING := REF(str); // pou has an init function
        END_VAR
        END_PROGRAM
    "#
    );

    assert_snapshot!(res, @r###""###);
}