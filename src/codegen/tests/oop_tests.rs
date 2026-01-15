use plc_util::filtered_assert_snapshot;
use test_utils::codegen;
mod debug_tests;
mod super_tests;

#[test]
fn members_from_base_class_are_available_in_subclasses() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR
            a : INT;
            b : STRING;
            c : ARRAY[0..10] OF STRING;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
        END_FUNCTION_BLOCK
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr }
    %foo = type { ptr, i16, [81 x i8], [11 x [81 x i8]] }
    %__vtable_bar = type { ptr }
    %bar = type { %foo }

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
      %a = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 2
      %c = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 3
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
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

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
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
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
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
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      ret void
    }
    "#);
}

#[test]
fn write_to_parent_variable_qualified_access() {
    let res = codegen(
        "
        FUNCTION_BLOCK fb
        VAR
            x : INT;
            y : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK fb2 EXTENDS fb
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK foo
        VAR
            myFb : fb2;
        END_VAR
            myFb.x := 1;
        END_FUNCTION_BLOCK
       ",
    );

    filtered_assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { ptr }
    %fb = type { ptr, i16, i16 }
    %__vtable_fb2 = type { ptr }
    %fb2 = type { %fb }
    %__vtable_foo = type { ptr }
    %foo = type { ptr, %fb2 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer
    @__fb__init = unnamed_addr constant %fb zeroinitializer
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer
    @____vtable_fb2__init = unnamed_addr constant %__vtable_fb2 zeroinitializer
    @__fb2__init = unnamed_addr constant %fb2 zeroinitializer
    @__vtable_fb2_instance = global %__vtable_fb2 zeroinitializer
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer

    define void @fb(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 1
      %y = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 2
      ret void
    }

    define void @fb2(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__fb = getelementptr inbounds nuw %fb2, ptr %0, i32 0, i32 0
      ret void
    }

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %myFb = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      %__fb = getelementptr inbounds nuw %fb2, ptr %myFb, i32 0, i32 0
      %x = getelementptr inbounds nuw %fb, ptr %__fb, i32 0, i32 1
      store i16 1, ptr %x, align 2
      ret void
    }

    define void @__init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @fb, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @fb2, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @foo, ptr %__body, align 8
      ret void
    }

    define void @__init_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__fb = getelementptr inbounds nuw %fb2, ptr %deref, i32 0, i32 0
      call void @__init_fb(ptr %__fb)
      %deref1 = load ptr, ptr %self, align 8
      %__fb2 = getelementptr inbounds nuw %fb2, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %fb, ptr %__fb2, i32 0, i32 0
      store ptr @__vtable_fb2_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %deref, i32 0, i32 0
      store ptr @__vtable_fb_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_foo(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %myFb = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 1
      call void @__init_fb2(ptr %myFb)
      %deref1 = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %deref1, i32 0, i32 0
      store ptr @__vtable_foo_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_fb2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__fb = getelementptr inbounds nuw %fb2, ptr %deref, i32 0, i32 0
      call void @__user_init_fb(ptr %__fb)
      ret void
    }

    define void @__user_init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_fb2(ptr %0) {
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
      %deref = load ptr, ptr %self, align 8
      %myFb = getelementptr inbounds nuw %foo, ptr %deref, i32 0, i32 1
      call void @__user_init_fb2(ptr %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(ptr @__vtable_fb_instance)
      call void @__init___vtable_fb2(ptr @__vtable_fb2_instance)
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_fb(ptr @__vtable_fb_instance)
      call void @__user_init___vtable_fb2(ptr @__vtable_fb2_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      ret void
    }
    "#);
}

#[test]
fn write_to_parent_variable_in_instance() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
        END_VAR
        METHOD baz
            s := 'hello';
        END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK bar EXTENDS foo
            s := 'world';
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR
            s: STRING;
            fb: bar;
        END_VAR
            fb.baz();
            fb();
        END_FUNCTION
    "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr, [81 x i8] }
    %__vtable_bar = type { ptr, ptr }
    %bar = type { %foo }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
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
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      ret void
    }

    define void @foo__baz(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %s, ptr align 1 @utf08_literal_0, i32 6, i1 false)
      ret void
    }

    define void @bar(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %0, i32 0, i32 0
      %s = getelementptr inbounds nuw %foo, ptr %__foo, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %s, ptr align 1 @utf08_literal_1, i32 6, i1 false)
      ret void
    }

    define void @main() {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      call void @llvm.memset.p0.i64(ptr align 1 %s, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %fb, ptr align 1 @__bar__init, i64 ptrtoint (ptr getelementptr (%bar, ptr null, i32 1) to i64), i1 false)
      call void @__init_bar(ptr %fb)
      call void @__user_init_bar(ptr %fb)
      %__foo = getelementptr inbounds nuw %bar, ptr %fb, i32 0, i32 0
      call void @foo__baz(ptr %__foo)
      call void @bar(ptr %fb)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

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
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init___vtable_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @bar, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %baz = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__baz, ptr %baz, align 8
      ret void
    }

    define void @__init_bar(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %bar, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
      store ptr @__vtable_bar_instance, ptr %__vtable, align 8
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
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %bar, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
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
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_bar(ptr @__vtable_bar_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_bar(ptr @__vtable_bar_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

#[test]
fn array_in_parent_generated() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK grandparent
        VAR
            y : ARRAY[0..5] OF INT;
            a : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent extends grandparent
            VAR
                x : ARRAY[0..10] OF INT;
                b : INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                z : ARRAY[0..10] OF INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main
        VAR
            arr: ARRAY[0..10] OF child;
        END_VAR
            arr[0].a := 10;
            arr[0].y[0] := 20;
            arr[1].b := 30;
            arr[1].x[1] := 40;
            arr[2].z[2] := 50;
        END_FUNCTION
        "#,
    );
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_grandparent = type { ptr }
    %grandparent = type { ptr, [6 x i16], i16 }
    %__vtable_parent = type { ptr }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %a = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      ret void
    }

    define void @main() {
    entry:
      %arr = alloca [11 x %child], align 8
      call void @llvm.memset.p0.i64(ptr align 1 %arr, i8 0, i64 ptrtoint (ptr getelementptr ([11 x %child], ptr null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 0
      %__parent = getelementptr inbounds nuw %child, ptr %tmpVar, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0
      %a = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 2
      store i16 10, ptr %a, align 2
      %tmpVar1 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 0
      %__parent2 = getelementptr inbounds nuw %child, ptr %tmpVar1, i32 0, i32 0
      %__grandparent3 = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %__grandparent3, i32 0, i32 1
      %tmpVar4 = getelementptr inbounds [6 x i16], ptr %y, i32 0, i32 0
      store i16 20, ptr %tmpVar4, align 2
      %tmpVar5 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 1
      %__parent6 = getelementptr inbounds nuw %child, ptr %tmpVar5, i32 0, i32 0
      %b = getelementptr inbounds nuw %parent, ptr %__parent6, i32 0, i32 2
      store i16 30, ptr %b, align 2
      %tmpVar7 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 1
      %__parent8 = getelementptr inbounds nuw %child, ptr %tmpVar7, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %__parent8, i32 0, i32 1
      %tmpVar9 = getelementptr inbounds [11 x i16], ptr %x, i32 0, i32 1
      store i16 40, ptr %tmpVar9, align 2
      %tmpVar10 = getelementptr inbounds [11 x %child], ptr %arr, i32 0, i32 2
      %z = getelementptr inbounds nuw %child, ptr %tmpVar10, i32 0, i32 1
      %tmpVar11 = getelementptr inbounds [11 x i16], ptr %z, i32 0, i32 2
      store i16 50, ptr %tmpVar11, align 2
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #0

    define void @__init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @grandparent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__init_grandparent(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align 8
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__user_init_grandparent(ptr %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

#[test]
fn complex_array_access_generated() {
    let result = codegen(
        r#"
        FUNCTION_BLOCK grandparent
        VAR
            y : ARRAY[0..5] OF INT;
            a : INT;
        END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent extends grandparent
            VAR
                x : ARRAY[0..10] OF INT;
                b : INT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
            VAR
                z : ARRAY[0..10] OF INT;
            END_VAR
            y[b + z[b*2] - a] := 20;
        END_FUNCTION_BLOCK
        "#,
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_grandparent = type { ptr }
    %grandparent = type { ptr, [6 x i16], i16 }
    %__vtable_parent = type { ptr }
    %parent = type { %grandparent, [11 x i16], i16 }
    %__vtable_child = type { ptr }
    %child = type { %parent, [11 x i16] }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_grandparent__init = unnamed_addr constant %__vtable_grandparent zeroinitializer
    @__grandparent__init = unnamed_addr constant %grandparent zeroinitializer
    @__vtable_grandparent_instance = global %__vtable_grandparent zeroinitializer
    @____vtable_parent__init = unnamed_addr constant %__vtable_parent zeroinitializer
    @__parent__init = unnamed_addr constant %parent zeroinitializer
    @__vtable_parent_instance = global %__vtable_parent zeroinitializer
    @____vtable_child__init = unnamed_addr constant %__vtable_child zeroinitializer
    @__child__init = unnamed_addr constant %child zeroinitializer
    @__vtable_child_instance = global %__vtable_child zeroinitializer

    define void @grandparent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 1
      %a = getelementptr inbounds nuw %grandparent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @parent(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %0, i32 0, i32 2
      ret void
    }

    define void @child(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 0
      %z = getelementptr inbounds nuw %child, ptr %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0
      %y = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 1
      %b = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_b = load i16, ptr %b, align 2
      %1 = sext i16 %load_b to i32
      %b1 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 2
      %load_b2 = load i16, ptr %b1, align 2
      %2 = sext i16 %load_b2 to i32
      %tmpVar = mul i32 %2, 2
      %tmpVar3 = mul i32 1, %tmpVar
      %tmpVar4 = add i32 %tmpVar3, 0
      %tmpVar5 = getelementptr inbounds [11 x i16], ptr %z, i32 0, i32 %tmpVar4
      %load_tmpVar = load i16, ptr %tmpVar5, align 2
      %3 = sext i16 %load_tmpVar to i32
      %tmpVar6 = add i32 %1, %3
      %__grandparent7 = getelementptr inbounds nuw %parent, ptr %__parent, i32 0, i32 0
      %a = getelementptr inbounds nuw %grandparent, ptr %__grandparent7, i32 0, i32 2
      %load_a = load i16, ptr %a, align 2
      %4 = sext i16 %load_a to i32
      %tmpVar8 = sub i32 %tmpVar6, %4
      %tmpVar9 = mul i32 1, %tmpVar8
      %tmpVar10 = add i32 %tmpVar9, 0
      %tmpVar11 = getelementptr inbounds [6 x i16], ptr %y, i32 0, i32 %tmpVar10
      store i16 20, ptr %tmpVar11, align 2
      ret void
    }

    define void @__init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @grandparent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @parent, ptr %__body, align 8
      ret void
    }

    define void @__init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_grandparent, ptr %deref, i32 0, i32 0
      store ptr @child, ptr %__body, align 8
      ret void
    }

    define void @__init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__init_grandparent(ptr %__grandparent)
      %deref1 = load ptr, ptr %self, align 8
      %__grandparent2 = getelementptr inbounds nuw %parent, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent2, i32 0, i32 0
      store ptr @__vtable_parent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %deref, i32 0, i32 0
      store ptr @__vtable_grandparent_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__init_parent(ptr %__parent)
      %deref1 = load ptr, ptr %self, align 8
      %__parent2 = getelementptr inbounds nuw %child, ptr %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds nuw %parent, ptr %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %grandparent, ptr %__grandparent, i32 0, i32 0
      store ptr @__vtable_child_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_grandparent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_child(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__parent = getelementptr inbounds nuw %child, ptr %deref, i32 0, i32 0
      call void @__user_init_parent(ptr %__parent)
      ret void
    }

    define void @__user_init_parent(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__grandparent = getelementptr inbounds nuw %parent, ptr %deref, i32 0, i32 0
      call void @__user_init_grandparent(ptr %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__init___vtable_child(ptr @__vtable_child_instance)
      call void @__user_init___vtable_grandparent(ptr @__vtable_grandparent_instance)
      call void @__user_init___vtable_parent(ptr @__vtable_parent_instance)
      call void @__user_init___vtable_child(ptr @__vtable_child_instance)
      ret void
    }
    "#);
}

#[test]
fn properties_are_methods() {
    let property = codegen(
        "
        FUNCTION_BLOCK fb
            VAR
                localPrivateVariable : DINT;
            END_VAR
            PROPERTY foo : DINT
                GET
                    foo := localPrivateVariable;
                END_GET

                SET
                    localPrivateVariable := foo;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
        ",
    );

    let method = codegen(
        "
        FUNCTION_BLOCK fb
          VAR
            localPrivateVariable : DINT;
          END_VAR

          METHOD __get_foo : DINT
            VAR
              foo : DINT;
            END_VAR

            foo := localPrivateVariable;
            __get_foo := foo;
          END_METHOD

          METHOD __set_foo
            VAR_INPUT
              foo : DINT;
            END_VAR

            localPrivateVariable := foo;
          END_METHOD
        END_FUNCTION_BLOCK
        ",
    );

    assert_eq!(property, method);
}

#[test]
fn this_in_method_call_chain() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
            METHOD Step
                THIS^.Increment();
            END_METHOD

            METHOD Increment
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr, ptr, ptr }
    %FB_Test = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test zeroinitializer
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      ret void
    }

    define void @FB_Test__Step(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align 8
      call void @FB_Test__Increment(ptr %deref)
      ret void
    }

    define void @FB_Test__Increment(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      ret void
    }

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %Step = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test__Step, ptr %Step, align 8
      %deref2 = load ptr, ptr %self, align 8
      %Increment = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref2, i32 0, i32 2
      store ptr @FB_Test__Increment, ptr %Increment, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      ret void
    }
    "#);
}

#[test]
fn this_in_method_and_body_in_function_block() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
        VAR
            val : INT := 5;
        END_VAR

        METHOD GetVal : INT
            GetVal := THIS^.val;
        END_METHOD
        val := this^.val;
        this^.val := val;
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr, ptr }
    %FB_Test = type { ptr, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test { ptr null, i16 5 }
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %deref = load ptr, ptr %this, align 8
      %val1 = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_val = load i16, ptr %val1, align 2
      store i16 %load_val, ptr %val, align 2
      %deref2 = load ptr, ptr %this, align 8
      %val3 = getelementptr inbounds nuw %FB_Test, ptr %deref2, i32 0, i32 1
      %load_val4 = load i16, ptr %val, align 2
      store i16 %load_val4, ptr %val3, align 2
      ret void
    }

    define i16 @FB_Test__GetVal(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %FB_Test.GetVal = alloca i16, align 2
      store i16 0, ptr %FB_Test.GetVal, align 2
      %deref = load ptr, ptr %this, align 8
      %val1 = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_val = load i16, ptr %val1, align 2
      store i16 %load_val, ptr %FB_Test.GetVal, align 2
      %FB_Test__GetVal_ret = load i16, ptr %FB_Test.GetVal, align 2
      ret i16 %FB_Test__GetVal_ret
    }

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %GetVal = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test__GetVal, ptr %GetVal, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      ret void
    }
    "#);
}

