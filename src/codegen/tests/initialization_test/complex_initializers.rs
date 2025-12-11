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

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global ptr null

    define void @__init___Test() {
    entry:
      store ptr @s, ptr @ps, align 8
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

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global ptr null

    define void @__init___Test() {
    entry:
      store ptr @s, ptr @ps, align 8
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

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global ptr null

    define void @__init___Test() {
    entry:
      store ptr @s, ptr @ps, align 8
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

    %PLC_PRG = type { ptr }

    @s = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer

    define void @PLC_PRG(ptr %0) {
    entry:
      %to_init = getelementptr inbounds nuw %PLC_PRG, ptr %0, i32 0, i32 0
      ret void
    }

    define void @__init_plc_prg(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %to_init = getelementptr inbounds nuw %PLC_PRG, ptr %deref, i32 0, i32 0
      store ptr @s, ptr %to_init, align 8
      ret void
    }

    define void @__user_init_PLC_PRG(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_plc_prg(ptr @PLC_PRG_instance)
      call void @__user_init_PLC_PRG(ptr @PLC_PRG_instance)
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, ptr }

    @s = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %to_init = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %to_init = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store ptr @s, ptr %to_init, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %mainProg = type { ptr, %foo }
    %foo = type { ptr, ptr, %bar }
    %bar = type { ptr, %baz }
    %baz = type { ptr, ptr }
    %__vtable_baz = type { ptr }
    %__vtable_bar = type { ptr }
    %__vtable_foo = type { ptr }
    %sideProg = type { ptr, %foo }

    @str = global [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @mainProg_instance = global %mainProg zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__baz__init = unnamed_addr constant %baz zeroinitializer
    @____vtable_baz__init = unnamed_addr constant %__vtable_baz zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @sideProg_instance = global %sideProg zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @__vtable_baz_instance = global %__vtable_baz zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      call void @bar__print(ptr %b)
      call void @bar(ptr %b)
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      call void @baz__print(ptr %b)
      ret void
    }

    define void @baz(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 1
      ret void
    }

    define void @mainProg(ptr %0) {
    entry:
      %other_ref_to_global = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 0
      %f = getelementptr inbounds nuw %mainProg, ptr %0, i32 0, i32 1
      ret void
    }

    define void @sideProg(ptr %0) {
    entry:
      %other_ref_to_global = getelementptr inbounds nuw %sideProg, ptr %0, i32 0, i32 0
      %f = getelementptr inbounds nuw %sideProg, ptr %0, i32 0, i32 1
      call void @foo(ptr %f)
      call void @foo__print(ptr %f)
      ret void
    }

    define void @bar__print(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__print(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @baz__print(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 1
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_baz(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @baz, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %b = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 2
      call void @__init_bar(ptr %b)
      %deref1 = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref2 = load ptr, ptr %self, align 8
      %str_ref = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr @str, ptr %str_ref, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %b = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 1
      call void @__init_baz(ptr %b)
      %deref1 = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_baz(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %baz, ptr %deref, i32 0, i32 0
      store ptr @__vtable_baz_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %str_ref = getelementptr inbounds nuw %baz, ptr %deref1, i32 0, i32 1
      store ptr @str, ptr %str_ref, align 8
      ret void
    }

    define void @__init_mainprog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %mainProg, ptr %deref, i32 0, i32 1
      call void @__init_foo(ptr %f)
      %deref1 = load ptr, ptr %self, align 8
      %other_ref_to_global = getelementptr inbounds nuw %mainProg, ptr %deref1, i32 0, i32 0
      store ptr @str, ptr %other_ref_to_global, align 8
      ret void
    }

    define void @__init_sideprog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %mainProg, ptr %deref, i32 0, i32 1
      call void @__init_foo(ptr %f)
      %deref1 = load ptr, ptr %self, align 8
      %other_ref_to_global = getelementptr inbounds nuw %mainProg, ptr %deref1, i32 0, i32 0
      store ptr @str, ptr %other_ref_to_global, align 8
      ret void
    }

    define void @__user_init___vtable_baz(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %b = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 1
      call void @__user_init_baz(ptr %b)
      ret void
    }

    define void @__user_init_baz(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_sideProg(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %mainProg, ptr %deref, i32 0, i32 1
      call void @__user_init_foo(ptr %f)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %b = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 2
      call void @__user_init_bar(ptr %b)
      ret void
    }

    define void @__user_init_mainProg(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %mainProg, ptr %deref, i32 0, i32 1
      call void @__user_init_foo(ptr %f)
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_mainprog(ptr @mainProg_instance)
      call void @__init_sideprog(ptr @sideProg_instance)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__init___vtable_baz(ptr @__vtable_baz_instance)
      call void @__user_init_mainProg(ptr @mainProg_instance)
      call void @__user_init_sideProg(ptr @sideProg_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_baz(ptr @__vtable_baz_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, i16, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %i = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %pi = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %pi = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      %deref2 = load ptr, ptr %self, align 8
      %i = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr %i, ptr %pi, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr, i16, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %i = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %pi = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__FB_INIT(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %i = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %pi = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      store ptr %i, ptr %pi, align 8
      ret void
    }

    define void @main() {
    entry:
      %fb = alloca %foo, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %fb, ptr align 1 @__foo__init, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      call void @__init_foo(ptr %fb)
      call void @__user_init_foo(ptr %fb)
      call void @foo(ptr %fb)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      call void @foo__FB_INIT(ptr %deref)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %myStruct }
    %myStruct = type { ptr, ptr }

    @s = global [81 x i8] c"Hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @s2 = global [2 x [81 x i8]] [[81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"world\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"]
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer

    define void @prog(ptr %0) {
    entry:
      %str = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      ret void
    }

    define void @__init_mystruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %member = getelementptr inbounds nuw %myStruct, ptr %deref, i32 0, i32 0
      store ptr @s, ptr %member, align 8
      %deref1 = load ptr, ptr %self, align 8
      %member2 = getelementptr inbounds nuw %myStruct, ptr %deref1, i32 0, i32 1
      store ptr @s2, ptr %member2, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %str = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__init_mystruct(ptr %str)
      ret void
    }

    define void @__user_init_myStruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %str = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__user_init_myStruct(ptr %str)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__user_init_prog(ptr @prog_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %myStruct = type { i32 }
    %prog = type {}
    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }
    %__vtable_cl = type { ptr }
    %cl = type { ptr }

    @__myStruct__init = unnamed_addr constant %myStruct zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_cl__init = unnamed_addr constant %__vtable_cl zeroinitializer
    @__cl__init = unnamed_addr constant %cl zeroinitializer
    @__vtable_cl_instance = global %__vtable_cl zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__m(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @cl__m(ptr %0) {
    entry:
      %__vtable = getelementptr inbounds nuw %cl, ptr %0, i32 0, i32 0
      ret void
    }

    define void @cl(ptr %0) {
    entry:
      %__vtable = getelementptr inbounds nuw %cl, ptr %0, i32 0, i32 0
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      ret void
    }

    define void @foo__act(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %m = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__m, ptr %m, align 8
      ret void
    }

    define void @__init___vtable_cl(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %m = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @cl__m, ptr %m, align 8
      ret void
    }

    define void @__init_mystruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init_cl(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_cl_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_cl(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_myStruct(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_cl(ptr @__vtable_cl_instance)
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_cl(ptr @__vtable_cl_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type {}
    %foo = type { ptr, ptr }
    %__vtable_foo = type { ptr }

    @ps = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @fb = global %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      call void @foo(ptr @fb)
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %s = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store ptr @ps, ptr %s, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init_foo(ptr @fb)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init_foo(ptr @fb)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %foo }
    %foo = type { ptr, ptr }
    %__vtable_foo = type { ptr }

    @ps = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @global_alias = global %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      %fb = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      call void @foo(ptr %fb)
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %s = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store ptr @ps, ptr %s, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %fb = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %fb)
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %fb = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %fb)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init_foo(ptr @global_alias)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init_foo(ptr @global_alias)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %FB, %FB }
    %FB = type { ptr, ptr }
    %__vtable_FB = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__FB__init = unnamed_addr constant %FB zeroinitializer
    @____vtable_FB__init = unnamed_addr constant %__vtable_FB zeroinitializer
    @__vtable_FB_instance = global %__vtable_FB zeroinitializer
    @__PI_1_2_1 = global i32 0
    @__PI_1_2_2 = global i32 0

    define void @FB(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 0
      %foo = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 1
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      %instance1 = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      %instance2 = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 1
      ret void
    }

    define void @__init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB, ptr %deref, i32 0, i32 0
      store ptr @FB, ptr %__body, align 8
      ret void
    }

    define void @__init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %instance1 = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__init_fb(ptr %instance1)
      %deref1 = load ptr, ptr %self, align 8
      %instance2 = getelementptr inbounds nuw %prog, ptr %deref1, i32 0, i32 1
      call void @__init_fb(ptr %instance2)
      ret void
    }

    define void @__user_init_FB(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %instance1 = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__user_init_FB(ptr %instance1)
      %deref1 = load ptr, ptr %self, align 8
      %instance2 = getelementptr inbounds nuw %prog, ptr %deref1, i32 0, i32 1
      call void @__user_init_FB(ptr %instance2)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init___vtable_fb(ptr @__vtable_FB_instance)
      call void @__init___var_config()
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init___vtable_FB(ptr @__vtable_FB_instance)
      ret void
    }

    define void @__init___var_config() {
    entry:
      store ptr @__PI_1_2_1, ptr getelementptr inbounds nuw (%FB, ptr @prog_instance, i32 0, i32 1), align 8
      store ptr @__PI_1_2_2, ptr getelementptr inbounds nuw (%FB, ptr getelementptr inbounds nuw (%prog, ptr @prog_instance, i32 0, i32 1), i32 0, i32 1), align 8
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @s = global [81 x i8] zeroinitializer
    @refString = global ptr null

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @bar() {
    entry:
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      store ptr @s, ptr @refString, align 8
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#)
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, [81 x i8], ptr, ptr, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %ptr = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %alias = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      %reference_to = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 4
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %ptr = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      %deref2 = load ptr, ptr %self, align 8
      %s = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr %s, ptr %ptr, align 8
      %deref3 = load ptr, ptr %self, align 8
      %alias = getelementptr inbounds nuw %foo, ptr %deref3, i32 0, i32 3
      %deref4 = load ptr, ptr %self, align 8
      %s5 = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 1
      store ptr %s5, ptr %alias, align 8
      %deref6 = load ptr, ptr %self, align 8
      %reference_to = getelementptr inbounds nuw %foo, ptr %deref6, i32 0, i32 4
      %deref7 = load ptr, ptr %self, align 8
      %s8 = getelementptr inbounds nuw %foo, ptr %deref7, i32 0, i32 1
      store ptr %s8, ptr %reference_to, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#)
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, [81 x i8], ptr, ptr, ptr }

    @s = global [81 x i8] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %ptr = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %alias = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      %reference_to = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 4
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %ptr = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      %deref2 = load ptr, ptr %self, align 8
      %s = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr %s, ptr %ptr, align 8
      %deref3 = load ptr, ptr %self, align 8
      %alias = getelementptr inbounds nuw %foo, ptr %deref3, i32 0, i32 3
      %deref4 = load ptr, ptr %self, align 8
      %s5 = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 1
      store ptr %s5, ptr %alias, align 8
      %deref6 = load ptr, ptr %self, align 8
      %reference_to = getelementptr inbounds nuw %foo, ptr %deref6, i32 0, i32 4
      %deref7 = load ptr, ptr %self, align 8
      %s8 = getelementptr inbounds nuw %foo, ptr %deref7, i32 0, i32 1
      store ptr %s8, ptr %reference_to, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#)
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, [81 x i8] }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %ptr = alloca ptr, align 8
      %alias = alloca ptr, align 8
      %reference_to = alloca ptr, align 8
      store ptr %s, ptr %ptr, align 8
      store ptr null, ptr %alias, align 8
      store ptr null, ptr %reference_to, align 8
      store ptr %s, ptr %ptr, align 8
      store ptr %s, ptr %alias, align 8
      store ptr %s, ptr %reference_to, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#)
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

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]

    define void @foo() {
    entry:
      %ptr = alloca ptr, align 8
      %alias = alloca ptr, align 8
      %s = alloca [81 x i8], align 1
      %reference_to = alloca ptr, align 8
      store ptr %s, ptr %ptr, align 8
      store ptr null, ptr %alias, align 8
      call void @llvm.memset.p0.i64(ptr align 1 %s, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      store ptr null, ptr %reference_to, align 8
      store ptr %s, ptr %ptr, align 8
      store ptr %s, ptr %alias, align 8
      %deref = load ptr, ptr %alias, align 8
      store ptr %deref, ptr %reference_to, align 8
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align 4
      %px = alloca ptr, align 8
      store i32 10, ptr %x, align 4
      store ptr %x, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr, i32 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo { ptr null, i32 5 }
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %px = alloca ptr, align 8
      store ptr %x, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }

    @x = global i32 0
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %px = alloca ptr, align 8
      store ptr @x, ptr %px, align 8
      store ptr @x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }

    @x = global i32 0
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align 4
      %px = alloca ptr, align 8
      store i32 0, ptr %x, align 4
      store ptr %x, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align 4
      %px = alloca ptr, align 8
      store i32 0, ptr %x, align 4
      store ptr null, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align 4
      %px = alloca ptr, align 8
      store i32 0, ptr %x, align 4
      store ptr null, ptr %px, align 8
      store ptr %x, ptr %px, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, i32, ptr }
    %__vtable_bar = type { ptr, ptr }
    %bar = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_bar__init = unnamed_addr constant %__vtable_bar zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      ret void
    }

    define void @bar__baz(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %fb = alloca %foo, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %fb, ptr align 1 @__foo__init, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      call void @__init_foo(ptr %fb)
      call void @__user_init_foo(ptr %fb)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 1
      store ptr @bar__baz, ptr %baz, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      %deref1 = load ptr, ptr %self, align 8
      %y = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      %deref2 = load ptr, ptr %self, align 8
      %x = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr %x, ptr %y, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %foo }
    %foo = type { ptr, i16, i16 }
    %__vtable_foo = type { ptr, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__FB_INIT(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      store i16 1, ptr %x, align 2
      store i16 2, ptr %y, align 2
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      %f = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      call void @foo(ptr %f)
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %f)
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %f)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      call void @foo__FB_INIT(ptr %deref)
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %bar }
    %bar = type { %foo }
    %foo = type { ptr, i16, i16 }
    %__vtable_foo = type { ptr, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__bar__init = unnamed_addr constant %bar zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @str = global %bar zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__FB_INIT(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      store i16 1, ptr %x, align 2
      store i16 2, ptr %y, align 2
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      %str = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      %f = getelementptr inbounds nuw %bar, ptr %str, i32 0, i32 0
      call void @foo(ptr %f)
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %f)
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %str = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__init_bar(ptr %str)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      call void @foo__FB_INIT(ptr %deref)
      ret void
    }

    define void @__user_init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %f)
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %str = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__user_init_bar(ptr %str)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init_bar(ptr @str)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init_bar(ptr @str)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
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

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prog = type { %foo }
    %foo = type { ptr, i16, i16 }
    %__vtable_foo = type { ptr, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @prog_instance = global %prog zeroinitializer
    @__foo__init = external unnamed_addr constant %foo
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    declare void @foo(ptr)

    declare void @foo__FB_INIT(ptr)

    define void @prog(ptr %0) {
    entry:
      %f = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      call void @foo(ptr %f)
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT, align 8
      ret void
    }

    define void @__init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_prog(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %f = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %f)
      ret void
    }

    define void @__user_init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      call void @foo__FB_INIT(ptr %deref)
      ret void
    }

    define void @__user_init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init_prog(ptr @prog_instance)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init_prog(ptr @prog_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
}
