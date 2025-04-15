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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global [81 x i8]* null
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8]
    @ps = external global [81 x i8]*
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "#);
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global [81 x i8]* null
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8]
    @ps = external global [81 x i8]*
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "#);
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global [81 x i8]* null
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8]
    @ps = external global [81 x i8]*
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "#);
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %PLC_PRG = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer

    define void @PLC_PRG(%PLC_PRG* %0) {
    entry:
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG
    @s = external global [81 x i8]

    define void @__init_plc_prg(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      %deref = load %PLC_PRG*, %PLC_PRG** %self, align 8
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    declare void @PLC_PRG(%PLC_PRG*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }

    declare void @__init_plc_prg(%PLC_PRG*)

    declare void @PLC_PRG(%PLC_PRG*)
    "#);
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %PLC_PRG = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer

    define void @PLC_PRG(%PLC_PRG* %0) {
    entry:
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG
    @s = external global [81 x i8]

    define void @__init_plc_prg(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      %deref = load %PLC_PRG*, %PLC_PRG** %self, align 8
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    declare void @PLC_PRG(%PLC_PRG*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }

    declare void @__init_plc_prg(%PLC_PRG*)

    declare void @PLC_PRG(%PLC_PRG*)
    "#);
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %to_init = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }

    @__foo__init = external global %foo
    @s = external global [81 x i8]

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %to_init = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]*, %bar }
    %bar = type { %baz }
    %baz = type { [81 x i8]* }
    %mainProg = type { [81 x i8]*, %foo }
    %sideProg = type { [81 x i8]*, %foo }

    @str = global [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__baz__init = unnamed_addr constant %baz zeroinitializer
    @mainProg_instance = global %mainProg zeroinitializer
    @sideProg_instance = global %sideProg zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %str_ref = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      call void @bar_print(%bar* %b)
      call void @bar(%bar* %b)
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      call void @baz_print(%baz* %b)
      ret void
    }

    define void @baz(%baz* %0) {
    entry:
      %str_ref = getelementptr inbounds %baz, %baz* %0, i32 0, i32 0
      ret void
    }

    define void @mainProg(%mainProg* %0) {
    entry:
      %other_ref_to_global = getelementptr inbounds %mainProg, %mainProg* %0, i32 0, i32 0
      %f = getelementptr inbounds %mainProg, %mainProg* %0, i32 0, i32 1
      ret void
    }

    define void @sideProg(%sideProg* %0) {
    entry:
      %other_ref_to_global = getelementptr inbounds %sideProg, %sideProg* %0, i32 0, i32 0
      %f = getelementptr inbounds %sideProg, %sideProg* %0, i32 0, i32 1
      call void @foo(%foo* %f)
      call void @foo_print(%foo* %f)
      ret void
    }

    define void @bar_print(%bar* %0) {
    entry:
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void
    }

    define void @foo_print(%foo* %0) {
    entry:
      %str_ref = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @baz_print(%baz* %0) {
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

    @__foo__init = external global %foo
    @__bar__init = external global %bar
    @__baz__init = external global %baz
    @mainProg_instance = external global %mainProg
    @sideProg_instance = external global %sideProg
    @str = external global [81 x i8]

    define void @__init_foo(%foo* %0) {
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

    declare void @foo(%foo*)

    declare void @bar(%bar*)

    declare void @baz(%baz*)

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %b = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_baz(%baz* %b)
      ret void
    }

    define void @__init_baz(%baz* %0) {
    entry:
      %self = alloca %baz*, align 8
      store %baz* %0, %baz** %self, align 8
      %deref = load %baz*, %baz** %self, align 8
      %str_ref = getelementptr inbounds %baz, %baz* %deref, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %str_ref, align 8
      ret void
    }

    define void @__init_mainprog(%mainProg* %0) {
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

    declare void @mainProg(%mainProg*)

    define void @__init_sideprog(%sideProg* %0) {
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

    declare void @sideProg(%sideProg*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %mainProg = type { [81 x i8]*, %foo }
    %foo = type { [81 x i8]*, %bar }
    %bar = type { %baz }
    %baz = type { [81 x i8]* }
    %sideProg = type { [81 x i8]*, %foo }

    @mainProg_instance = external global %mainProg
    @__foo__init = external global %foo
    @__bar__init = external global %bar
    @__baz__init = external global %baz
    @sideProg_instance = external global %sideProg
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_mainprog(%mainProg* @mainProg_instance)
      call void @__init_sideprog(%sideProg* @sideProg_instance)
      ret void
    }

    declare void @__init_mainprog(%mainProg*)

    declare void @mainProg(%mainProg*)

    declare void @foo(%foo*)

    declare void @bar(%bar*)

    declare void @baz(%baz*)

    declare void @__init_sideprog(%sideProg*)

    declare void @sideProg(%sideProg*)
    "#);
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

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }

    @s = global [81 x i8] c"Hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @s2 = global [2 x [81 x i8]] [[81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"world\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"]
    @prog_instance = global %prog zeroinitializer
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer

    define void @prog(%prog* %0) {
    entry:
      %str = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }
    %prog = type { %myStruct }

    @__myStruct__init = external global %myStruct
    @prog_instance = external global %prog
    @s = external global [81 x i8]
    @s2 = external global [2 x [81 x i8]]

    define void @__init_mystruct(%myStruct* %0) {
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

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %str = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_mystruct(%myStruct* %str)
      ret void
    }

    declare void @prog(%prog*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }

    @prog_instance = external global %prog
    @__myStruct__init = external global %myStruct
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      ret void
    }

    declare void @__init_prog(%prog*)

    declare void @prog(%prog*)
    "#);
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

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %prog = type {}
    %foo = type {}
    %cl = type {}
    %myStruct = type {}

    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__cl__init = unnamed_addr constant %cl zeroinitializer
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer

    define void @prog(%prog* %0) {
    entry:
      ret void
    }

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo_m(%foo* %0) {
    entry:
      ret void
    }

    define void @cl(%cl* %0) {
    entry:
      ret void
    }

    define void @cl_m(%cl* %0) {
    entry:
      ret void
    }

    define void @foo_act(%foo* %0) {
    entry:
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %myStruct = type {}
    %foo = type {}
    %prog = type {}
    %cl = type {}

    @__myStruct__init = external global %myStruct
    @__foo__init = external global %foo
    @prog_instance = external global %prog
    @__cl__init = external global %cl

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    declare void @prog(%prog*)

    define void @__init_cl(%cl* %0) {
    entry:
      %self = alloca %cl*, align 8
      store %cl* %0, %cl** %self, align 8
      ret void
    }

    declare void @cl(%cl*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type {}

    @prog_instance = external global %prog
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      ret void
    }

    declare void @__init_prog(%prog*)

    declare void @prog(%prog*)
    "#);
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

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }
    %prog = type {}

    @ps = global [81 x i8] zeroinitializer
    @fb = global %foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @prog_instance = global %prog zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      call void @foo(%foo* @fb)
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }
    %prog = type {}

    @__foo__init = external global %foo
    @prog_instance = external global %prog
    @ps = external global [81 x i8]

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      ret void
    }

    declare void @foo(%foo*)

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    declare void @prog(%prog*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type {}
    %foo = type { [81 x i8]* }

    @prog_instance = external global %prog
    @__foo__init = external global %foo
    @fb = external global %foo
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @fb)
      ret void
    }

    declare void @__init_prog(%prog*)

    declare void @prog(%prog*)

    declare void @__init_foo(%foo*)

    declare void @foo(%foo*)
    "#);
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

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8]* }
    %prog = type { %foo }

    @ps = global [81 x i8] zeroinitializer
    @global_alias = global %foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @prog_instance = global %prog zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      %fb = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      call void @foo(%foo* %fb)
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8]* }
    %prog = type { %foo }

    @__foo__init = external global %foo
    @prog_instance = external global %prog
    @ps = external global [81 x i8]

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      ret void
    }

    declare void @foo(%foo*)

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %fb = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %fb)
      ret void
    }

    declare void @prog(%prog*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %foo }
    %foo = type { [81 x i8]* }

    @prog_instance = external global %prog
    @__foo__init = external global %foo
    @global_alias = external global %foo
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @global_alias)
      ret void
    }

    declare void @__init_prog(%prog*)

    declare void @prog(%prog*)

    declare void @foo(%foo*)

    declare void @__init_foo(%foo*)
    "#);
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

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB = type { i32* }
    %prog = type { %FB, %FB }

    @__PI_1_2_1 = global i32 0
    @__PI_1_2_2 = global i32 0
    @__FB__init = unnamed_addr constant %FB zeroinitializer
    @prog_instance = global %prog zeroinitializer

    define void @FB(%FB* %0) {
    entry:
      %foo = getelementptr inbounds %FB, %FB* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      %instance1 = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      %instance2 = getelementptr inbounds %prog, %prog* %0, i32 0, i32 1
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %FB = type { i32* }
    %prog = type { %FB, %FB }

    @__FB__init = external global %FB
    @prog_instance = external global %prog

    define void @__init_fb(%FB* %0) {
    entry:
      %self = alloca %FB*, align 8
      store %FB* %0, %FB** %self, align 8
      ret void
    }

    declare void @FB(%FB*)

    define void @__init_prog(%prog* %0) {
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

    declare void @prog(%prog*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %prog = type { %FB, %FB }
    %FB = type { i32* }

    @prog_instance = external global %prog
    @__FB__init = external global %FB
    @__PI_1_2_1 = external global i32
    @__PI_1_2_2 = external global i32
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init___var_config()
      ret void
    }

    define void @__init___var_config() {
    entry:
      store i32* @__PI_1_2_1, i32** getelementptr inbounds (%prog, %prog* @prog_instance, i32 0, i32 0, i32 0), align 8
      store i32* @__PI_1_2_2, i32** getelementptr inbounds (%prog, %prog* @prog_instance, i32 0, i32 1, i32 0), align 8
      ret void
    }

    declare void @__init_prog(%prog*)

    declare void @prog(%prog*)

    declare void @FB(%FB*)
    "#);
}

#[test]
fn var_external_blocks_are_ignored_in_init_functions() {
    let res = codegen(
        r"
    VAR_GLOBAL
        s: STRING;
        refString AT s : STRING;
    END_VAR

    FUNCTION_BLOCK foo
    VAR_EXTERNAL
        refString : STRING;
    END_VAR
    END_FUNCTION

    FUNCTION bar
    VAR_EXTERNAL
        refString : STRING;
    END_VAR
    END_FUNCTION
        ",
    );

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}

    @s = global [81 x i8] zeroinitializer
    @refString = global [81 x i8]* null
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @bar() {
    entry:
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @s = external global [81 x i8]
    @refString = external global [81 x i8]*
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @refString, align 8
      ret void
    }
    "#)
}

