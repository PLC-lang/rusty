use insta::assert_snapshot;

use crate::test_utils::tests::codegen;

#[test]
fn simple_global() {
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

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "###);
}

#[test]
fn global_alias() {
    let result = codegen(
        r#"
        VAR_GLOBAL
            s: STRING := 'hello world!';
            ps AT s : STRING;
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      %deref = load [81 x i8]*, [81 x i8]** @ps, align 8
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "###);
}


#[test]
fn global_reference_to() {
    let result = codegen(
      r#"
      VAR_GLOBAL
        s: STRING := 'hello world!';
        ps: REFERENCE TO STRING := REF(s);
      END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      %deref = load [81 x i8]*, [81 x i8]** @ps, align 8
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "###);
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

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %PLC_PRG = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer, section "var-$RUSTY$s:s8u81"
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"

    define void @PLC_PRG(%PLC_PRG* %0) section "fn-$RUSTY$PLC_PRG:v" {
    entry:
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }

    define void @__init_plc_prg(%PLC_PRG* %0) section "fn-$RUSTY$__init_plc_prg:v[pr1ps8u81]" {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      %deref = load %PLC_PRG*, %PLC_PRG** %self, align 8
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }
    "###);
}

#[test]
fn init_functions_generated_for_function_blocks() {
    let result = codegen(
        r#"
        VAR_GLOBAL 
            s: STRING;
        END_VAR

        FUNCTION_BLOCK foo
        VAR
            to_init: REF_TO STRING := REF(s);
        END_VAR    
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %foo = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer, section "var-$RUSTY$s:s8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r1ps8u81"

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %to_init = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %to_init = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      ret void
    }
    "###);
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
        
        FUNCTION main : DINT
            __init();
            mainProg();
            sideProg();
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %foo = type { [81 x i8]*, %bar }
    %bar = type { [81 x i8]*, %baz }
    %baz = type { [81 x i8]* }
    %mainProg = type { [81 x i8]*, %foo }
    %sideProg = type { [81 x i8]*, %foo }

    @str = global [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$str:s8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r2ps8u81r2ps8u81r1ps8u81"
    @__bar__init = unnamed_addr constant %bar zeroinitializer, section "var-$RUSTY$__bar__init:r2ps8u81r1ps8u81"
    @__baz__init = unnamed_addr constant %baz zeroinitializer, section "var-$RUSTY$__baz__init:r1ps8u81"
    @mainProg_instance = global %mainProg zeroinitializer, section "var-$RUSTY$mainProg_instance:r2ps8u81r2ps8u81r2ps8u81r1ps8u81"
    @sideProg_instance = global %sideProg zeroinitializer, section "var-$RUSTY$sideProg_instance:r2ps8u81r2ps8u81r2ps8u81r1ps8u81"

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %str_ref = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      call void @bar.print(%bar* %b)
      call void @bar(%bar* %b)
      ret void
    }

    define void @bar(%bar* %0) section "fn-$RUSTY$bar:v" {
    entry:
      %str_ref = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 1
      call void @baz.print(%baz* %b)
      ret void
    }

    define void @baz(%baz* %0) section "fn-$RUSTY$baz:v" {
    entry:
      %str_ref = getelementptr inbounds %baz, %baz* %0, i32 0, i32 0
      ret void
    }

    define void @mainProg(%mainProg* %0) section "fn-$RUSTY$mainProg:v" {
    entry:
      %other_ref_to_global = getelementptr inbounds %mainProg, %mainProg* %0, i32 0, i32 0
      %f = getelementptr inbounds %mainProg, %mainProg* %0, i32 0, i32 1
      ret void
    }

    define void @sideProg(%sideProg* %0) section "fn-$RUSTY$sideProg:v" {
    entry:
      %other_ref_to_global = getelementptr inbounds %sideProg, %sideProg* %0, i32 0, i32 0
      %f = getelementptr inbounds %sideProg, %sideProg* %0, i32 0, i32 1
      call void @foo(%foo* %f)
      call void @foo.print(%foo* %f)
      ret void
    }

    define i32 @main() section "fn-$RUSTY$main:i32" {
    entry:
      %main = alloca i32, align 4
      store i32 0, i32* %main, align 4
      call void @__init()
      call void @mainProg(%mainProg* @mainProg_instance)
      call void @sideProg(%sideProg* @sideProg_instance)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    define void @bar.print(%bar* %0) section "fn-$RUSTY$bar.print:v" {
    entry:
      %str_ref = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 1
      ret void
    }

    define void @foo.print(%foo* %0) section "fn-$RUSTY$foo.print:v" {
    entry:
      %str_ref = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @baz.print(%baz* %0) section "fn-$RUSTY$baz.print:v" {
    entry:
      %str_ref = getelementptr inbounds %baz, %baz* %0, i32 0, i32 0
      ret void
    }

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      call void @__init_mainprog(%mainProg* @mainProg_instance)
      call void @__init_sideprog(%sideProg* @sideProg_instance)
      ret void
    }

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr2ps8u81r2ps8u81r1ps8u81]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %str_ref = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %str_ref, align 8
      %deref1 = load %foo*, %foo** %self, align 8
      %b = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 1
      call void @__init_bar(%bar* %b)
      ret void
    }

    define void @__init_bar(%bar* %0) section "fn-$RUSTY$__init_bar:v[pr2ps8u81r1ps8u81]" {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %str_ref = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %str_ref, align 8
      %deref1 = load %bar*, %bar** %self, align 8
      %b = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 1
      call void @__init_baz(%baz* %b)
      ret void
    }

    define void @__init_baz(%baz* %0) section "fn-$RUSTY$__init_baz:v[pr1ps8u81]" {
    entry:
      %self = alloca %baz*, align 8
      store %baz* %0, %baz** %self, align 8
      %deref = load %baz*, %baz** %self, align 8
      %str_ref = getelementptr inbounds %baz, %baz* %deref, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %str_ref, align 8
      ret void
    }

    define void @__init_mainprog(%mainProg* %0) section "fn-$RUSTY$__init_mainprog:v[pr2ps8u81r2ps8u81r2ps8u81r1ps8u81]" {
    entry:
      %self = alloca %mainProg*, align 8
      store %mainProg* %0, %mainProg** %self, align 8
      %deref = load %mainProg*, %mainProg** %self, align 8
      %other_ref_to_global = getelementptr inbounds %mainProg, %mainProg* %deref, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %other_ref_to_global, align 8
      %deref1 = load %mainProg*, %mainProg** %self, align 8
      %f = getelementptr inbounds %mainProg, %mainProg* %deref1, i32 0, i32 1
      call void @__init_foo(%foo* %f)
      ret void
    }

    define void @__init_sideprog(%sideProg* %0) section "fn-$RUSTY$__init_sideprog:v[pr2ps8u81r2ps8u81r2ps8u81r1ps8u81]" {
    entry:
      %self = alloca %sideProg*, align 8
      store %sideProg* %0, %sideProg** %self, align 8
      %deref = load %sideProg*, %sideProg** %self, align 8
      %other_ref_to_global = getelementptr inbounds %sideProg, %sideProg* %deref, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %other_ref_to_global, align 8
      %deref1 = load %sideProg*, %sideProg** %self, align 8
      %f = getelementptr inbounds %sideProg, %sideProg* %deref1, i32 0, i32 1
      call void @__init_foo(%foo* %f)
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
    "#,
    );

    assert_snapshot!(res, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { i64, [81 x i8]* }

    @str = global [81 x i8] zeroinitializer, section "var-$RUSTY$str:s8u81"
    @prg_instance = global %prg zeroinitializer, section "var-$RUSTY$prg_instance:r2i64ps8u81"

    define void @prg(%prg* %0) section "fn-$RUSTY$prg:v" {
    entry:
      %a = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %b = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      ret void
    }

    define void @__init_prg(%prg* %0) section "fn-$RUSTY$__init_prg:v[pr2i64ps8u81]" {
    entry:
      %self = alloca %prg*, align 8
      store %prg* %0, %prg** %self, align 8
      %deref = load %prg*, %prg** %self, align 8
      %b = getelementptr inbounds %prg, %prg* %deref, i32 0, i32 1
      store [81 x i8]* @str, [81 x i8]** %b, align 8
      ret void
    }

    define void @__init() section "fn-$RUSTY$__init:v" {
    entry:
      call void @__init_prg(%prg* @prg_instance)
      ret void
    }
    "###);
}

#[test]
fn local_address() {
    let res = codegen(
        r#"      
        FUNCTION_BLOCK foo
        VAR
            i : INT;
            pi: REF_TO INT := REF(i);
        END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
fn needs_validation() {
    let res = codegen(
        r#"      
        VAR_GLOBAL
          s : STRING;
        END_VAR

        FUNCTION_BLOCK foo
        VAR
            pi: REF_TO INT := REF(s); // not an int-pointer
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar 
        VAR_INPUT
            st: STRING;
        END_VAR
        VAR
            ps: LWORD := ADR(st); // address of an input-variable
        END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}