#[test]
fn pass_this_to_method() {
    // pass `this` pointer of FB1 to a method of another fb called FB2 which calls a method of FB1
    // and changes a value of the passed `this` pointer
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
        VAR
            x : INT := 5;
        END_VAR
        METHOD foo
            VAR
                test : FB_Test2;
                x : INT;
            END_VAR
            test.bar(THIS);
        END_METHOD
        END_FUNCTION_BLOCK
        FUNCTION_BLOCK FB_Test2
            METHOD bar: INT
                VAR_INPUT
                    test : REF_TO FB_Test;
                END_VAR
                bar := test^.x;
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr, ptr }
    %FB_Test = type { ptr, i16 }
    %FB_Test2 = type { ptr }
    %__vtable_FB_Test2 = type { ptr, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test { ptr null, i16 5 }
    @__FB_Test2__init = unnamed_addr constant %FB_Test2 zeroinitializer
    @____vtable_FB_Test2__init = unnamed_addr constant %__vtable_FB_Test2 zeroinitializer
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer
    @__vtable_FB_Test2_instance = global %__vtable_FB_Test2 zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      ret void
    }

    define void @FB_Test__foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %test = alloca %FB_Test2, align 8
      %x1 = alloca i16, align 2
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %test, ptr align 1 @__FB_Test2__init, i64 ptrtoint (ptr getelementptr (%FB_Test2, ptr null, i32 1) to i64), i1 false)
      store i16 0, ptr %x1, align 2
      call void @__init_fb_test2(ptr %test)
      call void @__user_init_FB_Test2(ptr %test)
      %1 = load ptr, ptr %this, align 8
      %call = call i16 @FB_Test2__bar(ptr %test, ptr %1)
      ret void
    }

    define void @FB_Test2(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test2, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @FB_Test2__bar(ptr %0, ptr %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test2, ptr %0, i32 0, i32 0
      %FB_Test2.bar = alloca i16, align 2
      %test = alloca ptr, align 8
      store ptr %1, ptr %test, align 8
      store i16 0, ptr %FB_Test2.bar, align 2
      %deref = load ptr, ptr %test, align 8
      %x = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_x = load i16, ptr %x, align 2
      store i16 %load_x, ptr %FB_Test2.bar, align 2
      %FB_Test2__bar_ret = load i16, ptr %FB_Test2.bar, align 2
      ret i16 %FB_Test2__bar_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test__foo, ptr %foo, align 8
      ret void
    }

    define void @__init___vtable_fb_test2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test2, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %bar = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test2__bar, ptr %bar, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__init_fb_test2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test2, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test2_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init_FB_Test2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__init___vtable_fb_test2(ptr @__vtable_FB_Test2_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test2(ptr @__vtable_FB_Test2_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn this_with_shadowed_variable() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
        VAR
            val : INT := 5;
        END_VAR
        METHOD shadow_val
            VAR
                val : INT := 10;
                local_val: INT;
                shadow_val : INT;
            END_VAR
            local_val := THIS^.val;
            shadow_val := val;
        END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr, ptr }
    %FB_Test = type { ptr, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test { ptr null, i16 5 }
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      ret void
    }

    define void @FB_Test__shadow_val(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %val = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %val1 = alloca i16, align 2
      %local_val = alloca i16, align 2
      %shadow_val = alloca i16, align 2
      store i16 10, ptr %val1, align 2
      store i16 0, ptr %local_val, align 2
      store i16 0, ptr %shadow_val, align 2
      %deref = load ptr, ptr %this, align 8
      %val2 = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_val = load i16, ptr %val2, align 2
      store i16 %load_val, ptr %local_val, align 2
      %load_val3 = load i16, ptr %val1, align 2
      store i16 %load_val3, ptr %shadow_val, align 2
      ret void
    }

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %shadow_val = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test__shadow_val, ptr %shadow_val, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      ret void
    }
    "#);
}

