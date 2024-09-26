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
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"
    @ps = external global [81 x i8]*, section "var-$RUSTY$ps:ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
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
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"
    @ps = external global [81 x i8]*, section "var-$RUSTY$ps:ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
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
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @ps = global [81 x i8]* null, section "var-$RUSTY$ps:ps8u81"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"
    @ps = external global [81 x i8]*, section "var-$RUSTY$ps:ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
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
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %PLC_PRG = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer, section "var-$RUSTY$s:s8u81"
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"

    define void @PLC_PRG(%PLC_PRG* %0) section "fn-$RUSTY$PLC_PRG:v" {
    entry:
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"
    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"

    define void @__init_plc_prg(%PLC_PRG* %0) section "fn-$RUSTY$__init_plc_prg:v[pr1ps8u81]" {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      %deref = load %PLC_PRG*, %PLC_PRG** %self, align 8
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    declare void @PLC_PRG(%PLC_PRG*) section "fn-$RUSTY$PLC_PRG:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }

    declare void @__init_plc_prg(%PLC_PRG*) section "fn-$RUSTY$__init_plc_prg:v[pr1ps8u81]"

    declare void @PLC_PRG(%PLC_PRG*) section "fn-$RUSTY$PLC_PRG:v"
    "###);
}

#[test]
fn init_functions_work_with_adr() {
    let result = codegen(
        r#"
        PROGRAM PLC_PRG
        VAR
            to_init: REF_TO STRING := ADR(s);
        END_VAR    
        END_PROGRAM

        VAR_GLOBAL 
            s: STRING;
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %PLC_PRG = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer, section "var-$RUSTY$s:s8u81"
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"

    define void @PLC_PRG(%PLC_PRG* %0) section "fn-$RUSTY$PLC_PRG:v" {
    entry:
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"
    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"

    define void @__init_plc_prg(%PLC_PRG* %0) section "fn-$RUSTY$__init_plc_prg:v[pr1ps8u81]" {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      %deref = load %PLC_PRG*, %PLC_PRG** %self, align 8
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    declare void @PLC_PRG(%PLC_PRG*) section "fn-$RUSTY$PLC_PRG:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG, section "var-$RUSTY$PLC_PRG_instance:r1ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }

    declare void @__init_plc_prg(%PLC_PRG*) section "fn-$RUSTY$__init_plc_prg:v[pr1ps8u81]"

    declare void @PLC_PRG(%PLC_PRG*) section "fn-$RUSTY$PLC_PRG:v"
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
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer, section "var-$RUSTY$s:s8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r1ps8u81"

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %to_init = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }

    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"
    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %to_init = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
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
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]*, %bar }
    %bar = type { %baz }
    %baz = type { [81 x i8]* }
    %mainProg = type { [81 x i8]*, %foo }
    %sideProg = type { [81 x i8]*, %foo }

    @str = global [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$str:s8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r2ps8u81r1r1ps8u81"
    @__bar__init = unnamed_addr constant %bar zeroinitializer, section "var-$RUSTY$__bar__init:r1r1ps8u81"
    @__baz__init = unnamed_addr constant %baz zeroinitializer, section "var-$RUSTY$__baz__init:r1ps8u81"
    @mainProg_instance = global %mainProg zeroinitializer, section "var-$RUSTY$mainProg_instance:r2ps8u81r2ps8u81r1r1ps8u81"
    @sideProg_instance = global %sideProg zeroinitializer, section "var-$RUSTY$sideProg_instance:r2ps8u81r2ps8u81r1r1ps8u81"

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
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
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

    define void @bar.print(%bar* %0) section "fn-$RUSTY$bar.print:v" {
    entry:
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
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
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]*, %bar }
    %bar = type { %baz }
    %baz = type { [81 x i8]* }
    %mainProg = type { [81 x i8]*, %foo }
    %sideProg = type { [81 x i8]*, %foo }

    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r2ps8u81r1r1ps8u81"
    @__bar__init = external global %bar, section "var-$RUSTY$__bar__init:r1r1ps8u81"
    @__baz__init = external global %baz, section "var-$RUSTY$__baz__init:r1ps8u81"
    @mainProg_instance = external global %mainProg, section "var-$RUSTY$mainProg_instance:r2ps8u81r2ps8u81r1r1ps8u81"
    @sideProg_instance = external global %sideProg, section "var-$RUSTY$sideProg_instance:r2ps8u81r2ps8u81r1r1ps8u81"
    @str = external global [81 x i8], section "var-$RUSTY$str:s8u81"

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr2ps8u81r1r1ps8u81]" {
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

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    declare void @bar(%bar*) section "fn-$RUSTY$bar:v"

    declare void @baz(%baz*) section "fn-$RUSTY$baz:v"

    define void @__init_bar(%bar* %0) section "fn-$RUSTY$__init_bar:v[pr1r1ps8u81]" {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %b = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
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

    define void @__init_mainprog(%mainProg* %0) section "fn-$RUSTY$__init_mainprog:v[pr2ps8u81r2ps8u81r1r1ps8u81]" {
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

    declare void @mainProg(%mainProg*) section "fn-$RUSTY$mainProg:v"

    define void @__init_sideprog(%sideProg* %0) section "fn-$RUSTY$__init_sideprog:v[pr2ps8u81r2ps8u81r1r1ps8u81]" {
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

    declare void @sideProg(%sideProg*) section "fn-$RUSTY$sideProg:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %mainProg = type { [81 x i8]*, %foo }
    %foo = type { [81 x i8]*, %bar }
    %bar = type { %baz }
    %baz = type { [81 x i8]* }
    %sideProg = type { [81 x i8]*, %foo }

    @mainProg_instance = external global %mainProg, section "var-$RUSTY$mainProg_instance:r2ps8u81r2ps8u81r1r1ps8u81"
    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r2ps8u81r1r1ps8u81"
    @__bar__init = external global %bar, section "var-$RUSTY$__bar__init:r1r1ps8u81"
    @__baz__init = external global %baz, section "var-$RUSTY$__baz__init:r1ps8u81"
    @sideProg_instance = external global %sideProg, section "var-$RUSTY$sideProg_instance:r2ps8u81r2ps8u81r1r1ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_mainprog(%mainProg* @mainProg_instance)
      call void @__init_sideprog(%sideProg* @sideProg_instance)
      ret void
    }

    declare void @__init_mainprog(%mainProg*) section "fn-$RUSTY$__init_mainprog:v[pr2ps8u81r2ps8u81r1r1ps8u81]"

    declare void @mainProg(%mainProg*) section "fn-$RUSTY$mainProg:v"

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    declare void @bar(%bar*) section "fn-$RUSTY$bar:v"

    declare void @baz(%baz*) section "fn-$RUSTY$baz:v"

    declare void @__init_sideprog(%sideProg*) section "fn-$RUSTY$__init_sideprog:v[pr2ps8u81r2ps8u81r1r1ps8u81]"

    declare void @sideProg(%sideProg*) section "fn-$RUSTY$sideProg:v"
    "###);
}

#[test]
#[ignore = "initializing references in same POU not yet supported"]
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
#[ignore = "initializing references in same POU not yet supported"]
fn tmpo() {
    let res = codegen(
        r#"      
        FUNCTION_BLOCK foo
        VAR
            i : INT;
            pi: REF_TO INT;
        END_VAR
        END_FUNCTION_BLOCK

        ACTION foo.init
          pi := REF(i);
        END_ACTION
        "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "stack-local vars not yet supported"]
fn stack_allocated_variables_are_initialized_in_pou_body() {
    let res = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR_TEMP
            st: STRING;
        END_VAR
        VAR
            ps AT st : STRING;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION bar
        VAR
            st: STRING;
            ps AT st : STRING;
        END_VAR
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "initializing references in same POU not yet supported"]
fn ref_to_input_variable() {
    let res = codegen(
        r#"        
    FUNCTION_BLOCK bar 
    VAR_INPUT
        st: STRING;
    END_VAR
    VAR
        ps: LWORD := REF(st);
    END_VAR
    END_FUNCTION_BLOCK  
    "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "initializing references in same POU not yet supported"]
fn ref_to_inout_variable() {
    let res = codegen(
        r#"        
    FUNCTION_BLOCK bar 
    VAR_IN_OUT
        st: STRING;
    END_VAR
    VAR
        ps: LWORD := REF(st);
    END_VAR
    END_FUNCTION_BLOCK  
    "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
fn struct_types() {
    let res = codegen(
        r#"      
      TYPE myStruct : STRUCT
              member : REF_TO STRING := REF(s);
              member2 AT s2 : ARRAY[0..1] OF STRING;
          END_STRUCT
      END_TYPE

      VAR_GLOBAL
          s : STRING := 'Hello world!';
          s2 : ARRAY[0..1] OF STRING := ['hello', 'world'];
      END_VAR

      PROGRAM prog 
      VAR 
          str: myStruct;
      END_VAR
      END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }

    @s = global [81 x i8] c"Hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", section "var-$RUSTY$s:s8u81"
    @s2 = global [2 x [81 x i8]] [[81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"world\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], section "var-$RUSTY$s2:as8u81"
    @prog_instance = global %prog zeroinitializer, section "var-$RUSTY$prog_instance:r1r2ps8u81pas8u81"
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, section "var-$RUSTY$__myStruct__init:r2ps8u81pas8u81"

    define void @prog(%prog* %0) section "fn-$RUSTY$prog:v" {
    entry:
      %str = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }
    %prog = type { %myStruct }

    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r2ps8u81pas8u81"
    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r1r2ps8u81pas8u81"
    @s = external global [81 x i8], section "var-$RUSTY$s:s8u81"
    @s2 = external global [2 x [81 x i8]], section "var-$RUSTY$s2:as8u81"

    define void @__init_mystruct(%myStruct* %0) section "fn-$RUSTY$__init_mystruct:v[pr2ps8u81pas8u81]" {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      %deref = load %myStruct*, %myStruct** %self, align 8
      %member = getelementptr inbounds %myStruct, %myStruct* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %member, align 8
      %deref1 = load %myStruct*, %myStruct** %self, align 8
      %member2 = getelementptr inbounds %myStruct, %myStruct* %deref1, i32 0, i32 1
      store [2 x [81 x i8]]* @s2, [2 x [81 x i8]]** %member2, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) section "fn-$RUSTY$__init_prog:v[pr1r2ps8u81pas8u81]" {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %str = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_mystruct(%myStruct* %str)
      ret void
    }

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }

    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r1r2ps8u81pas8u81"
    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r2ps8u81pas8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      ret void
    }

    declare void @__init_prog(%prog*) section "fn-$RUSTY$__init_prog:v[pr1r2ps8u81pas8u81]"

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    "###);
}

#[test]
fn stateful_pous_methods_and_structs_get_init_functions() {
    let res = codegen(
        r#"      
      TYPE myStruct : STRUCT
          END_STRUCT
      END_TYPE

      PROGRAM prog 
      VAR 
      END_VAR
      END_PROGRAM

      FUNCTION_BLOCK foo
        METHOD m
        END_METHOD
      END_FUNCTION_BLOCK

      CLASS cl
        METHOD m
        END_METHOD
      END_CLASS
      
      // no init function is expected for this action
      ACTION foo.act
      END_ACTION
      "#,
    );

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %prog = type {}
    %foo = type {}
    %cl = type {}
    %myStruct = type {}
    %foo.m = type {}
    %cl.m = type {}

    @prog_instance = global %prog zeroinitializer, section "var-$RUSTY$prog_instance:r0"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r0"
    @__cl__init = unnamed_addr constant %cl zeroinitializer, section "var-$RUSTY$__cl__init:r0"
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer, section "var-$RUSTY$__myStruct__init:r0"

    define void @prog(%prog* %0) section "fn-$RUSTY$prog:v" {
    entry:
      ret void
    }

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      ret void
    }

    define void @foo.m(%foo* %0, %foo.m* %1) section "fn-$RUSTY$foo.m:v" {
    entry:
      ret void
    }

    define void @cl(%cl* %0) section "fn-$RUSTY$cl:v" {
    entry:
      ret void
    }

    define void @cl.m(%cl* %0, %cl.m* %1) section "fn-$RUSTY$cl.m:v" {
    entry:
      ret void
    }

    define void @foo.act(%foo* %0) section "fn-$RUSTY$foo.act:v" {
    entry:
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type {}
    %foo = type {}
    %prog = type {}
    %cl = type {}
    %foo.m = type {}
    %cl.m = type {}

    @__myStruct__init = external global %myStruct, section "var-$RUSTY$__myStruct__init:r0"
    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r0"
    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r0"
    @__cl__init = external global %cl, section "var-$RUSTY$__cl__init:r0"

    define void @__init_mystruct(%myStruct* %0) section "fn-$RUSTY$__init_mystruct:v[pr0]" {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr0]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    define void @__init_foo.m(%foo.m* %0) section "fn-$RUSTY$__init_foo.m:v[pr0]" {
    entry:
      %self = alloca %foo.m*, align 8
      store %foo.m* %0, %foo.m** %self, align 8
      ret void
    }

    declare void @foo.m(%foo*, %foo.m*) section "fn-$RUSTY$foo.m:v"

    define void @__init_prog(%prog* %0) section "fn-$RUSTY$__init_prog:v[pr0]" {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"

    define void @__init_cl.m(%cl.m* %0) section "fn-$RUSTY$__init_cl.m:v[pr0]" {
    entry:
      %self = alloca %cl.m*, align 8
      store %cl.m* %0, %cl.m** %self, align 8
      ret void
    }

    declare void @cl.m(%cl*, %cl.m*) section "fn-$RUSTY$cl.m:v"

    define void @__init_cl(%cl* %0) section "fn-$RUSTY$__init_cl:v[pr0]" {
    entry:
      %self = alloca %cl*, align 8
      store %cl* %0, %cl** %self, align 8
      ret void
    }

    declare void @cl(%cl*) section "fn-$RUSTY$cl:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type {}

    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r0"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      ret void
    }

    declare void @__init_prog(%prog*) section "fn-$RUSTY$__init_prog:v[pr0]"

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    "###);
}

#[test]
fn global_instance() {
    let res = codegen(
        r#"
      VAR_GLOBAL
          ps: STRING;
          fb: foo;
      END_VAR

      FUNCTION_BLOCK foo
      VAR
          s: REF_TO STRING := REF(ps);
      END_VAR
      END_FUNCTION_BLOCK

      PROGRAM prog
          fb();
      END_PROGRAM
      "#,
    );

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }
    %prog = type {}

    @ps = global [81 x i8] zeroinitializer, section "var-$RUSTY$ps:s8u81"
    @fb = global %foo zeroinitializer, section "var-$RUSTY$fb:r1ps8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r1ps8u81"
    @prog_instance = global %prog zeroinitializer, section "var-$RUSTY$prog_instance:r0"

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) section "fn-$RUSTY$prog:v" {
    entry:
      call void @foo(%foo* @fb)
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }
    %prog = type {}

    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"
    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r0"
    @ps = external global [81 x i8], section "var-$RUSTY$ps:s8u81"

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      ret void
    }

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    define void @__init_prog(%prog* %0) section "fn-$RUSTY$__init_prog:v[pr0]" {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type {}
    %foo = type { [81 x i8]* }

    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r0"
    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"
    @fb = external global %foo, section "var-$RUSTY$fb:r1ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @fb)
      ret void
    }

    declare void @__init_prog(%prog*) section "fn-$RUSTY$__init_prog:v[pr0]"

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"

    declare void @__init_foo(%foo*) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]"

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"
    "###);
}

#[test]
fn aliased_types() {
    let res = codegen(
        r#"
      VAR_GLOBAL
          ps: STRING;
          global_alias: alias;
      END_VAR    

      TYPE alias : foo; END_TYPE

      FUNCTION_BLOCK foo
      VAR
          s: REF_TO STRING := REF(ps);
      END_VAR
      END_FUNCTION_BLOCK

      PROGRAM prog
      VAR
          fb: alias;
      END_VAR
          fb();
      END_PROGRAM
      "#,
    );

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }
    %prog = type { %foo }

    @ps = global [81 x i8] zeroinitializer, section "var-$RUSTY$ps:s8u81"
    @global_alias = global %foo zeroinitializer, section "var-$RUSTY$global_alias:r1ps8u81"
    @__foo__init = unnamed_addr constant %foo zeroinitializer, section "var-$RUSTY$__foo__init:r1ps8u81"
    @prog_instance = global %prog zeroinitializer, section "var-$RUSTY$prog_instance:r1r1ps8u81"

    define void @foo(%foo* %0) section "fn-$RUSTY$foo:v" {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) section "fn-$RUSTY$prog:v" {
    entry:
      %fb = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      call void @foo(%foo* %fb)
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }
    %prog = type { %foo }

    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"
    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r1r1ps8u81"
    @ps = external global [81 x i8], section "var-$RUSTY$ps:s8u81"

    define void @__init_foo(%foo* %0) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]" {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      ret void
    }

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    define void @__init_prog(%prog* %0) section "fn-$RUSTY$__init_prog:v[pr1r1ps8u81]" {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %fb = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %fb)
      ret void
    }

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %foo }
    %foo = type { [81 x i8]* }

    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r1r1ps8u81"
    @__foo__init = external global %foo, section "var-$RUSTY$__foo__init:r1ps8u81"
    @global_alias = external global %foo, section "var-$RUSTY$global_alias:r1ps8u81"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @global_alias)
      ret void
    }

    declare void @__init_prog(%prog*) section "fn-$RUSTY$__init_prog:v[pr1r1ps8u81]"

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"

    declare void @foo(%foo*) section "fn-$RUSTY$foo:v"

    declare void @__init_foo(%foo*) section "fn-$RUSTY$__init_foo:v[pr1ps8u81]"
    "###);
}

