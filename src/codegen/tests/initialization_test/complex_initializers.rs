use driver::generate_to_string;
use plc_source::SourceCode;
use plc_util::filtered_assert_snapshot;

#[test]
fn simple_global() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            VAR_GLOBAL
                s: STRING := 'hello world!';
                ps: REF_TO STRING := REF(s);
            END_VAR
            "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global [81 x i8]* null

    define void @__init___Test() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "#);
}

#[test]
fn global_alias() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
        VAR_GLOBAL
            s: STRING := 'hello world!';
            ps AT s : STRING;
        END_VAR
        "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global [81 x i8]* null

    define void @__init___Test() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "#);
}

#[test]
fn global_reference_to() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
        VAR_GLOBAL
            s: STRING := 'hello world!';
            ps: REFERENCE TO STRING := REF(s);
        END_VAR
        "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global [81 x i8]* null

    define void @__init___Test() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @ps, align 8
      ret void
    }
    "#);
}

#[test]
fn init_functions_generated_for_programs() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %PLC_PRG = type { [81 x i8]* }

    @s = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer

    define void @PLC_PRG(%PLC_PRG* %0) {
    entry:
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %0, i32 0, i32 0
      ret void
    }

    define void @__init_plc_prg(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      %deref = load %PLC_PRG*, %PLC_PRG** %self, align 8
      %to_init = getelementptr inbounds %PLC_PRG, %PLC_PRG* %deref, i32 0, i32 0
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      ret void
    }

    define void @__user_init_PLC_PRG(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      call void @__user_init_PLC_PRG(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }
    "#);
}

#[test]
#[ignore = "ADR() currently not working, tracked in PRG-2686"]
fn init_functions_work_with_adr() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            PROGRAM PLC_PRG
            VAR
                to_init: LWORD := ADR(s);
            END_VAR
            END_PROGRAM

            VAR_GLOBAL
                s: STRING;
            END_VAR
            "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r#"
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

    define void @__user_init_PLC_PRG(%PLC_PRG* %0) {
    entry:
      %self = alloca %PLC_PRG*, align 8
      store %PLC_PRG* %0, %PLC_PRG** %self, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    %PLC_PRG = type { [81 x i8]* }

    @PLC_PRG_instance = external global %PLC_PRG
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      call void @__init_plc_prg(%PLC_PRG* @PLC_PRG_instance)
      call void @__user_init_PLC_PRG(%PLC_PRG* @PLC_PRG_instance)
      ret void
    }

    declare void @__init_plc_prg(%PLC_PRG*)

    declare void @PLC_PRG(%PLC_PRG*)

    declare void @__user_init_PLC_PRG(%PLC_PRG*)
    "#);
}