#[test]
fn this_calling_function_and_passing_this() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                x : INT;
            END_VAR
            foo(this);
        END_FUNCTION_BLOCK
        FUNCTION foo : INT
            VAR_INPUT
                pfb: REF_TO FB_TEST;
            END_VAR
                foo := pfb^.x;
        END_FUNCTION
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr }
    %FB_Test = type { ptr, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test zeroinitializer
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %1 = load ptr, ptr %this, align 8
      %call = call i16 @foo(ptr %1)
      ret void
    }

    define i16 @foo(ptr %0) {
    entry:
      %foo = alloca i16, align 2
      %pfb = alloca ptr, align 8
      store ptr %0, ptr %pfb, align 8
      store i16 0, ptr %foo, align 2
      %deref = load ptr, ptr %pfb, align 8
      %x = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_x = load i16, ptr %x, align 2
      store i16 %load_x, ptr %foo, align 2
      %foo_ret = load i16, ptr %foo, align 2
      ret i16 %foo_ret
    }

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      ret void
    }
    "#);
}

#[test]
fn this_in_property_and_calling_method() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                x : INT;
            END_VAR

            METHOD DoubleX : INT
                DoubleX := 2 * THIS^.x;
            END_METHOD

            PROPERTY Value : INT
                GET
                    Value := THIS^.DoubleX();
                END_GET
                SET
                    this^.x := Value;
                END_SET
            END_PROPERTY
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr, ptr, ptr, ptr }
    %FB_Test = type { ptr, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test zeroinitializer
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      ret void
    }

    define i16 @FB_Test__DoubleX(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %FB_Test.DoubleX = alloca i16, align 2
      store i16 0, ptr %FB_Test.DoubleX, align 2
      %deref = load ptr, ptr %this, align 8
      %x1 = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_x = load i16, ptr %x1, align 2
      %1 = sext i16 %load_x to i32
      %tmpVar = mul i32 2, %1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, ptr %FB_Test.DoubleX, align 2
      %FB_Test__DoubleX_ret = load i16, ptr %FB_Test.DoubleX, align 2
      ret i16 %FB_Test__DoubleX_ret
    }

    define i16 @FB_Test____get_Value(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %FB_Test.__get_Value = alloca i16, align 2
      %Value = alloca i16, align 2
      store i16 0, ptr %Value, align 2
      store i16 0, ptr %FB_Test.__get_Value, align 2
      %deref = load ptr, ptr %this, align 8
      %call = call i16 @FB_Test__DoubleX(ptr %deref)
      store i16 %call, ptr %Value, align 2
      %load_Value = load i16, ptr %Value, align 2
      store i16 %load_Value, ptr %FB_Test.__get_Value, align 2
      %FB_Test____get_Value_ret = load i16, ptr %FB_Test.__get_Value, align 2
      ret i16 %FB_Test____get_Value_ret
    }

    define void @FB_Test____set_Value(ptr %0, i16 %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %Value = alloca i16, align 2
      store i16 %1, ptr %Value, align 2
      %deref = load ptr, ptr %this, align 8
      %x1 = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 1
      %load_Value = load i16, ptr %Value, align 2
      store i16 %load_Value, ptr %x1, align 2
      ret void
    }

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %DoubleX = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test__DoubleX, ptr %DoubleX, align 8
      %deref2 = load ptr, ptr %self, align 8
      %__get_Value = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref2, i32 0, i32 2
      store ptr @FB_Test____get_Value, ptr %__get_Value, align 8
      %deref3 = load ptr, ptr %self, align 8
      %__set_Value = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref3, i32 0, i32 3
      store ptr @FB_Test____set_Value, ptr %__set_Value, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      ret void
    }
    "#);
}