#[test]
fn ref_to_local_member() {
    let res = codegen(
        r"
    FUNCTION_BLOCK foo
    VAR
        s  : STRING;
        ptr : REF_TO STRING := REF(s);
        alias AT s : STRING;
        reference_to : REFERENCE TO STRING REF= s;
    END_VAR
    END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8], [81 x i8]*, [81 x i8]*, [81 x i8]* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %ptr = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %alias = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %reference_to = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8], [81 x i8]*, [81 x i8]*, [81 x i8]* }

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %ptr = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      %deref1 = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      %deref2 = load %foo*, %foo** %self, align 8
      %alias = getelementptr inbounds %foo, %foo* %deref2, i32 0, i32 2
      %deref3 = load %foo*, %foo** %self, align 8
      %s4 = getelementptr inbounds %foo, %foo* %deref3, i32 0, i32 0
      store [81 x i8]* %s4, [81 x i8]** %alias, align 8
      %deref5 = load %foo*, %foo** %self, align 8
      %reference_to = getelementptr inbounds %foo, %foo* %deref5, i32 0, i32 3
      %deref6 = load %foo*, %foo** %self, align 8
      %s7 = getelementptr inbounds %foo, %foo* %deref6, i32 0, i32 0
      store [81 x i8]* %s7, [81 x i8]** %reference_to, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#)
}