#[test]
#[ignore = "not yet implemented"]
fn array_of_instances() {
    let res = codegen(
        r#"
    VAR_GLOBAL
        ps: STRING;
        globals: ARRAY[0..10] OF foo;
        globals2: ARRAY[0..10] OF foo;
    END_VAR    

    FUNCTION_BLOCK foo
    VAR
        s: REF_TO STRING := REF(ps);
    END_VAR
    END_FUNCTION_BLOCK

    PROGRAM prog
    VAR
        fb: ARRAY[0..10] OF foo;
        i : DINT;
    END_VAR
        FOR i := 0 TO 10 DO
          fb[i]();
        END_FOR;
    END_PROGRAM
    "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "not yet implemented"]
fn override_default_initializer() {
    let res = codegen(
        r#"
    VAR_GLOBAL
        ps: STRING;
    END_VAR

    FUNCTION_BLOCK foo
    VAR
        s: REF_TO STRING := REF(ps);
    END_VAR
    END_FUNCTION_BLOCK

    PROGRAM prog
    VAR
        fb: foo := (s1 := REF(ps));
    END_VAR
        fb();
    END_PROGRAM
    "#,
    );

    insta::assert_snapshot!(res, @r###""###);
}

#[test]
fn var_config_aliased_variables_initialized() {
    let res = codegen(
        r"
    FUNCTION_BLOCK FB 
    VAR 
      foo AT %I* : DINT; 
    END_VAR
    END_FUNCTION_BLOCK

    VAR_CONFIG
      prog.instance1.foo AT %IX1.2.1 : DINT;
      prog.instance2.foo AT %QX1.2.2 : DINT;
    END_VAR

    PROGRAM prog 
    VAR
        instance1: FB;
        instance2: FB;
    END_VAR
    END_PROGRAM
        ",
    );

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB = type { i32* }
    %prog = type { %FB, %FB }

    @__PI_1_2_1 = global i32 0, section "var-$RUSTY$__PI_1_2_1:i32"
    @__PI_1_2_2 = global i32 0, section "var-$RUSTY$__PI_1_2_2:i32"
    @__FB__init = unnamed_addr constant %FB zeroinitializer, section "var-$RUSTY$__FB__init:r1pi32"
    @prog_instance = global %prog zeroinitializer, section "var-$RUSTY$prog_instance:r2r1pi32r1pi32"

    define void @FB(%FB* %0) section "fn-$RUSTY$FB:v" {
    entry:
      %foo = getelementptr inbounds %FB, %FB* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) section "fn-$RUSTY$prog:v" {
    entry:
      %instance1 = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      %instance2 = getelementptr inbounds %prog, %prog* %0, i32 0, i32 1
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %FB = type { i32* }
    %prog = type { %FB, %FB }

    @__FB__init = external global %FB, section "var-$RUSTY$__FB__init:r1pi32"
    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r2r1pi32r1pi32"

    define void @__init_fb(%FB* %0) section "fn-$RUSTY$__init_fb:v[pr1pi32]" {
    entry:
      %self = alloca %FB*, align 8
      store %FB* %0, %FB** %self, align 8
      ret void
    }

    declare void @FB(%FB*) section "fn-$RUSTY$FB:v"

    define void @__init_prog(%prog* %0) section "fn-$RUSTY$__init_prog:v[pr2r1pi32r1pi32]" {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %instance1 = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_fb(%FB* %instance1)
      %deref1 = load %prog*, %prog** %self, align 8
      %instance2 = getelementptr inbounds %prog, %prog* %deref1, i32 0, i32 1
      call void @__init_fb(%FB* %instance2)
      ret void
    }

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %FB, %FB }
    %FB = type { i32* }

    @prog_instance = external global %prog, section "var-$RUSTY$prog_instance:r2r1pi32r1pi32"
    @__FB__init = external global %FB, section "var-$RUSTY$__FB__init:r1pi32"
    @__PI_1_2_1 = external global i32, section "var-$RUSTY$__PI_1_2_1:i32"
    @__PI_1_2_2 = external global i32, section "var-$RUSTY$__PI_1_2_2:i32"

    define void @__init___testproject() section "fn-$RUSTY$__init___testproject:v" {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init___var_config()
      ret void
    }

    define void @__init___var_config() section "fn-$RUSTY$__init___var_config:v" {
    entry:
      store i32* @__PI_1_2_1, i32** getelementptr inbounds (%prog, %prog* @prog_instance, i32 0, i32 0, i32 0), align 8
      store i32* @__PI_1_2_2, i32** getelementptr inbounds (%prog, %prog* @prog_instance, i32 0, i32 1, i32 0), align 8
      ret void
    }

    declare void @__init_prog(%prog*) section "fn-$RUSTY$__init_prog:v[pr2r1pi32r1pi32]"

    declare void @prog(%prog*) section "fn-$RUSTY$prog:v"

    declare void @FB(%FB*) section "fn-$RUSTY$FB:v"
    "###);
}
