use driver::{generate_to_string, generate_to_string_constructors_only};
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

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global ptr null
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [13 x i8] c"hello world!\00"

    define void @__global_ps__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @s, ptr align [filtered] @utf08_literal_0, i32 13, i1 false)
      call void @__global_ps__ctor(ptr @ps)
      store ptr @s, ptr @ps, align [filtered]
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global ptr null
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [13 x i8] c"hello world!\00"

    define void @__global_ps__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @s, ptr align [filtered] @utf08_literal_0, i32 13, i1 false)
      %deref = load ptr, ptr @ps, align [filtered]
      call void @__global_ps__ctor(ptr %deref)
      store ptr @s, ptr @ps, align [filtered]
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    @s = global [81 x i8] c"hello world!\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @ps = global ptr null
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [13 x i8] c"hello world!\00"

    define void @__global_ps__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @s, ptr align [filtered] @utf08_literal_0, i32 13, i1 false)
      %deref = load ptr, ptr @ps, align [filtered]
      call void @__global_ps__ctor(ptr %deref)
      store ptr @s, ptr @ps, align [filtered]
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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
    @PLC_PRG_instance = global %PLC_PRG zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @PLC_PRG(ptr %0) {
    entry:
      %to_init = getelementptr inbounds nuw %PLC_PRG, ptr %0, i32 0, i32 0
      ret void
    }

    define void @PLC_PRG__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %to_init = getelementptr inbounds nuw %PLC_PRG, ptr %deref, i32 0, i32 0
      call void @__PLC_PRG_to_init__ctor(ptr %to_init)
      %deref1 = load ptr, ptr %self, align [filtered]
      %to_init2 = getelementptr inbounds nuw %PLC_PRG, ptr %deref1, i32 0, i32 0
      store ptr @s, ptr %to_init2, align [filtered]
      ret void
    }

    define void @__PLC_PRG_to_init__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @PLC_PRG__ctor(ptr @PLC_PRG_instance)
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
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %to_init = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %to_init = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      call void @__foo_to_init__ctor(ptr %to_init)
      %deref2 = load ptr, ptr %self, align [filtered]
      %to_init3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr @s, ptr %to_init3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @__foo_to_init__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    %__vtable_foo = type { ptr }
    %__vtable_bar = type { ptr }
    %__vtable_baz = type { ptr }
    %mainProg = type { ptr, %foo }
    %foo = type { ptr, ptr, %bar }
    %bar = type { ptr, %baz }
    %baz = type { ptr, ptr }
    %sideProg = type { ptr, %foo }

    @str = global [81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @__vtable_baz_instance = global %__vtable_baz zeroinitializer
    @mainProg_instance = global %mainProg zeroinitializer
    @sideProg_instance = global %sideProg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      call void @bar__print(ptr %b)
      call void @bar(ptr %b)
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      call void @baz__print(ptr %b)
      ret void
    }

    define void @baz(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
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

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %str_ref = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      call void @__foo_str_ref__ctor(ptr %str_ref)
      %deref2 = load ptr, ptr %self, align [filtered]
      %str_ref3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr @str, ptr %str_ref3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 2
      call void @bar__ctor(ptr %b)
      %deref5 = load ptr, ptr %self, align [filtered]
      %__vtable6 = getelementptr inbounds nuw %foo, ptr %deref5, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable6, align [filtered]
      ret void
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__bar___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 1
      call void @baz__ctor(ptr %b)
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %bar, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %baz, ptr %deref, i32 0, i32 0
      call void @__baz___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %str_ref = getelementptr inbounds nuw %baz, ptr %deref1, i32 0, i32 1
      call void @__baz_str_ref__ctor(ptr %str_ref)
      %deref2 = load ptr, ptr %self, align [filtered]
      %str_ref3 = getelementptr inbounds nuw %baz, ptr %deref2, i32 0, i32 1
      store ptr @str, ptr %str_ref3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %baz, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_baz_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @mainProg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %other_ref_to_global = getelementptr inbounds nuw %mainProg, ptr %deref, i32 0, i32 0
      call void @__mainProg_other_ref_to_global__ctor(ptr %other_ref_to_global)
      %deref1 = load ptr, ptr %self, align [filtered]
      %other_ref_to_global2 = getelementptr inbounds nuw %mainProg, ptr %deref1, i32 0, i32 0
      store ptr @str, ptr %other_ref_to_global2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %f = getelementptr inbounds nuw %mainProg, ptr %deref3, i32 0, i32 1
      call void @foo__ctor(ptr %f)
      ret void
    }

    define void @sideProg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %other_ref_to_global = getelementptr inbounds nuw %sideProg, ptr %deref, i32 0, i32 0
      call void @__sideProg_other_ref_to_global__ctor(ptr %other_ref_to_global)
      %deref1 = load ptr, ptr %self, align [filtered]
      %other_ref_to_global2 = getelementptr inbounds nuw %sideProg, ptr %deref1, i32 0, i32 0
      store ptr @str, ptr %other_ref_to_global2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %f = getelementptr inbounds nuw %sideProg, ptr %deref3, i32 0, i32 1
      call void @foo__ctor(ptr %f)
      ret void
    }

    define void @__foo_str_ref__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__baz_str_ref__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__mainProg_other_ref_to_global__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__sideProg_other_ref_to_global__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0
      call void @____vtable_bar___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0
      store ptr @bar, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_baz, ptr %deref, i32 0, i32 0
      call void @____vtable_baz___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_baz, ptr %deref1, i32 0, i32 0
      store ptr @baz, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__bar___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__baz___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_baz___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @str, ptr align [filtered] @utf08_literal_0, i32 6, i1 false)
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance)
      call void @__vtable_baz__ctor(ptr @__vtable_baz_instance)
      call void @mainProg__ctor(ptr @mainProg_instance)
      call void @sideProg__ctor(ptr @sideProg_instance)
      ret void
    }

    define void @bar__print(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__print(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @baz__print(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 0
      %str_ref = getelementptr inbounds nuw %baz, ptr %0, i32 0, i32 1
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %i = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %pi = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %pi = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      call void @__foo_pi__ctor(ptr %pi)
      %deref2 = load ptr, ptr %self, align [filtered]
      %pi3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 2
      %deref4 = load ptr, ptr %self, align [filtered]
      %i = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 1
      store ptr %i, ptr %pi3, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %__vtable6 = getelementptr inbounds nuw %foo, ptr %deref5, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable6, align [filtered]
      ret void
    }

    define void @__foo_pi__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %i = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %pi = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__FB_INIT(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %i = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %pi = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      store ptr %i, ptr %pi, align [filtered]
      ret void
    }

    define void @main() {
    entry:
      %fb = alloca %foo, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %fb, i8 0, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      call void @foo__ctor(ptr %fb)
      call void @foo(ptr %fb)
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %pi = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      call void @__foo_pi__ctor(ptr %pi)
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      call void @foo__FB_INIT(ptr %deref4)
      ret void
    }

    define void @__foo_pi__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_FB_INIT__ctor(ptr %FB_INIT)
      %deref4 = load ptr, ptr %self, align [filtered]
      %FB_INIT5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_FB_INIT__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
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
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [13 x i8] c"Hello world!\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_2 = private unnamed_addr constant [6 x i8] c"world\00"

    define void @prog(ptr %0) {
    entry:
      %str = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      ret void
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %str = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @myStruct__ctor(ptr %str)
      ret void
    }

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %member = getelementptr inbounds nuw %myStruct, ptr %deref, i32 0, i32 0
      call void @__myStruct_member__ctor(ptr %member)
      %deref1 = load ptr, ptr %self, align [filtered]
      %member2 = getelementptr inbounds nuw %myStruct, ptr %deref1, i32 0, i32 0
      store ptr @s, ptr %member2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %member24 = getelementptr inbounds nuw %myStruct, ptr %deref3, i32 0, i32 1
      %deref5 = load ptr, ptr %member24, align [filtered]
      call void @__myStruct_member2__ctor(ptr %deref5)
      %deref6 = load ptr, ptr %self, align [filtered]
      %member27 = getelementptr inbounds nuw %myStruct, ptr %deref6, i32 0, i32 1
      store ptr @s2, ptr %member27, align [filtered]
      ret void
    }

    define void @__global_s2__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__myStruct_member__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__myStruct_member2___ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__myStruct_member2__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @s, ptr align [filtered] @utf08_literal_0, i32 13, i1 false)
      call void @__global_s2__ctor(ptr @s2)
      store [2 x [81 x i8]] [[81 x i8] c"hello\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", [81 x i8] c"world\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"], ptr @s2, align [filtered]
      call void @prog__ctor(ptr @prog_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    %__vtable_foo = type { ptr, ptr }
    %__vtable_cl = type { ptr }
    %prog = type {}
    %foo = type { ptr }
    %cl = type { ptr }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_cl_instance = global %__vtable_cl zeroinitializer
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__m(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
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

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @cl__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %cl, ptr %deref, i32 0, i32 0
      call void @__cl___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %cl, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_cl_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @myStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %m = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_m__ctor(ptr %m)
      %deref4 = load ptr, ptr %self, align [filtered]
      %m5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__m, ptr %m5, align [filtered]
      ret void
    }

    define void @__vtable_cl__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %m = getelementptr inbounds nuw %__vtable_cl, ptr %deref, i32 0, i32 0
      call void @____vtable_cl_m__ctor(ptr %m)
      %deref1 = load ptr, ptr %self, align [filtered]
      %m2 = getelementptr inbounds nuw %__vtable_cl, ptr %deref1, i32 0, i32 0
      store ptr @cl__m, ptr %m2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__cl___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_m__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_cl_m__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @__vtable_cl__ctor(ptr @__vtable_cl_instance)
      call void @prog__ctor(ptr @prog_instance)
      ret void
    }

    define void @foo__act(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
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

    %foo = type { ptr, ptr }
    %__vtable_foo = type { ptr }
    %prog = type {}

    @ps = global [81 x i8] zeroinitializer
    @fb = global %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      call void @foo(ptr @fb)
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      call void @__foo_s__ctor(ptr %s)
      %deref2 = load ptr, ptr %self, align [filtered]
      %s3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr @ps, ptr %s3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_s__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @foo__ctor(ptr @fb)
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @prog__ctor(ptr @prog_instance)
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

    %foo = type { ptr, ptr }
    %__vtable_foo = type { ptr }
    %prog = type { %foo }

    @ps = global [81 x i8] zeroinitializer
    @global_alias = global %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
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

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      call void @__foo_s__ctor(ptr %s)
      %deref2 = load ptr, ptr %self, align [filtered]
      %s3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 1
      store ptr @ps, ptr %s3, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %__vtable5 = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable5, align [filtered]
      ret void
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %fb = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @alias__ctor(ptr %fb)
      ret void
    }

    define void @alias__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      call void @foo__ctor(ptr %deref)
      ret void
    }

    define void @__foo_s__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @alias__ctor(ptr @global_alias)
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @prog__ctor(ptr @prog_instance)
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

    %__vtable_FB = type { ptr }
    %prog = type { %FB, %FB }
    %FB = type { ptr, ptr }

    @__PI_1_2_1 = global i32 0
    @__PI_1_2_2 = global i32 0
    @__vtable_FB_instance = global %__vtable_FB zeroinitializer
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @FB(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
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

    define void @FB__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %FB, ptr %deref, i32 0, i32 0
      call void @__FB___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %foo = getelementptr inbounds nuw %FB, ptr %deref1, i32 0, i32 1
      %deref2 = load ptr, ptr %foo, align [filtered]
      call void @__FB_foo__ctor(ptr %deref2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %FB, ptr %deref3, i32 0, i32 0
      store ptr @__vtable_FB_instance, ptr %__vtable4, align [filtered]
      ret void
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %instance1 = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @FB__ctor(ptr %instance1)
      %deref1 = load ptr, ptr %self, align [filtered]
      %instance2 = getelementptr inbounds nuw %prog, ptr %deref1, i32 0, i32 1
      call void @FB__ctor(ptr %instance2)
      ret void
    }

    define void @__FB_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_FB__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_FB, ptr %deref, i32 0, i32 0
      call void @____vtable_FB___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_FB, ptr %deref1, i32 0, i32 0
      store ptr @FB, ptr %__body2, align [filtered]
      ret void
    }

    define void @__FB___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_FB___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_FB__ctor(ptr @__vtable_FB_instance)
      store ptr @__PI_1_2_1, ptr getelementptr inbounds nuw (%FB, ptr @prog_instance, i32 0, i32 1), align [filtered]
      store ptr @__PI_1_2_2, ptr getelementptr inbounds nuw (%FB, ptr getelementptr inbounds nuw (%prog, ptr @prog_instance, i32 0, i32 1), i32 0, i32 1), align [filtered]
      call void @prog__ctor(ptr @prog_instance)
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

    @s = global [81 x i8] zeroinitializer
    @refString = global ptr null
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @bar() {
    entry:
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__global_refString__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      %deref = load ptr, ptr @refString, align [filtered]
      call void @__global_refString__ctor(ptr %deref)
      store ptr @s, ptr @refString, align [filtered]
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %ptr = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %alias = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      %reference_to = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 4
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %ptr = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      call void @__foo_ptr__ctor(ptr %ptr)
      %deref2 = load ptr, ptr %self, align [filtered]
      %ptr3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 2
      %deref4 = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 1
      store ptr %s, ptr %ptr3, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %alias = getelementptr inbounds nuw %foo, ptr %deref5, i32 0, i32 3
      %deref6 = load ptr, ptr %alias, align [filtered]
      call void @__foo_alias__ctor(ptr %deref6)
      %deref7 = load ptr, ptr %self, align [filtered]
      %alias8 = getelementptr inbounds nuw %foo, ptr %deref7, i32 0, i32 3
      %deref9 = load ptr, ptr %self, align [filtered]
      %s10 = getelementptr inbounds nuw %foo, ptr %deref9, i32 0, i32 1
      store ptr %s10, ptr %alias8, align [filtered]
      %deref11 = load ptr, ptr %self, align [filtered]
      %reference_to = getelementptr inbounds nuw %foo, ptr %deref11, i32 0, i32 4
      %deref12 = load ptr, ptr %reference_to, align [filtered]
      call void @__foo_reference_to__ctor(ptr %deref12)
      %deref13 = load ptr, ptr %self, align [filtered]
      %reference_to14 = getelementptr inbounds nuw %foo, ptr %deref13, i32 0, i32 4
      %deref15 = load ptr, ptr %self, align [filtered]
      %s16 = getelementptr inbounds nuw %foo, ptr %deref15, i32 0, i32 1
      store ptr %s16, ptr %reference_to14, align [filtered]
      %deref17 = load ptr, ptr %self, align [filtered]
      %__vtable18 = getelementptr inbounds nuw %foo, ptr %deref17, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable18, align [filtered]
      ret void
    }

    define void @__foo_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_alias__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_reference_to__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %ptr = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %alias = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      %reference_to = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 4
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %ptr = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      call void @__foo_ptr__ctor(ptr %ptr)
      %deref2 = load ptr, ptr %self, align [filtered]
      %ptr3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 2
      %deref4 = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %foo, ptr %deref4, i32 0, i32 1
      store ptr %s, ptr %ptr3, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %alias = getelementptr inbounds nuw %foo, ptr %deref5, i32 0, i32 3
      %deref6 = load ptr, ptr %alias, align [filtered]
      call void @__foo_alias__ctor(ptr %deref6)
      %deref7 = load ptr, ptr %self, align [filtered]
      %alias8 = getelementptr inbounds nuw %foo, ptr %deref7, i32 0, i32 3
      %deref9 = load ptr, ptr %self, align [filtered]
      %s10 = getelementptr inbounds nuw %foo, ptr %deref9, i32 0, i32 1
      store ptr %s10, ptr %alias8, align [filtered]
      %deref11 = load ptr, ptr %self, align [filtered]
      %reference_to = getelementptr inbounds nuw %foo, ptr %deref11, i32 0, i32 4
      %deref12 = load ptr, ptr %reference_to, align [filtered]
      call void @__foo_reference_to__ctor(ptr %deref12)
      %deref13 = load ptr, ptr %self, align [filtered]
      %reference_to14 = getelementptr inbounds nuw %foo, ptr %deref13, i32 0, i32 4
      %deref15 = load ptr, ptr %self, align [filtered]
      %s16 = getelementptr inbounds nuw %foo, ptr %deref15, i32 0, i32 1
      store ptr %s16, ptr %reference_to14, align [filtered]
      %deref17 = load ptr, ptr %self, align [filtered]
      %__vtable18 = getelementptr inbounds nuw %foo, ptr %deref17, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable18, align [filtered]
      ret void
    }

    define void @__foo_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_alias__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_reference_to__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %ptr = alloca ptr, align [filtered]
      %alias = alloca ptr, align [filtered]
      %reference_to = alloca ptr, align [filtered]
      store ptr %s, ptr %ptr, align [filtered]
      store ptr null, ptr %alias, align [filtered]
      store ptr null, ptr %reference_to, align [filtered]
      call void @__foo_ptr__ctor(ptr %ptr)
      store ptr %s, ptr %ptr, align [filtered]
      %deref = load ptr, ptr %alias, align [filtered]
      call void @__foo_alias__ctor(ptr %deref)
      store ptr %s, ptr %alias, align [filtered]
      %deref1 = load ptr, ptr %reference_to, align [filtered]
      call void @__foo_reference_to__ctor(ptr %deref1)
      store ptr %s, ptr %reference_to, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_alias__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_reference_to__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    define void @foo() {
    entry:
      %ptr = alloca ptr, align [filtered]
      %alias = alloca ptr, align [filtered]
      %s = alloca [81 x i8], align [filtered]
      %reference_to = alloca ptr, align [filtered]
      store ptr %s, ptr %ptr, align [filtered]
      store ptr null, ptr %alias, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %s, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      store ptr null, ptr %reference_to, align [filtered]
      call void @__foo_ptr__ctor(ptr %ptr)
      store ptr %s, ptr %ptr, align [filtered]
      %deref = load ptr, ptr %alias, align [filtered]
      call void @__foo_alias__ctor(ptr %deref)
      store ptr %s, ptr %alias, align [filtered]
      %deref1 = load ptr, ptr %reference_to, align [filtered]
      call void @__foo_reference_to__ctor(ptr %deref1)
      %deref2 = load ptr, ptr %alias, align [filtered]
      store ptr %deref2, ptr %reference_to, align [filtered]
      ret void
    }

    define void @__foo_ptr__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_alias__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__foo_reference_to__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align [filtered]
      %px = alloca ptr, align [filtered]
      store i32 10, ptr %x, align [filtered]
      store ptr %x, ptr %px, align [filtered]
      store i32 10, ptr %x, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %px = alloca ptr, align [filtered]
      store ptr %x, ptr %px, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store i32 5, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %px = alloca ptr, align [filtered]
      store ptr @x, ptr %px, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr @x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align [filtered]
      %px = alloca ptr, align [filtered]
      store i32 0, ptr %x, align [filtered]
      store ptr %x, ptr %px, align [filtered]
      call void @__foo.bar_px__ctor(ptr %px)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align [filtered]
      %px = alloca ptr, align [filtered]
      store i32 0, ptr %x, align [filtered]
      store ptr null, ptr %px, align [filtered]
      %deref = load ptr, ptr %px, align [filtered]
      call void @__foo.bar_px__ctor(ptr %deref)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo__bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = alloca i32, align [filtered]
      %px = alloca ptr, align [filtered]
      store i32 0, ptr %x, align [filtered]
      store ptr null, ptr %px, align [filtered]
      %deref = load ptr, ptr %px, align [filtered]
      call void @__foo.bar_px__ctor(ptr %deref)
      store ptr %x, ptr %px, align [filtered]
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo.bar_px__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %bar = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_bar__ctor(ptr %bar)
      %deref4 = load ptr, ptr %self, align [filtered]
      %bar5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__bar, ptr %bar5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
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
    %__vtable_bar = type { ptr, ptr }
    %foo = type { ptr, i32, ptr }
    %bar = type { ptr }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @__vtable_bar_instance = global %__vtable_bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      ret void
    }

    define void @bar__baz(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %fb = alloca %foo, align [filtered]
      call void @llvm.memset.p0.i64(ptr align [filtered] %fb, i8 0, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      call void @foo__ctor(ptr %fb)
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %y = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 2
      %deref2 = load ptr, ptr %y, align [filtered]
      call void @__foo_y__ctor(ptr %deref2)
      %deref3 = load ptr, ptr %self, align [filtered]
      %y4 = getelementptr inbounds nuw %foo, ptr %deref3, i32 0, i32 2
      %deref5 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %foo, ptr %deref5, i32 0, i32 1
      store ptr %x, ptr %y4, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %__vtable7 = getelementptr inbounds nuw %foo, ptr %deref6, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable7, align [filtered]
      ret void
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__bar___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__vtable2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable2, align [filtered]
      ret void
    }

    define void @__foo_y__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      ret void
    }

    define void @__vtable_bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_bar, ptr %deref, i32 0, i32 0
      call void @____vtable_bar___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_bar, ptr %deref1, i32 0, i32 0
      store ptr @bar, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %baz = getelementptr inbounds nuw %__vtable_bar, ptr %deref3, i32 0, i32 1
      call void @____vtable_bar_baz__ctor(ptr %baz)
      %deref4 = load ptr, ptr %self, align [filtered]
      %baz5 = getelementptr inbounds nuw %__vtable_bar, ptr %deref4, i32 0, i32 1
      store ptr @bar__baz, ptr %baz5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__bar___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_bar___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_bar_baz__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @__vtable_bar__ctor(ptr @__vtable_bar_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
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

    %__vtable_foo = type { ptr, ptr }
    %prog = type { %foo }
    %foo = type { ptr, i16, i16 }

    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__FB_INIT(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      store i16 1, ptr %x, align [filtered]
      store i16 2, ptr %y, align [filtered]
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      %f = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      call void @foo(ptr %f)
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store i16 0, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %y = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 2
      store i16 0, ptr %y, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %foo, ptr %deref3, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      call void @foo__FB_INIT(ptr %deref5)
      ret void
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %f = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @foo__ctor(ptr %f)
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_FB_INIT__ctor(ptr %FB_INIT)
      %deref4 = load ptr, ptr %self, align [filtered]
      %FB_INIT5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_FB_INIT__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @prog__ctor(ptr @prog_instance)
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

    %bar = type { %foo }
    %foo = type { ptr, i16, i16 }
    %__vtable_foo = type { ptr, ptr }
    %prog = type { %bar }

    @str = global %bar zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      ret void
    }

    define void @foo__FB_INIT(ptr %0) {
    entry:
      %this = alloca ptr, align [filtered]
      store ptr %0, ptr %this, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      store i16 1, ptr %x, align [filtered]
      store i16 2, ptr %y, align [filtered]
      ret void
    }

    define void @prog(ptr %0) {
    entry:
      %str = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      %f = getelementptr inbounds nuw %bar, ptr %str, i32 0, i32 0
      call void @foo(ptr %f)
      ret void
    }

    define void @foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      call void @__foo___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 1
      store i16 0, ptr %x, align [filtered]
      %deref2 = load ptr, ptr %self, align [filtered]
      %y = getelementptr inbounds nuw %foo, ptr %deref2, i32 0, i32 2
      store i16 0, ptr %y, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %__vtable4 = getelementptr inbounds nuw %foo, ptr %deref3, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      call void @foo__FB_INIT(ptr %deref5)
      ret void
    }

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %str = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @bar__ctor(ptr %str)
      ret void
    }

    define void @bar__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %f = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @foo__ctor(ptr %f)
      ret void
    }

    define void @__vtable_foo__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      call void @____vtable_foo___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 0
      store ptr @foo, ptr %__body2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %FB_INIT = getelementptr inbounds nuw %__vtable_foo, ptr %deref3, i32 0, i32 1
      call void @____vtable_foo_FB_INIT__ctor(ptr %FB_INIT)
      %deref4 = load ptr, ptr %self, align [filtered]
      %FB_INIT5 = getelementptr inbounds nuw %__vtable_foo, ptr %deref4, i32 0, i32 1
      store ptr @foo__FB_INIT, ptr %FB_INIT5, align [filtered]
      ret void
    }

    define void @__foo___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_foo_FB_INIT__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @bar__ctor(ptr @str)
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @prog__ctor(ptr @prog_instance)
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

    %__vtable_foo = type { ptr, ptr }
    %prog = type { %foo }
    %foo = type { ptr, i16, i16 }

    @__vtable_foo_instance = external global %__vtable_foo
    @prog_instance = global %prog zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    declare void @foo(ptr)

    declare void @foo__FB_INIT(ptr)

    define void @prog(ptr %0) {
    entry:
      %f = getelementptr inbounds nuw %prog, ptr %0, i32 0, i32 0
      call void @foo(ptr %f)
      ret void
    }

    declare void @foo__ctor(ptr)

    define void @prog__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %f = getelementptr inbounds nuw %prog, ptr %deref, i32 0, i32 0
      call void @foo__ctor(ptr %f)
      ret void
    }

    declare void @__vtable_foo__ctor(ptr)

    declare void @__foo___vtable__ctor(ptr)

    declare void @____vtable_foo___body__ctor(ptr)

    declare void @____vtable_foo_FB_INIT__ctor(ptr)

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_foo__ctor(ptr @__vtable_foo_instance)
      call void @prog__ctor(ptr @prog_instance)
      ret void
    }
    "#);
}