#[test]
fn init_functions_generated_for_function_blocks() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32*, [81 x i8]* }
    %__vtable_foo_type = type { i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @s = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %to_init = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %to_init = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      store [81 x i8]* @s, [81 x i8]** %to_init, align 8
      %deref1 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn nested_initializer_pous() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { [81 x i8]*, %foo }
    %foo = type { i32*, [81 x i8]*, %bar }
    %bar = type { i32*, %baz }
    %baz = type { i32*, [81 x i8]* }
    %sideProg = type { [81 x i8]*, %foo }
    %__vtable_foo_type = type { i32* }
    %__vtable_bar_type = type { i32* }
    %__vtable_baz_type = type { i32* }

    @str = global [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @mainProg_instance = global %mainProg zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__baz__init = unnamed_addr constant %baz zeroinitializer
    @sideProg_instance = global %sideProg zeroinitializer
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer
    @__vtable_bar = global %__vtable_bar_type zeroinitializer
    @____vtable_baz_type__init = constant %__vtable_baz_type zeroinitializer
    @__vtable_baz = global %__vtable_baz_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %str_ref = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      call void @bar__print(%bar* %b)
      call void @bar(%bar* %b)
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      call void @baz__print(%baz* %b)
      ret void
    }

    define void @baz(%baz* %0) {
    entry:
      %this = alloca %baz*, align 8
      store %baz* %0, %baz** %this, align 8
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
      call void @foo__print(%foo* %f)
      ret void
    }

    define void @bar__print(%bar* %0) {
    entry:
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %b = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void
    }

    define void @foo__print(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %str_ref = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @baz__print(%baz* %0) {
    entry:
      %this = alloca %baz*, align 8
      store %baz* %0, %baz** %this, align 8
      %str_ref = getelementptr inbounds %baz, %baz* %0, i32 0, i32 0
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %b = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      call void @__init_bar(%bar* %b)
      %deref1 = load %foo*, %foo** %self, align 8
      %str_ref = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %str_ref, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %b = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 1
      call void @__init_baz(%baz* %b)
      %deref1 = load %bar*, %bar** %self, align 8
      %__vtable = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_baz(%baz* %0) {
    entry:
      %self = alloca %baz*, align 8
      store %baz* %0, %baz** %self, align 8
      %deref = load %baz*, %baz** %self, align 8
      %str_ref = getelementptr inbounds %baz, %baz* %deref, i32 0, i32 1
      store [81 x i8]* @str, [81 x i8]** %str_ref, align 8
      %deref1 = load %baz*, %baz** %self, align 8
      %__vtable = getelementptr inbounds %baz, %baz* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_baz_type* @__vtable_baz to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_mainprog(%mainProg* %0) {
    entry:
      %self = alloca %mainProg*, align 8
      store %mainProg* %0, %mainProg** %self, align 8
      %deref = load %mainProg*, %mainProg** %self, align 8
      %f = getelementptr inbounds %mainProg, %mainProg* %deref, i32 0, i32 1
      call void @__init_foo(%foo* %f)
      %deref1 = load %mainProg*, %mainProg** %self, align 8
      %other_ref_to_global = getelementptr inbounds %mainProg, %mainProg* %deref1, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %other_ref_to_global, align 8
      ret void
    }

    define void @__init_sideprog(%sideProg* %0) {
    entry:
      %self = alloca %sideProg*, align 8
      store %sideProg* %0, %sideProg** %self, align 8
      %deref = load %sideProg*, %sideProg** %self, align 8
      %f = getelementptr inbounds %sideProg, %sideProg* %deref, i32 0, i32 1
      call void @__init_foo(%foo* %f)
      %deref1 = load %sideProg*, %sideProg** %self, align 8
      %other_ref_to_global = getelementptr inbounds %sideProg, %sideProg* %deref1, i32 0, i32 0
      store [81 x i8]* @str, [81 x i8]** %other_ref_to_global, align 8
      ret void
    }

    define void @__user_init_sideProg(%sideProg* %0) {
    entry:
      %self = alloca %sideProg*, align 8
      store %sideProg* %0, %sideProg** %self, align 8
      %deref = load %sideProg*, %sideProg** %self, align 8
      %f = getelementptr inbounds %sideProg, %sideProg* %deref, i32 0, i32 1
      call void @__user_init_foo(%foo* %f)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %b = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 2
      call void @__user_init_bar(%bar* %b)
      ret void
    }

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %b = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 1
      call void @__user_init_baz(%baz* %b)
      ret void
    }

    define void @__user_init_baz(%baz* %0) {
    entry:
      %self = alloca %baz*, align 8
      store %baz* %0, %baz** %self, align 8
      ret void
    }

    define void @__user_init_mainProg(%mainProg* %0) {
    entry:
      %self = alloca %mainProg*, align 8
      store %mainProg* %0, %mainProg** %self, align 8
      %deref = load %mainProg*, %mainProg** %self, align 8
      %f = getelementptr inbounds %mainProg, %mainProg* %deref, i32 0, i32 1
      call void @__user_init_foo(%foo* %f)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_mainprog(%mainProg* @mainProg_instance)
      call void @__init_sideprog(%sideProg* @sideProg_instance)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_bar_type(%__vtable_bar_type* @__vtable_bar)
      call void @__init___vtable_baz_type(%__vtable_baz_type* @__vtable_baz)
      call void @__user_init_mainProg(%mainProg* @mainProg_instance)
      call void @__user_init_sideProg(%sideProg* @sideProg_instance)
      ret void
    }
    "###);
}

#[test]
fn local_address() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            FUNCTION_BLOCK foo
            VAR
                i : INT;
                pi: REF_TO INT := REF(i);
            END_VAR
            END_FUNCTION_BLOCK
            "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i16, i16* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %i = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %pi = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %pi = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      %deref1 = load %foo*, %foo** %self, align 8
      %i = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i16* %i, i16** %pi, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }
    "###);
}