#[test]
fn this_with_self_pointer() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB_Test
            VAR
                refToSelf : REF_TO FB_Test;
            END_VAR

            METHOD InitRef
                refToSelf := ADR(THIS^);
                refToSelf := REF(THIS^);
                refToSelf := THIS;
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB_Test = type { ptr, ptr }
    %FB_Test = type { ptr, ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB_Test__init = unnamed_addr constant %__vtable_FB_Test zeroinitializer
    @__FB_Test__init = unnamed_addr constant %FB_Test zeroinitializer
    @__vtable_FB_Test_instance = global %__vtable_FB_Test zeroinitializer

    define void @FB_Test(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %refToSelf = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      ret void
    }

    define void @FB_Test__InitRef(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 0
      %refToSelf = getelementptr inbounds nuw %FB_Test, ptr %0, i32 0, i32 1
      %deref = load ptr, ptr %this, align 8
      store ptr %deref, ptr %refToSelf, align 8
      %deref1 = load ptr, ptr %this, align 8
      store ptr %deref1, ptr %refToSelf, align 8
      %1 = load ptr, ptr %this, align 8
      store ptr %1, ptr %refToSelf, align 8
      ret void
    }

    define void @__init___vtable_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @FB_Test, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %InitRef = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref1, i32 0, i32 1
      store ptr @FB_Test__InitRef, ptr %InitRef, align 8
      ret void
    }

    define void @__init_fb_test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %__vtable_FB_Test, ptr %deref, i32 0, i32 0
      store ptr @__vtable_FB_Test_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_FB_Test(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_test(ptr @__vtable_FB_Test_instance)
      call void @__user_init___vtable_FB_Test(ptr @__vtable_FB_Test_instance)
      ret void
    }
    "#);
}

