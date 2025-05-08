use test_utils::codegen;

mod debug_tests;
mod super_tests;
mod vtable_tests;

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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i32*, i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }
    %__vtable_foo_type = type { i32* }
    %__vtable_bar_type = type { %__vtable_foo_type, i32* }

    @__foo__init = constant %foo zeroinitializer
    @__bar__init = constant %bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer
    @__vtable_bar = global %__vtable_bar_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 3
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
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
      %deref = load %__vtable_bar_type*, %__vtable_bar_type** %self, align 8
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 0
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
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

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      %deref1 = load %bar*, %bar** %self, align 8
      %__foo2 = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %foo, %foo* %__foo2, i32 0, i32 0
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %__foo)
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
    "###);
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

    insta::assert_snapshot!(res, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb2 = type { %fb }
    %fb = type { i32*, i16, i16 }
    %foo = type { i32*, %fb2 }
    %__vtable_fb_type = type { i32* }
    %__vtable_fb2_type = type { %__vtable_fb_type, i32* }
    %__vtable_foo_type = type { i32* }

    @__fb2__init = constant %fb2 zeroinitializer
    @__fb__init = constant %fb zeroinitializer
    @__foo__init = constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_fb_type__init = constant %__vtable_fb_type zeroinitializer
    @__vtable_fb = global %__vtable_fb_type zeroinitializer
    @____vtable_fb2_type__init = constant %__vtable_fb2_type zeroinitializer
    @__vtable_fb2 = global %__vtable_fb2_type zeroinitializer
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer

    define void @fb(%fb* %0) {
    entry:
      %__vtable = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 2
      ret void
    }

    define void @fb2(%fb2* %0) {
    entry:
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void
    }

    define void @foo(%foo* %0) {
    entry:
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 1
      store i16 1, i16* %x, align 2
      ret void
    }

    define void @__init___vtable_fb_type(%__vtable_fb_type* %0) {
    entry:
      %self = alloca %__vtable_fb_type*, align 8
      store %__vtable_fb_type* %0, %__vtable_fb_type** %self, align 8
      ret void
    }

    define void @__init___vtable_fb2_type(%__vtable_fb2_type* %0) {
    entry:
      %self = alloca %__vtable_fb2_type*, align 8
      store %__vtable_fb2_type* %0, %__vtable_fb2_type** %self, align 8
      %deref = load %__vtable_fb2_type*, %__vtable_fb2_type** %self, align 8
      %__vtable_fb_type = getelementptr inbounds %__vtable_fb2_type, %__vtable_fb2_type* %deref, i32 0, i32 0
      call void @__init___vtable_fb_type(%__vtable_fb_type* %__vtable_fb_type)
      ret void
    }

    define void @__init___vtable_foo_type(%__vtable_foo_type* %0) {
    entry:
      %self = alloca %__vtable_foo_type*, align 8
      store %__vtable_foo_type* %0, %__vtable_foo_type** %self, align 8
      ret void
    }

    define void @__init_fb2(%fb2* %0) {
    entry:
      %self = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %self, align 8
      %deref = load %fb2*, %fb2** %self, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %deref, i32 0, i32 0
      call void @__init_fb(%fb* %__fb)
      %deref1 = load %fb2*, %fb2** %self, align 8
      %__fb2 = getelementptr inbounds %fb2, %fb2* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %fb, %fb* %__fb2, i32 0, i32 0
      store i32* bitcast (%__vtable_fb2_type* @__vtable_fb2 to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      %deref = load %fb*, %fb** %self, align 8
      %__vtable = getelementptr inbounds %fb, %fb* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_fb_type* @__vtable_fb to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      call void @__init_fb2(%fb2* %myFb)
      %deref1 = load %foo*, %foo** %self, align 8
      %__vtable = getelementptr inbounds %foo, %foo* %deref1, i32 0, i32 0
      store i32* bitcast (%__vtable_foo_type* @__vtable_foo to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__user_init_fb2(%fb2* %0) {
    entry:
      %self = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %self, align 8
      %deref = load %fb2*, %fb2** %self, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %deref, i32 0, i32 0
      call void @__user_init_fb(%fb* %__fb)
      ret void
    }

    define void @__user_init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 1
      call void @__user_init_fb2(%fb2* %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_fb_type(%__vtable_fb_type* @__vtable_fb)
      call void @__init___vtable_fb2_type(%__vtable_fb2_type* @__vtable_fb2)
      call void @__init___vtable_foo_type(%__vtable_foo_type* @__vtable_foo)
      ret void
    }
    "###);
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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %bar = type { %foo }
    %foo = type { i32*, [81 x i8] }
    %__vtable_foo_type = type { i32*, i32* }
    %__vtable_bar_type = type { %__vtable_foo_type, i32* }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = constant %bar zeroinitializer
    @__foo__init = constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_foo_type__init = constant %__vtable_foo_type zeroinitializer
    @__vtable_foo = global %__vtable_foo_type zeroinitializer
    @____vtable_bar_type__init = constant %__vtable_bar_type zeroinitializer
    @__vtable_bar = global %__vtable_bar_type zeroinitializer

    define void @foo(%foo* %0) {
    entry:
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      ret void
    }

    define void @foo_baz(%foo* %0) {
    entry:
      %__vtable = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 1
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_1, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    define void @main() {
    entry:
      %s = alloca [81 x i8], align 1
      %fb = alloca %bar, align 8
      %0 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %1 = bitcast %bar* %fb to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%bar* @__bar__init to i8*), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
      call void @__init_bar(%bar* %fb)
      call void @__user_init_bar(%bar* %fb)
      %__foo = getelementptr inbounds %bar, %bar* %fb, i32 0, i32 0
      call void @foo_baz(%foo* %__foo)
      call void @bar(%bar* %fb)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

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
      %deref = load %__vtable_bar_type*, %__vtable_bar_type** %self, align 8
      %__vtable_foo_type = getelementptr inbounds %__vtable_bar_type, %__vtable_bar_type* %deref, i32 0, i32 0
      call void @__init___vtable_foo_type(%__vtable_foo_type* %__vtable_foo_type)
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      %deref1 = load %bar*, %bar** %self, align 8
      %__foo2 = getelementptr inbounds %bar, %bar* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %foo, %foo* %__foo2, i32 0, i32 0
      store i32* bitcast (%__vtable_bar_type* @__vtable_bar to i32*), i32** %__vtable, align 8
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

    define void @__user_init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__user_init_foo(%foo* %__foo)
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
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    "###);
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
    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { i32*, [6 x i16], i16 }
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { %__vtable_grandparent_type, i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32* }

    @__child__init = constant %child zeroinitializer
    @__parent__init = constant %parent zeroinitializer
    @__grandparent__init = constant %grandparent zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer
    @__vtable_parent = global %__vtable_parent_type zeroinitializer
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer
    @__vtable_child = global %__vtable_child_type zeroinitializer

    define void @grandparent(%grandparent* %0) {
    entry:
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void
    }

    define void @parent(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      ret void
    }

    define void @main() {
    entry:
      %arr = alloca [11 x %child], align 8
      %0 = bitcast [11 x %child]* %arr to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([11 x %child]* getelementptr ([11 x %child], [11 x %child]* null, i32 1) to i64), i1 false)
      %tmpVar = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0
      %__parent = getelementptr inbounds %child, %child* %tmpVar, i32 0, i32 0
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 2
      store i16 10, i16* %a, align 2
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 1
      %tmpVar4 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 0
      store i16 20, i16* %tmpVar4, align 2
      %tmpVar5 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1
      %__parent6 = getelementptr inbounds %child, %child* %tmpVar5, i32 0, i32 0
      %b = getelementptr inbounds %parent, %parent* %__parent6, i32 0, i32 2
      store i16 30, i16* %b, align 2
      %tmpVar7 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 1
      %__parent8 = getelementptr inbounds %child, %child* %tmpVar7, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %__parent8, i32 0, i32 1
      %tmpVar9 = getelementptr inbounds [11 x i16], [11 x i16]* %x, i32 0, i32 1
      store i16 40, i16* %tmpVar9, align 2
      %tmpVar10 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 2
      %z = getelementptr inbounds %child, %child* %tmpVar10, i32 0, i32 1
      %tmpVar11 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 2
      store i16 50, i16* %tmpVar11, align 2
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    define void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %0) {
    entry:
      %self = alloca %__vtable_grandparent_type*, align 8
      store %__vtable_grandparent_type* %0, %__vtable_grandparent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      %deref = load %__vtable_parent_type*, %__vtable_parent_type** %self, align 8
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      %deref1 = load %parent*, %parent** %self, align 8
      %__grandparent2 = getelementptr inbounds %parent, %parent* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent2, i32 0, i32 0
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent_type* @__vtable_grandparent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    "###);
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

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { i32*, [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }
    %__vtable_grandparent_type = type { i32* }
    %__vtable_parent_type = type { %__vtable_grandparent_type, i32* }
    %__vtable_child_type = type { %__vtable_parent_type, i32* }

    @__parent__init = constant %parent zeroinitializer
    @__grandparent__init = constant %grandparent zeroinitializer
    @__child__init = constant %child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]
    @____vtable_grandparent_type__init = constant %__vtable_grandparent_type zeroinitializer
    @__vtable_grandparent = global %__vtable_grandparent_type zeroinitializer
    @____vtable_parent_type__init = constant %__vtable_parent_type zeroinitializer
    @__vtable_parent = global %__vtable_parent_type zeroinitializer
    @____vtable_child_type__init = constant %__vtable_child_type zeroinitializer
    @__vtable_child = global %__vtable_child_type zeroinitializer

    define void @grandparent(%grandparent* %0) {
    entry:
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 2
      ret void
    }

    define void @parent(%parent* %0) {
    entry:
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2
      %load_b = load i16, i16* %b, align 2
      %1 = sext i16 %load_b to i32
      %b1 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 2
      %load_b2 = load i16, i16* %b1, align 2
      %2 = sext i16 %load_b2 to i32
      %tmpVar = mul i32 %2, 2
      %tmpVar3 = mul i32 1, %tmpVar
      %tmpVar4 = add i32 %tmpVar3, 0
      %tmpVar5 = getelementptr inbounds [11 x i16], [11 x i16]* %z, i32 0, i32 %tmpVar4
      %load_tmpVar = load i16, i16* %tmpVar5, align 2
      %3 = sext i16 %load_tmpVar to i32
      %tmpVar6 = add i32 %1, %3
      %__grandparent7 = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 2
      %load_a = load i16, i16* %a, align 2
      %4 = sext i16 %load_a to i32
      %tmpVar8 = sub i32 %tmpVar6, %4
      %tmpVar9 = mul i32 1, %tmpVar8
      %tmpVar10 = add i32 %tmpVar9, 0
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10
      store i16 20, i16* %tmpVar11, align 2
      ret void
    }

    define void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %0) {
    entry:
      %self = alloca %__vtable_grandparent_type*, align 8
      store %__vtable_grandparent_type* %0, %__vtable_grandparent_type** %self, align 8
      ret void
    }

    define void @__init___vtable_parent_type(%__vtable_parent_type* %0) {
    entry:
      %self = alloca %__vtable_parent_type*, align 8
      store %__vtable_parent_type* %0, %__vtable_parent_type** %self, align 8
      %deref = load %__vtable_parent_type*, %__vtable_parent_type** %self, align 8
      %__vtable_grandparent_type = getelementptr inbounds %__vtable_parent_type, %__vtable_parent_type* %deref, i32 0, i32 0
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* %__vtable_grandparent_type)
      ret void
    }

    define void @__init___vtable_child_type(%__vtable_child_type* %0) {
    entry:
      %self = alloca %__vtable_child_type*, align 8
      store %__vtable_child_type* %0, %__vtable_child_type** %self, align 8
      %deref = load %__vtable_child_type*, %__vtable_child_type** %self, align 8
      %__vtable_parent_type = getelementptr inbounds %__vtable_child_type, %__vtable_child_type* %deref, i32 0, i32 0
      call void @__init___vtable_parent_type(%__vtable_parent_type* %__vtable_parent_type)
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      %deref1 = load %parent*, %parent** %self, align 8
      %__grandparent2 = getelementptr inbounds %parent, %parent* %deref1, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent2, i32 0, i32 0
      store i32* bitcast (%__vtable_parent_type* @__vtable_parent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      %deref = load %grandparent*, %grandparent** %self, align 8
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %deref, i32 0, i32 0
      store i32* bitcast (%__vtable_grandparent_type* @__vtable_grandparent to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      %deref1 = load %child*, %child** %self, align 8
      %__parent2 = getelementptr inbounds %child, %child* %deref1, i32 0, i32 0
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %__vtable = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0
      store i32* bitcast (%__vtable_child_type* @__vtable_child to i32*), i32** %__vtable, align 8
      ret void
    }

    define void @__user_init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__user_init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__user_init_parent(%parent* %__parent)
      ret void
    }

    define void @__user_init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__user_init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init___Test() {
    entry:
      call void @__init___vtable_grandparent_type(%__vtable_grandparent_type* @__vtable_grandparent)
      call void @__init___vtable_parent_type(%__vtable_parent_type* @__vtable_parent)
      call void @__init___vtable_child_type(%__vtable_child_type* @__vtable_child)
      ret void
    }
    "###);
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
fn global_vtable_variables_are_generated() {
    let result = codegen(
        r"
        FUNCTION_BLOCK fb
          METHOD foo
          END_METHOD
        END_FUNCTION_BLOCK
      ",
    );

    assert!(result.contains("@__vtable_fb = global %__vtable_fb_type zeroinitializer"))
}