#[test]
fn user_init_called_for_variables_on_stack() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            FUNCTION_BLOCK foo
            VAR
                i : INT;
                pi: REF_TO INT;
            END_VAR
                METHOD FB_INIT
                  pi := ADR(i);
                END_METHOD
            END_FUNCTION_BLOCK

            FUNCTION main
            VAR
                fb: foo;
            END_VAR
                fb();
            END_FUNCTION
            "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i16, i16* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %i = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %pi = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @foo__FB_INIT(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %i = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %pi = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      store i16* %i, i16** %pi, align 8
      ret void
    }

    define void @main() {
    entry:
      %fb = alloca %foo, align 8
      %0 = bitcast %foo* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%foo* @__foo__init to i8*), i64 ptrtoint (%foo* getelementptr (%foo, %foo* null, i32 1) to i64), i1 false)
      call void @__init_foo(%foo* %fb)
      call void @__user_init_foo(%foo* %fb)
      call void @foo(%foo* %fb)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      call void @foo__FB_INIT(%foo* %deref)
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
#[ignore = "stack-local vars not yet supported"]
fn stack_allocated_variables_are_initialized_in_pou_body() {
    let result = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(result, @r###""###);
}

#[test]
#[ignore = "initializing references in same POU not yet supported"]
fn ref_to_input_variable() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "initializing references in same POU not yet supported"]
fn ref_to_inout_variable() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###""###);
}

#[test]
fn struct_types() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %myStruct }
    %myStruct = type { [81 x i8]*, [2 x [81 x i8]]* }

    @s = global [81 x i8] c"Hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @s2 = global [2 x [81 x i8]] [[81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"world\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"]
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer

    define void @prog(%prog* %0) {
    entry:
      %str = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      ret void
    }

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

    define void @__user_init_myStruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %str = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__user_init_myStruct(%myStruct* %str)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__user_init_prog(%prog* @prog_instance)
      ret void
    }
    "###);
}

#[test]
fn stateful_pous_methods_and_structs_get_init_functions() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            TYPE myStruct : STRUCT
                    x: DINT;
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myStruct = type { i32 }
    %foo = type { i32* }
    %cl = type { i32* }
    %prog = type {}
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_cl_type = type { i32* }

    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__cl__init = unnamed_addr constant %cl zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer
    @____vtable_cl_type__init = constant %__vtable_cl_type zeroinitializer
    @__vtable_cl = global %__vtable_cl_type zeroinitializer

    define void @prog(%prog* %0) {
    entry:
      ret void
    }

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @foo__m(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @cl(%cl* %0) {
    entry:
      %__vtable = getelementptr inbounds %cl, %cl* %0, i32 0, i32 0
      ret void
    }

    define void @cl__m(%cl* %0) {
    entry:
      %__vtable = getelementptr inbounds %cl, %cl* %0, i32 0, i32 0
      ret void
    }

    define void @foo__act(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @__init_mystruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init___vtable_cl_type(%__vtable_cl_type* %0) {
    entry:
      %self = alloca %__vtable_cl_type*, align 8
      store %__vtable_cl_type* %0, %__vtable_cl_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__init_cl(%cl* %0) {
    entry:
      %self = alloca %cl*, align 8
      store %cl* %0, %cl** %self, align 8
      %deref = load %cl*, %cl** %self, align 8
      %__vtable = getelementptr inbounds %cl, %cl* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_cl_type* @__vtable_cl to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__user_init_myStruct(%myStruct* %0) {
    entry:
      %self = alloca %myStruct*, align 8
      store %myStruct* %0, %myStruct** %self, align 8
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_cl_type(%__vtable_cl_type* @__vtable_cl)
      call void @__user_init_prog(%prog* @prog_instance)
      ret void
    }
    "###);
}

#[test]
fn global_instance() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type {}
    %foo = type { i32*, [81 x i8]* }
    %__vtable_foo_type = type { i32* }

    @ps = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @fb = global %foo zeroinitializer
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      call void @foo(%foo* @fb)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      %deref1 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @fb)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__user_init_prog(%prog* @prog_instance)
      call void @__user_init_foo(%foo* @fb)
      ret void
    }
    "###);
}

#[test]
fn aliased_types() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %foo }
    %foo = type { i32*, [81 x i8]* }
    %__vtable_foo_type = type { i32* }

    @ps = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @global_alias = global %foo zeroinitializer
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      %fb = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      call void @foo(%foo* %fb)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      store [81 x i8]* @ps, [81 x i8]** %s, align 8
      %deref1 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %fb = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %fb)
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %fb = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %fb)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_foo(%foo* @global_alias)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__user_init_prog(%prog* @prog_instance)
      call void @__user_init_foo(%foo* @global_alias)
      ret void
    }
    "###);
}