#[test]
fn this_in_variable_initialization() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK FB
            VAR CONSTANT
                x : INT := 5;
            END_VAR
            VAR
                self : REF_TO FB;
                y : INT := THIS^.x;
            END_VAR
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_FB = type { ptr }
    %FB = type { ptr, i16, ptr, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_FB__init = unnamed_addr constant %__vtable_FB zeroinitializer
    @__FB__init = unnamed_addr constant %FB { ptr null, i16 5, ptr null, i16 5 }
    @__vtable_FB_instance = global %__vtable_FB zeroinitializer

    define void @FB(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 1
      %self = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 2
      %y = getelementptr inbounds nuw %FB, ptr %0, i32 0, i32 3
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

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(ptr @__vtable_FB_instance)
      call void @__user_init___vtable_FB(ptr @__vtable_FB_instance)
      ret void
    }
    "#);
}

#[test]
fn this_in_action_in_functionblock() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK fb
        END_FUNCTION_BLOCK

        ACTION fb.foo
            THIS^();
        END_ACTION
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { ptr }
    %fb = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer
    @__fb__init = unnamed_addr constant %fb zeroinitializer
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer

    define void @fb(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      ret void
    }

    define void @fb__foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align 8
      call void @fb(ptr %deref)
      ret void
    }

    define void @__init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @fb, ptr %__body, align 8
      ret void
    }

    define void @__init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @__vtable_fb_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(ptr @__vtable_fb_instance)
      call void @__user_init___vtable_fb(ptr @__vtable_fb_instance)
      ret void
    }
    "#);
}