#[test]
fn ref_to_local_member_shadows_global() {
    let res = codegen(
        r"
    VAR_GLOBAL
        s : STRING;
    END_VAR

    FUNCTION_BLOCK foo
    VAR
        s : STRING;
        ptr : REF_TO STRING := REF(s);
        alias AT s : STRING;
        reference_to : REFERENCE TO STRING REF= s;
    END_VAR
    END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8], [81 x i8]*, [81 x i8]*, [81 x i8]* }

    @s = global [81 x i8] zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %ptr = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %alias = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %reference_to = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8], [81 x i8]*, [81 x i8]*, [81 x i8]* }

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %ptr = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      %deref1 = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      %deref2 = load %foo*, %foo** %self, align 8
      %alias = getelementptr inbounds %foo, %foo* %deref2, i32 0, i32 2
      %deref3 = load %foo*, %foo** %self, align 8
      %s4 = getelementptr inbounds %foo, %foo* %deref3, i32 0, i32 0
      store [81 x i8]* %s4, [81 x i8]** %alias, align 8
      %deref5 = load %foo*, %foo** %self, align 8
      %reference_to = getelementptr inbounds %foo, %foo* %deref5, i32 0, i32 3
      %deref6 = load %foo*, %foo** %self, align 8
      %s7 = getelementptr inbounds %foo, %foo* %deref6, i32 0, i32 0
      store [81 x i8]* %s7, [81 x i8]** %reference_to, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#)
}