#[test]
#[ignore = "not yet implemented"]
fn array_of_instances() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "not yet implemented"]
fn override_default_initializer() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
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
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###""###);
}

#[test]
#[ignore = "FIXME: Vtable causes a problem here, the problem also exists on master but vtable makes it appear more often"]
fn var_config_aliased_variables_initialized() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB = type {}
    %prog = type { %FB, %FB }
    %FB = type { i32*, i32* }

    @____vtable_FB__init = constant %__vtable_FB zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__FB__init = unnamed_addr constant %FB zeroinitializer
    @__PI_1_2_1 = global i32 0
    @__PI_1_2_2 = global i32 0

    define void @FB(%FB* %0) {
    entry:
      %this = alloca %FB*, align 8
      store %FB* %0, %FB** %this, align 8
      %foo = getelementptr inbounds %FB, %FB* %0, i32 0, i32 0
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      %instance1 = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      %instance2 = getelementptr inbounds %prog, %prog* %0, i32 0, i32 1
      ret void
    }

    define void @__init___vtable_fb(%__vtable_FB* %0) {
    entry:
      %self = alloca %__vtable_FB*, align 8
      store %__vtable_FB* %0, %__vtable_FB** %self, align 8
      ret void
    }

    define void @__init_fb(%FB* %0) {
    entry:
      %self = alloca %FB*, align 8
      store %FB* %0, %FB** %self, align 8
      ret void
    }

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

    define void @__user_init_FB(%FB* %0) {
    entry:
      %self = alloca %FB*, align 8
      store %FB* %0, %FB** %self, align 8
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %instance1 = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__user_init_FB(%FB* %instance1)
      %deref1 = load %prog*, %prog** %self, align 8
      %instance2 = getelementptr inbounds %prog, %prog* %deref1, i32 0, i32 1
      call void @__user_init_FB(%FB* %instance2)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init___var_config()
      call void @__user_init_prog(%prog* @prog_instance)
      ret void
    }

    define void @__init___var_config() {
    entry:
      store i32* @__PI_1_2_1, i32** getelementptr inbounds (%prog, %prog* @prog_instance, i32 0, i32 0, i32 1), align 8
      store i32* @__PI_1_2_2, i32** getelementptr inbounds (%prog, %prog* @prog_instance, i32 0, i32 1, i32 1), align 8
      ret void
    }
    "###);
}

#[test]
fn var_external_blocks_are_ignored_in_init_functions() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            VAR_GLOBAL
                s: STRING;
                refString AT s : STRING;
            END_VAR

            FUNCTION_BLOCK foo
            VAR_EXTERNAL
                refString : STRING;
            END_VAR
            END_FUNCTION_BLOCK

            FUNCTION bar
            VAR_EXTERNAL
                refString : STRING;
            END_VAR
            END_FUNCTION
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32* }
    %__vtable_foo_type = type { i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @s = global [81 x i8] zeroinitializer
    @refString = global [81 x i8]* null
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @bar() {
    entry:
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      store [81 x i8]* @s, [81 x i8]** @refString, align 8
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###)
}

#[test]
fn ref_to_local_member() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            FUNCTION_BLOCK foo
            VAR
                s  : STRING;
                ptr : REF_TO STRING := REF(s);
                alias AT s : STRING;
                reference_to : REFERENCE TO STRING REF= s;
            END_VAR
            END_FUNCTION_BLOCK
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32*, [81 x i8], [81 x i8]*, [81 x i8]*, [81 x i8]* }
    %__vtable_foo_type = type { i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %ptr = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %alias = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %reference_to = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %ptr = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 2
      %deref1 = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 1
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      %deref2 = load %foo*, %foo** %self, align 8
      %alias = getelementptr inbounds %foo, %foo* %deref2, i32 0, i32 3
      %deref3 = load %foo*, %foo** %self, align 8
      %s4 = getelementptr inbounds %foo, %foo* %deref3, i32 0, i32 1
      store [81 x i8]* %s4, [81 x i8]** %alias, align 8
      %deref5 = load %foo*, %foo** %self, align 8
      %reference_to = getelementptr inbounds %foo, %foo* %deref5, i32 0, i32 4
      %deref6 = load %foo*, %foo** %self, align 8
      %s7 = getelementptr inbounds %foo, %foo* %deref6, i32 0, i32 1
      store [81 x i8]* %s7, [81 x i8]** %reference_to, align 8
      %deref8 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref8, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###)
}