#[test]
fn this_calling_functionblock_body_from_method() {
    let code = codegen(
        r#"
        FUNCTION_BLOCK fb
            METHOD foo : INT
                THIS^();
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_fb = type { ptr, ptr }
    %fb = type { ptr }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_fb__init = unnamed_addr constant %__vtable_fb zeroinitializer
    @__fb__init = unnamed_addr constant %fb zeroinitializer
    @__vtable_fb_instance = global %__vtable_fb zeroinitializer

    define void @fb(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @fb__foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %0, i32 0, i32 0
      %fb.foo = alloca i16, align 2
      store i16 0, ptr %fb.foo, align 2
      %deref = load ptr, ptr %this, align 8
      call void @fb(ptr %deref)
      %fb__foo_ret = load i16, ptr %fb.foo, align 2
      ret i16 %fb__foo_ret
    }

    define void @__init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_fb, ptr %deref, i32 0, i32 0
      store ptr @fb, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %foo = getelementptr inbounds nuw %__vtable_fb, ptr %deref1, i32 0, i32 1
      store ptr @fb__foo, ptr %foo, align 8
      ret void
    }

    define void @__init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__vtable = getelementptr inbounds nuw %fb, ptr %deref, i32 0, i32 0
      store ptr @__vtable_fb_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__user_init___vtable_fb(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb(ptr @__vtable_fb_instance)
      call void @__user_init___vtable_fb(ptr @__vtable_fb_instance)
      ret void
    }
    "#);
}