#[test]
fn constructors_only_emits_only_ctor_definitions() {
    let result = generate_to_string_constructors_only(
        "Test",
        vec![SourceCode::from(
            r#"
            TYPE MyStruct : STRUCT
                x : DINT := 10;
                y : DINT;
            END_STRUCT
            END_TYPE

            FUNCTION_BLOCK MyFB
            VAR
                s : MyStruct;
            END_VAR
                s.y := s.x + 1;
            END_FUNCTION_BLOCK

            PROGRAM main
            VAR
                fb : MyFB;
            END_VAR
                fb();
            END_PROGRAM
            "#,
        )],
    )
    .unwrap();

    // User POU bodies (main, MyFB) should be `declare` stubs,
    // while constructors (__ctor) should be `define`d.
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_MyFB = type { ptr }
    %main = type { %MyFB }
    %MyFB = type { ptr, %MyStruct }
    %MyStruct = type { i32, i32 }

    @__vtable_MyFB_instance = global %__vtable_MyFB zeroinitializer
    @main_instance = global %main { %MyFB { ptr null, %MyStruct { i32 10, i32 0 } } }
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    declare void @MyFB(ptr)

    declare void @main(ptr)

    define void @MyFB__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__vtable = getelementptr inbounds nuw %MyFB, ptr %deref, i32 0, i32 0
      call void @__MyFB___vtable__ctor(ptr %__vtable)
      %deref1 = load ptr, ptr %self, align [filtered]
      %s = getelementptr inbounds nuw %MyFB, ptr %deref1, i32 0, i32 1
      call void @MyStruct__ctor(ptr %s)
      %deref2 = load ptr, ptr %self, align [filtered]
      %__vtable3 = getelementptr inbounds nuw %MyFB, ptr %deref2, i32 0, i32 0
      store ptr @__vtable_MyFB_instance, ptr %__vtable3, align [filtered]
      ret void
    }

    define void @main__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %fb = getelementptr inbounds nuw %main, ptr %deref, i32 0, i32 0
      call void @MyFB__ctor(ptr %fb)
      ret void
    }

    define void @MyStruct__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %x = getelementptr inbounds nuw %MyStruct, ptr %deref, i32 0, i32 0
      store i32 10, ptr %x, align [filtered]
      ret void
    }

    define void @__vtable_MyFB__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %__body = getelementptr inbounds nuw %__vtable_MyFB, ptr %deref, i32 0, i32 0
      call void @____vtable_MyFB___body__ctor(ptr %__body)
      %deref1 = load ptr, ptr %self, align [filtered]
      %__body2 = getelementptr inbounds nuw %__vtable_MyFB, ptr %deref1, i32 0, i32 0
      store ptr @MyFB, ptr %__body2, align [filtered]
      ret void
    }

    define void @__MyFB___vtable__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @____vtable_MyFB___body__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__vtable_MyFB__ctor(ptr @__vtable_MyFB_instance)
      call void @main__ctor(ptr @main_instance)
      ret void
    }
    "#);
}