#[test]
fn ref_to_local_member_shadows_global() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32*, [81 x i8], [81 x i8]*, [81 x i8]*, [81 x i8]* }
    %__vtable_foo_type = type { i32* }

    @s = global [81 x i8] zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %ptr = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %alias = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %reference_to = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %ptr = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 2
      %deref1 = load %foo*, %foo** %self, align 8
      %s = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 1
      store [81 x i8]* %s, [81 x i8]** %ptr, align 8
      %deref2 = load %foo*, %foo** %self, align 8
      %alias = getelementptr inbounds %foo, %foo* %deref2, i32 0, i32 3
      %deref3 = load %foo*, %foo** %self, align 8
      %s4 = getelementptr inbounds %foo, %foo* %deref3, i32 0, i32 1
      store [81 x i8]* %s4, [81 x i8]** %alias, align 8
      %deref5 = load %foo*, %foo** %self, align 8
      %reference_to = getelementptr inbounds %foo, %foo* %deref5, i32 0, i32 4
      %deref6 = load %foo*, %foo** %self, align 8
      %s7 = getelementptr inbounds %foo, %foo* %deref6, i32 0, i32 1
      store [81 x i8]* %s7, [81 x i8]** %reference_to, align 8
      %deref8 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref8, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###)
}

#[test]
fn temporary_variable_ref_to_local_member() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32*, [81 x i8] }
    %__vtable_foo_type = type { i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
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

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###)
}

#[test]
fn temporary_variable_ref_to_temporary_variable() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

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

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    "#)
}

#[test]
fn initializing_method_variables_with_refs() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            FUNCTION_BLOCK foo
            METHOD bar
                VAR
                    x   : DINT := 10;
                    px : REF_TO DINT := REF(x);
                END_VAR
            END_METHOD
            END_FUNCTION_BLOCK
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32* }
    %__vtable_foo_type = type { i32*, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @foo__bar(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 10, i32* %x, align 4
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn initializing_method_variables_with_refs_referencing_parent_pou_variable() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32*, i32 }
    %__vtable_foo_type = type { i32*, i32* }

    @__foo__init = unnamed_addr constant %foo { i32 5 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %px = alloca i32*, align 8
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn initializing_method_variables_with_refs_referencing_global_variable() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32* }
    %__vtable_foo_type = type { i32*, i32* }

    @x = global i32 0
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @foo__bar(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %px = alloca i32*, align 8
      store i32* @x, i32** %px, align 8
      store i32* @x, i32** %px, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn initializing_method_variables_with_refs_shadowing() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32* }
    %__vtable_foo_type = type { i32*, i32* }

    @x = global i32 0
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @foo__bar(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 0, i32* %x, align 4
      store i32* %x, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn initializing_method_variables_with_alias() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            FUNCTION_BLOCK foo
                METHOD bar
                    VAR
                        x : DINT;
                        px AT x : DINT;
                    END_VAR
                END_METHOD
            END_FUNCTION_BLOCK
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32* }
    %__vtable_foo_type = type { i32*, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @foo__bar(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 0, i32* %x, align 4
      store i32* null, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn initializing_method_variables_with_reference_to() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
            FUNCTION_BLOCK foo
                METHOD bar
                VAR
                    x : DINT;
                    px : REFERENCE TO DINT := REF(x);
                END_VAR
                END_METHOD
            END_FUNCTION_BLOCK
            "#,
        )],
    )
    .unwrap();
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32* }
    %__vtable_foo_type = type { i32*, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      ret void
    }

    define void @foo__bar(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = alloca i32, align 4
      %px = alloca i32*, align 8
      store i32 0, i32* %x, align 4
      store i32* null, i32** %px, align 8
      store i32* %x, i32** %px, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
}