#[test]
fn fb_extension_with_output() {
    let code = codegen(
        "FUNCTION_BLOCK foo
            METHOD met1 : INT
            VAR_INPUT
            mandatoryInput : INT;
            optionalInput : INT := 5;
            END_VAR
            VAR_OUTPUT
            outputValue : INT;
            END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK foo2 EXTENDS foo
            met1(
                mandatoryInput := 0,
                optionalInput := 0,
                outputValue =>
            );
        END_FUNCTION_BLOCK",
    );
    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }
    %__vtable_foo2 = type { ptr, ptr }
    %foo2 = type { %foo }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_foo2__init = unnamed_addr constant %__vtable_foo2 zeroinitializer
    @__foo2__init = unnamed_addr constant %foo2 zeroinitializer
    @__vtable_foo2_instance = global %__vtable_foo2 zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @foo__met1(ptr %0, i16 %1, i16 %2, ptr %3) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %foo.met1 = alloca i16, align 2
      %mandatoryInput = alloca i16, align 2
      store i16 %1, ptr %mandatoryInput, align 2
      %optionalInput = alloca i16, align 2
      store i16 %2, ptr %optionalInput, align 2
      %outputValue = alloca ptr, align 8
      store ptr %3, ptr %outputValue, align 8
      store i16 0, ptr %foo.met1, align 2
      %foo__met1_ret = load i16, ptr %foo.met1, align 2
      ret i16 %foo__met1_ret
    }

    define void @foo2(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %foo2, ptr %0, i32 0, i32 0
      %deref = load ptr, ptr %this, align 8
      %__foo1 = getelementptr inbounds nuw %foo2, ptr %deref, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo1, i32 0, i32 0
      %deref2 = load ptr, ptr %__vtable, align 8
      %met1 = getelementptr inbounds nuw %__vtable_foo2, ptr %deref2, i32 0, i32 1
      %1 = load ptr, ptr %met1, align 8
      %deref3 = load ptr, ptr %this, align 8
      %2 = alloca i16, align 2
      %fnptr_call = call i16 %1(ptr %deref3, i16 0, i16 0, ptr %2)
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
      %met1 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__met1, ptr %met1, align 8
      ret void
    }

    define void @__init___vtable_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo2, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %met1 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__met1, ptr %met1, align 8
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

    define void @__init_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %foo2, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %foo2, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
      store ptr @__vtable_foo2_instance, ptr %__vtable, align 8
      ret void
    }

    define void @__user_init_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %foo2, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
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

    define void @__user_init___vtable_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_foo2(ptr @__vtable_foo2_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo2(ptr @__vtable_foo2_instance)
      ret void
    }
    "#);
}