#[test]
fn temporary_variable_ref_to_local_member() {
    let res = codegen(
        r"
    FUNCTION_BLOCK foo
    VAR
        s  : STRING;
    END_VAR
    VAR_TEMP
        ptr : REF_TO STRING := REF(s);
        alias AT s : STRING;
        reference_to : REFERENCE TO STRING REF= s;
    END_VAR
    END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { [81 x i8] }

    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %ptr = alloca [81 x i8]*, align 8
      %alias = alloca [81 x i8]*, align 8
      %reference_to = alloca [81 x i8]*, align 8
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      store [81 x i8]* null, [81 x i8]** %alias, align 8
      store [81 x i8]* null, [81 x i8]** %reference_to, align 8
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      store [81 x i8]* %s, [81 x i8]** %alias, align 8
      store [81 x i8]* %s, [81 x i8]** %reference_to, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { [81 x i8] }

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#)
}

#[test]
fn temporary_variable_ref_to_temporary_variable() {
    let res = codegen(
        r"
    FUNCTION foo
    VAR
        ptr : REF_TO STRING := REF(s);
        alias AT s : STRING;
    END_VAR
    VAR_TEMP
        s  : STRING;
        reference_to : REFERENCE TO STRING REF= alias;
    END_VAR
    END_FUNCTION
        ",
    );

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    define void @foo() {
    entry:
      %ptr = alloca [81 x i8]*, align 8
      %alias = alloca [81 x i8]*, align 8
      %s = alloca [81 x i8], align 1
      %reference_to = alloca [81 x i8]*, align 8
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      store [81 x i8]* null, [81 x i8]** %alias, align 8
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      store [81 x i8]* null, [81 x i8]** %reference_to, align 8
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      store [81 x i8]* %s, [81 x i8]** %alias, align 8
      %deref = load [81 x i8]*, [81 x i8]** %alias, align 8
      store [81 x i8]* %deref, [81 x i8]** %reference_to, align 8
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#)
}

#[test]
fn initializing_method_variables_with_refs() {
    let src = r"
    FUNCTION_BLOCK foo
        METHOD bar
            VAR
                x   : DINT := 10;
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}

    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo_bar(%foo* %0) {
    entry:
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 10, i32* %x, align 4
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn initializing_method_variables_with_refs_referencing_parent_pou_variable() {
    let src = r"
    FUNCTION_BLOCK foo
        VAR
            x : DINT := 5;
        END_VAR

        METHOD bar
            VAR
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32 }

    @__foo__init = unnamed_addr constant %foo { i32 5 }

    define void @foo(%foo* %0) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @foo_bar(%foo* %0) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %px = alloca i32*, align 8
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { i32 }

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn initializing_method_variables_with_refs_referencing_global_variable() {
    let src = r"
    VAR_GLOBAL
        x : DINT;
    END_VAR

    FUNCTION_BLOCK foo
        METHOD bar
            VAR
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}

    @x = global i32 0
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo_bar(%foo* %0) {
    entry:
      %px = alloca i32*, align 8
      store i32* @x, i32** %px, align 8
      store i32* @x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn initializing_method_variables_with_refs_shadowing() {
    let src = r"
    VAR_GLOBAL
        x : DINT;
    END_VAR

    FUNCTION_BLOCK foo
        METHOD bar
            VAR
                x : DINT;
                px : REF_TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}

    @x = global i32 0
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo_bar(%foo* %0) {
    entry:
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 0, i32* %x, align 4
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn initializing_method_variables_with_alias() {
    let src = r"
    FUNCTION_BLOCK foo
        METHOD bar
            VAR
                x : DINT;
                px AT x : DINT;
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}

    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo_bar(%foo* %0) {
    entry:
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 0, i32* %x, align 4
      store i32* null, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn initializing_method_variables_with_reference_to() {
    let src = r"
    FUNCTION_BLOCK foo
        METHOD bar
            VAR
                x : DINT;
                px : REFERENCE TO DINT := REF(x);
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
    ";

    insta::assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type {}

    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      ret void
    }

    define void @foo_bar(%foo* %0) {
    entry:
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 0, i32* %x, align 4
      store i32* null, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type {}

    @__foo__init = external global %foo

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    declare void @foo(%foo*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn methods_call_init_functions_for_their_members() {
    let src = r#"
    FUNCTION_BLOCK foo
        VAR
            x : DINT;
            y AT x : DINT;
        END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK bar
        METHOD baz
            VAR 
                fb: foo;
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK
  "#;

    // when compiling to ir, we expect `bar.baz` to call `__init_foo` with the local instance.
    assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32, i32* }
    %bar = type {}

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      ret void
    }

    define void @bar_baz(%bar* %0) {
    entry:
      %fb = alloca %foo, align 8
      %1 = bitcast %foo* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%foo* @__foo__init to i8*), i64 ptrtoint (%foo* getelementptr (%foo, %foo* null, i32 1) to i64), i1 false)
      call void @__init_foo(%foo* %fb)
      ret void
    }

    declare void @__init_foo(%foo*)

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    ; ModuleID = '__initializers'
    source_filename = "__initializers"

    %foo = type { i32, i32* }
    %bar = type {}

    @__foo__init = external global %foo
    @__bar__init = external global %bar

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %y = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      %deref1 = load %foo*, %foo** %self, align 8
      %x = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i32* %x, i32** %y, align 8
      ret void
    }

    declare void @foo(%foo*)

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      ret void
    }

    declare void @bar(%bar*)
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}