#[test]
fn methods_call_init_functions_for_their_members() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
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
        "#,
        )],
    )
    .unwrap();
    // when compiling to ir, we expect `bar.baz` to call `__init_foo` with the local instance.
    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %foo = type { i32*, i32, i32* }
    %bar = type { i32* }
    %__vtable_foo_type = type { i32* }
    %__vtable_bar_type = type { i32*, i32* }

    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer
    @__vtable_bar = global %__vtable_bar_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      ret void
    }

    define void @bar__baz(%bar* %0) {
    entry:
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %fb = alloca %foo, align 8
      %1 = bitcast %foo* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%foo* @__foo__init to i8*), i64 ptrtoint (%foo* getelementptr (%foo, %foo* null, i32 1) to i64), i1 false)
      call void @__init_foo(%foo* %fb)
      call void @__user_init_foo(%foo* %fb)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %y = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 2
      %deref1 = load %foo*, %foo** %self, align 8
      %x = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 1
      store i32* %x, i32** %y, align 8
      %deref2 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref2, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init___vtable_bar_type(%__vtable_bar_type* %0) {
    entry:
      %self = alloca %__vtable_bar_type*, align 8
      store %__vtable_bar_type* %0, %__vtable_bar_type** %self, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__vtable = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__init___vtable_bar_type(%__vtable_bar_type* @__vtable_bar)
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn user_fb_init_is_added_and_called_if_it_exists() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
        FUNCTION_BLOCK foo
        VAR
            x : INT := 0;
            y : INT := 0;
        END_VAR
            METHOD FB_INIT
                x := 1;
                y := 2;
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM prog
        VAR
            f : foo;
        END_VAR
            f();
        END_PROGRAM
        "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %foo }
    %foo = type { i32*, i16, i16 }
    %__vtable_foo_type = type { i32*, i32* }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @foo__FB_INIT(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      store i16 1, i16* %x, align 2
      store i16 2, i16* %y, align 2
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      %f = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      call void @foo(%foo* %f)
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %f = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %f)
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %f = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %f)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      call void @foo__FB_INIT(%foo* %deref)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__user_init_prog(%prog* @prog_instance)
      ret void
    }
    "###);
}

#[test]
fn user_fb_init_in_global_struct() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
        TYPE
            bar : STRUCT
               f: foo;
            END_STRUCT;
        END_TYPE

        VAR_GLOBAL
            str: bar;
        END_VAR

        FUNCTION_BLOCK foo
        VAR
            x : INT := 0;
            y : INT := 0;
        END_VAR
            METHOD FB_INIT
                x := 1;
                y := 2;
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM prog
        VAR
            str: bar;
        END_VAR
            str.f();
        END_PROGRAM
        "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %bar }
    %bar = type { %foo }
    %foo = type { i32*, i16, i16 }
    %__vtable_foo_type = type { i32*, i32* }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @str = global %bar zeroinitializer
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @foo__FB_INIT(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %x = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %y = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      store i16 1, i16* %x, align 2
      store i16 2, i16* %y, align 2
      ret void
    }

    define void @prog(%prog* %0) {
    entry:
      %str = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      %f = getelementptr inbounds %bar, %bar* %str, i32 0, i32 0
      call void @foo(%foo* %f)
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %f = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %f)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %str = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__init_bar(%bar* %str)
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %str = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__user_init_bar(%bar* %str)
      ret void
    }

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %f = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %f)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      call void @foo__FB_INIT(%foo* %deref)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init_bar(%bar* @str)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__user_init_prog(%prog* @prog_instance)
      call void @__user_init_bar(%bar* @str)
      ret void
    }
    "###);
}

#[test]
fn user_init_called_when_declared_as_external() {
    let res = generate_to_string(
        "Test",
        vec![SourceCode::from(
            r#"
        {external}
        FUNCTION_BLOCK foo
        VAR
            x : INT;
            y : INT;
        END_VAR
            METHOD FB_INIT
            END_METHOD
        END_FUNCTION_BLOCK

        PROGRAM prog
        VAR
            f: foo;
        END_VAR
            f();
        END_PROGRAM
        "#,
        )],
    )
    .unwrap();

    filtered_assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %foo }
    %foo = type { i32*, i16, i16 }
    %__vtable_foo_type = type { i32*, i32* }

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = external unnamed_addr constant %foo

    declare void @foo(%foo*)

    declare void @foo__FB_INIT(%foo*)

    define void @prog(%prog* %0) {
    entry:
      %f = getelementptr inbounds %prog, %prog* %0, i32 0, i32 0
      call void @foo(%foo* %f)
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      ret void
    }

    define void @__user_init_prog(%prog* %0) {
    entry:
      %self = alloca %prog*, align 8
      store %prog* %0, %prog** %self, align 8
      %deref = load %prog*, %prog** %self, align 8
      %f = getelementptr inbounds %prog, %prog* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %f)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      call void @foo__FB_INIT(%foo* %deref)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(%prog* @prog_instance)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      call void @__user_init_prog(%prog* @prog_instance)
      ret void
    }
    "###);
}