#[test]
fn function_with_output_used_in_main_by_extension() {
    let code = codegen(
        "
    FUNCTION_BLOCK foo
    METHOD met1 : INT
        VAR_INPUT
        mandatoryInput : INT;
        optionalInput : INT := 5;
        END_VAR
        VAR_OUTPUT
        outputValue : INT;
        END_VAR
        outputValue := mandatoryInput + optionalInput;
    END_METHOD
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK foo2 EXTENDS foo
    VAR
        x : INT;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION main : DINT
    VAR
        foo_inst: foo;
        foo2_inst : foo2;
        out : INT;
    END_VAR
    foo_inst.met1(mandatoryInput:= 1, outputValue => out);
    foo2_inst.met1(mandatoryInput:= 2, outputValue => out);
    END_FUNCTION

    ",
    );

    filtered_assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %__vtable_foo = type { ptr, ptr }
    %foo = type { ptr }
    %__vtable_foo2 = type { ptr, ptr }
    %foo2 = type { %foo, i16 }

    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 0, ptr @__init___Test, ptr null }]
    @____vtable_foo__init = unnamed_addr constant %__vtable_foo zeroinitializer
    @__foo__init = unnamed_addr constant %foo zeroinitializer
    @__vtable_foo_instance = global %__vtable_foo zeroinitializer
    @____vtable_foo2__init = unnamed_addr constant %__vtable_foo2 zeroinitializer
    @__foo2__init = unnamed_addr constant %foo2 zeroinitializer
    @__vtable_foo2_instance = global %__vtable_foo2 zeroinitializer

    define void @foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      ret void
    }

    define i16 @foo__met1(ptr %0, i16 %1, i16 %2, ptr %3) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__vtable = getelementptr inbounds nuw %foo, ptr %0, i32 0, i32 0
      %foo.met1 = alloca i16, align 2
      %mandatoryInput = alloca i16, align 2
      store i16 %1, ptr %mandatoryInput, align 2
      %optionalInput = alloca i16, align 2
      store i16 %2, ptr %optionalInput, align 2
      %outputValue = alloca ptr, align 8
      store ptr %3, ptr %outputValue, align 8
      store i16 0, ptr %foo.met1, align 2
      %deref = load ptr, ptr %outputValue, align 8
      %load_mandatoryInput = load i16, ptr %mandatoryInput, align 2
      %4 = sext i16 %load_mandatoryInput to i32
      %load_optionalInput = load i16, ptr %optionalInput, align 2
      %5 = sext i16 %load_optionalInput to i32
      %tmpVar = add i32 %4, %5
      %6 = trunc i32 %tmpVar to i16
      store i16 %6, ptr %deref, align 2
      %foo__met1_ret = load i16, ptr %foo.met1, align 2
      ret i16 %foo__met1_ret
    }

    define void @foo2(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %__foo = getelementptr inbounds nuw %foo2, ptr %0, i32 0, i32 0
      %x = getelementptr inbounds nuw %foo2, ptr %0, i32 0, i32 1
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %foo_inst = alloca %foo, align 8
      %foo2_inst = alloca %foo2, align 8
      %out = alloca i16, align 2
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %foo_inst, ptr align 1 @__foo__init, i64 ptrtoint (ptr getelementptr (%foo, ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %foo2_inst, ptr align 1 @__foo2__init, i64 ptrtoint (ptr getelementptr (%foo2, ptr null, i32 1) to i64), i1 false)
      store i16 0, ptr %out, align 2
      store i32 0, ptr %main, align 4
      call void @__init_foo(ptr %foo_inst)
      call void @__init_foo2(ptr %foo2_inst)
      call void @__user_init_foo(ptr %foo_inst)
      call void @__user_init_foo2(ptr %foo2_inst)
      %call = call i16 @foo__met1(ptr %foo_inst, i16 1, i16 5, ptr %out)
      %__foo = getelementptr inbounds nuw %foo2, ptr %foo2_inst, i32 0, i32 0
      %call1 = call i16 @foo__met1(ptr %__foo, i16 2, i16 5, ptr %out)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
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
      %met1 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__met1, ptr %met1, align 8
      ret void
    }

    define void @__init___vtable_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__body = getelementptr inbounds nuw %__vtable_foo, ptr %deref, i32 0, i32 0
      store ptr @foo2, ptr %__body, align 8
      %deref1 = load ptr, ptr %self, align 8
      %met1 = getelementptr inbounds nuw %__vtable_foo, ptr %deref1, i32 0, i32 1
      store ptr @foo__met1, ptr %met1, align 8
      ret void
    }

    define void @__init_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %foo2, ptr %deref, i32 0, i32 0
      call void @__init_foo(ptr %__foo)
      %deref1 = load ptr, ptr %self, align 8
      %__foo2 = getelementptr inbounds nuw %foo2, ptr %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds nuw %foo, ptr %__foo2, i32 0, i32 0
      store ptr @__vtable_foo2_instance, ptr %__vtable, align 8
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

    define void @__user_init_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      %deref = load ptr, ptr %self, align 8
      %__foo = getelementptr inbounds nuw %foo2, ptr %deref, i32 0, i32 0
      call void @__user_init_foo(ptr %__foo)
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

    define void @__user_init___vtable_foo2(ptr %0) {
    entry:
      %self = alloca ptr, align 8
      store ptr %0, ptr %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__init___vtable_foo2(ptr @__vtable_foo2_instance)
      call void @__user_init___vtable_foo(ptr @__vtable_foo_instance)
      call void @__user_init___vtable_foo2(ptr @__vtable_foo2_instance)
      ret void
    }

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
