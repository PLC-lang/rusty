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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %foo = type { i16, [81 x i8], [11 x [81 x i8]] }
    %bar = type { %foo }

    @__foo__init = constant %foo zeroinitializer
    @__bar__init = constant %bar zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %a = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %b = getelementptr inbounds %foo, %foo* %0, i32 0, i32 1
      %c = getelementptr inbounds %foo, %foo* %0, i32 0, i32 2
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      ret void
    }

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
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

    insta::assert_snapshot!(res, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb2 = type { %fb }
    %fb = type { i16, i16 }
    %foo = type { %fb2 }

    @__fb2__init = constant %fb2 zeroinitializer
    @__fb__init = constant %fb zeroinitializer
    @__foo__init = constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @fb(%fb* %0) {
    entry:
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %x = getelementptr inbounds %fb, %fb* %0, i32 0, i32 0
      %y = getelementptr inbounds %fb, %fb* %0, i32 0, i32 1
      ret void
    }

    define void @fb2(%fb2* %0) {
    entry:
      %this = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %this, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %0, i32 0, i32 0
      ret void
    }

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %myFb = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %__fb = getelementptr inbounds %fb2, %fb2* %myFb, i32 0, i32 0
      %x = getelementptr inbounds %fb, %fb* %__fb, i32 0, i32 0
      store i16 1, i16* %x, align 2
      ret void
    }

    define void @__init_fb2(%fb2* %0) {
    entry:
      %self = alloca %fb2*, align 8
      store %fb2* %0, %fb2** %self, align 8
      %deref = load %fb2*, %fb2** %self, align 8
      %__fb = getelementptr inbounds %fb2, %fb2* %deref, i32 0, i32 0
      call void @__init_fb(%fb* %__fb)
      ret void
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
      %deref = load %foo*, %foo** %self, align 8
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      call void @__init_fb2(%fb2* %myFb)
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
      %myFb = getelementptr inbounds %foo, %foo* %deref, i32 0, i32 0
      call void @__user_init_fb2(%fb2* %myFb)
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %bar = type { %foo }
    %foo = type { [81 x i8] }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"
    @__bar__init = constant %bar zeroinitializer
    @__foo__init = constant %foo zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @foo(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      ret void
    }

    define void @foo_baz(%foo* %0) {
    entry:
      %this = alloca %foo*, align 8
      store %foo* %0, %foo** %this, align 8
      %s = getelementptr inbounds %foo, %foo* %0, i32 0, i32 0
      %1 = bitcast [81 x i8]* %s to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %1, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    define void @bar(%bar* %0) {
    entry:
      %this = alloca %bar*, align 8
      store %bar* %0, %bar** %this, align 8
      %__foo = getelementptr inbounds %bar, %bar* %0, i32 0, i32 0
      %s = getelementptr inbounds %foo, %foo* %__foo, i32 0, i32 0
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
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%bar, %bar* @__bar__init, i32 0, i32 0, i32 0, i32 0), i64 ptrtoint (%bar* getelementptr (%bar, %bar* null, i32 1) to i64), i1 false)
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

    define void @__init_bar(%bar* %0) {
    entry:
      %self = alloca %bar*, align 8
      store %bar* %0, %bar** %self, align 8
      %deref = load %bar*, %bar** %self, align 8
      %__foo = getelementptr inbounds %bar, %bar* %deref, i32 0, i32 0
      call void @__init_foo(%foo* %__foo)
      ret void
    }

    define void @__init_foo(%foo* %0) {
    entry:
      %self = alloca %foo*, align 8
      store %foo* %0, %foo** %self, align 8
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
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
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
    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %child = type { %parent, [11 x i16] }
    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }

    @__child__init = constant %child zeroinitializer
    @__parent__init = constant %parent zeroinitializer
    @__grandparent__init = constant %grandparent zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) {
    entry:
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void
    }

    define void @parent(%parent* %0) {
    entry:
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
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
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 1
      store i16 10, i16* %a, align 2
      %tmpVar1 = getelementptr inbounds [11 x %child], [11 x %child]* %arr, i32 0, i32 0
      %__parent2 = getelementptr inbounds %child, %child* %tmpVar1, i32 0, i32 0
      %__grandparent3 = getelementptr inbounds %parent, %parent* %__parent2, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent3, i32 0, i32 0
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

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
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
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
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

    insta::assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %parent = type { %grandparent, [11 x i16], i16 }
    %grandparent = type { [6 x i16], i16 }
    %child = type { %parent, [11 x i16] }

    @__parent__init = constant %parent zeroinitializer
    @__grandparent__init = constant %grandparent zeroinitializer
    @__child__init = constant %child zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @grandparent(%grandparent* %0) {
    entry:
      %this = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %this, align 8
      %y = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 0
      %a = getelementptr inbounds %grandparent, %grandparent* %0, i32 0, i32 1
      ret void
    }

    define void @parent(%parent* %0) {
    entry:
      %this = alloca %parent*, align 8
      store %parent* %0, %parent** %this, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %0, i32 0, i32 0
      %x = getelementptr inbounds %parent, %parent* %0, i32 0, i32 1
      %b = getelementptr inbounds %parent, %parent* %0, i32 0, i32 2
      ret void
    }

    define void @child(%child* %0) {
    entry:
      %this = alloca %child*, align 8
      store %child* %0, %child** %this, align 8
      %__parent = getelementptr inbounds %child, %child* %0, i32 0, i32 0
      %z = getelementptr inbounds %child, %child* %0, i32 0, i32 1
      %__grandparent = getelementptr inbounds %parent, %parent* %__parent, i32 0, i32 0
      %y = getelementptr inbounds %grandparent, %grandparent* %__grandparent, i32 0, i32 0
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
      %a = getelementptr inbounds %grandparent, %grandparent* %__grandparent7, i32 0, i32 1
      %load_a = load i16, i16* %a, align 2
      %4 = sext i16 %load_a to i32
      %tmpVar8 = sub i32 %tmpVar6, %4
      %tmpVar9 = mul i32 1, %tmpVar8
      %tmpVar10 = add i32 %tmpVar9, 0
      %tmpVar11 = getelementptr inbounds [6 x i16], [6 x i16]* %y, i32 0, i32 %tmpVar10
      store i16 20, i16* %tmpVar11, align 2
      ret void
    }

    define void @__init_parent(%parent* %0) {
    entry:
      %self = alloca %parent*, align 8
      store %parent* %0, %parent** %self, align 8
      %deref = load %parent*, %parent** %self, align 8
      %__grandparent = getelementptr inbounds %parent, %parent* %deref, i32 0, i32 0
      call void @__init_grandparent(%grandparent* %__grandparent)
      ret void
    }

    define void @__init_grandparent(%grandparent* %0) {
    entry:
      %self = alloca %grandparent*, align 8
      store %grandparent* %0, %grandparent** %self, align 8
      ret void
    }

    define void @__init_child(%child* %0) {
    entry:
      %self = alloca %child*, align 8
      store %child* %0, %child** %self, align 8
      %deref = load %child*, %child** %self, align 8
      %__parent = getelementptr inbounds %child, %child* %deref, i32 0, i32 0
      call void @__init_parent(%parent* %__parent)
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type {}

    @__FB_Test__init = constant %FB_Test zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      ret void
    }

    define void @FB_Test_Step(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      call void @FB_Test_Increment(%FB_Test* %deref)
      ret void
    }

    define void @FB_Test_Increment(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      ret void
    }

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type { i16 }

    @__FB_Test__init = constant %FB_Test { i16 5 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %val = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      %val1 = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_val = load i16, i16* %val1, align 2
      store i16 %load_val, i16* %val, align 2
      %deref2 = load %FB_Test*, %FB_Test** %this, align 8
      %val3 = getelementptr inbounds %FB_Test, %FB_Test* %deref2, i32 0, i32 0
      %load_val4 = load i16, i16* %val, align 2
      store i16 %load_val4, i16* %val3, align 2
      ret void
    }

    define i16 @FB_Test_GetVal(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %val = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %FB_Test.GetVal = alloca i16, align 2
      store i16 0, i16* %FB_Test.GetVal, align 2
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      %val1 = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_val = load i16, i16* %val1, align 2
      store i16 %load_val, i16* %FB_Test.GetVal, align 2
      %FB_Test_GetVal_ret = load i16, i16* %FB_Test.GetVal, align 2
      ret i16 %FB_Test_GetVal_ret
    }

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type { i16 }
    %FB_Test2 = type {}

    @__FB_Test__init = constant %FB_Test { i16 5 }
    @__FB_Test2__init = constant %FB_Test2 zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      ret void
    }

    define void @FB_Test_foo(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %test = alloca %FB_Test2, align 8
      %x1 = alloca i16, align 2
      %1 = bitcast %FB_Test2* %test to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%FB_Test2* @__FB_Test2__init to i8*), i64 ptrtoint (%FB_Test2* getelementptr (%FB_Test2, %FB_Test2* null, i32 1) to i64), i1 false)
      store i16 0, i16* %x1, align 2
      call void @__init_fb_test2(%FB_Test2* %test)
      call void @__user_init_FB_Test2(%FB_Test2* %test)
      %2 = load %FB_Test*, %FB_Test** %this, align 8
      %call = call i16 @FB_Test2_bar(%FB_Test2* %test, %FB_Test* %2)
      ret void
    }

    define void @FB_Test2(%FB_Test2* %0) {
    entry:
      %this = alloca %FB_Test2*, align 8
      store %FB_Test2* %0, %FB_Test2** %this, align 8
      ret void
    }

    define i16 @FB_Test2_bar(%FB_Test2* %0, %FB_Test* %1) {
    entry:
      %this = alloca %FB_Test2*, align 8
      store %FB_Test2* %0, %FB_Test2** %this, align 8
      %FB_Test2.bar = alloca i16, align 2
      %test = alloca %FB_Test*, align 8
      store %FB_Test* %1, %FB_Test** %test, align 8
      store i16 0, i16* %FB_Test2.bar, align 2
      %deref = load %FB_Test*, %FB_Test** %test, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_x = load i16, i16* %x, align 2
      store i16 %load_x, i16* %FB_Test2.bar, align 2
      %FB_Test2_bar_ret = load i16, i16* %FB_Test2.bar, align 2
      ret i16 %FB_Test2_bar_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init_fb_test2(%FB_Test2* %0) {
    entry:
      %self = alloca %FB_Test2*, align 8
      store %FB_Test2* %0, %FB_Test2** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test2(%FB_Test2* %0) {
    entry:
      %self = alloca %FB_Test2*, align 8
      store %FB_Test2* %0, %FB_Test2** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }

    attributes #0 = { argmemonly nofree nounwind willreturn }
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type { i16 }

    @__FB_Test__init = constant %FB_Test { i16 5 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %val = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      ret void
    }

    define void @FB_Test_shadow_val(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %val = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %val1 = alloca i16, align 2
      %local_val = alloca i16, align 2
      %shadow_val = alloca i16, align 2
      store i16 10, i16* %val1, align 2
      store i16 0, i16* %local_val, align 2
      store i16 0, i16* %shadow_val, align 2
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      %val2 = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_val = load i16, i16* %val2, align 2
      store i16 %load_val, i16* %local_val, align 2
      %load_val3 = load i16, i16* %val1, align 2
      store i16 %load_val3, i16* %shadow_val, align 2
      ret void
    }

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type { i16 }

    @__FB_Test__init = constant %FB_Test zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %1 = load %FB_Test*, %FB_Test** %this, align 8
      %call = call i16 @foo(%FB_Test* %1)
      ret void
    }

    define i16 @foo(%FB_Test* %0) {
    entry:
      %foo = alloca i16, align 2
      %pfb = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %pfb, align 8
      store i16 0, i16* %foo, align 2
      %deref = load %FB_Test*, %FB_Test** %pfb, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_x = load i16, i16* %x, align 2
      store i16 %load_x, i16* %foo, align 2
      %foo_ret = load i16, i16* %foo, align 2
      ret i16 %foo_ret
    }

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type { i16 }

    @__FB_Test__init = constant %FB_Test zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      ret void
    }

    define i16 @FB_Test_DoubleX(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %FB_Test.DoubleX = alloca i16, align 2
      store i16 0, i16* %FB_Test.DoubleX, align 2
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      %x1 = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_x = load i16, i16* %x1, align 2
      %1 = sext i16 %load_x to i32
      %tmpVar = mul i32 2, %1
      %2 = trunc i32 %tmpVar to i16
      store i16 %2, i16* %FB_Test.DoubleX, align 2
      %FB_Test_DoubleX_ret = load i16, i16* %FB_Test.DoubleX, align 2
      ret i16 %FB_Test_DoubleX_ret
    }

    define i16 @FB_Test___get_Value(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %FB_Test.__get_Value = alloca i16, align 2
      %Value = alloca i16, align 2
      store i16 0, i16* %Value, align 2
      store i16 0, i16* %FB_Test.__get_Value, align 2
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      %call = call i16 @FB_Test_DoubleX(%FB_Test* %deref)
      store i16 %call, i16* %Value, align 2
      %load_Value = load i16, i16* %Value, align 2
      store i16 %load_Value, i16* %FB_Test.__get_Value, align 2
      %FB_Test___get_Value_ret = load i16, i16* %FB_Test.__get_Value, align 2
      ret i16 %FB_Test___get_Value_ret
    }

    define void @FB_Test___set_Value(%FB_Test* %0, i16 %1) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %x = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %Value = alloca i16, align 2
      store i16 %1, i16* %Value, align 2
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      %x1 = getelementptr inbounds %FB_Test, %FB_Test* %deref, i32 0, i32 0
      %load_Value = load i16, i16* %Value, align 2
      store i16 %load_Value, i16* %x1, align 2
      ret void
    }

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB_Test = type { %FB_Test* }

    @__FB_Test__init = constant %FB_Test zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB_Test(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %refToSelf = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      ret void
    }

    define void @FB_Test_InitRef(%FB_Test* %0) {
    entry:
      %this = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %this, align 8
      %refToSelf = getelementptr inbounds %FB_Test, %FB_Test* %0, i32 0, i32 0
      %deref = load %FB_Test*, %FB_Test** %this, align 8
      store %FB_Test* %deref, %FB_Test** %refToSelf, align 8
      %deref1 = load %FB_Test*, %FB_Test** %this, align 8
      store %FB_Test* %deref1, %FB_Test** %refToSelf, align 8
      %1 = load %FB_Test*, %FB_Test** %this, align 8
      store %FB_Test* %1, %FB_Test** %refToSelf, align 8
      ret void
    }

    define void @__init_fb_test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__user_init_FB_Test(%FB_Test* %0) {
    entry:
      %self = alloca %FB_Test*, align 8
      store %FB_Test* %0, %FB_Test** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %FB = type { i16, %FB*, i16 }

    @__FB__init = constant %FB { i16 5, %FB* null, i16 5 }
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @FB(%FB* %0) {
    entry:
      %this = alloca %FB*, align 8
      store %FB* %0, %FB** %this, align 8
      %x = getelementptr inbounds %FB, %FB* %0, i32 0, i32 0
      %self = getelementptr inbounds %FB, %FB* %0, i32 0, i32 1
      %y = getelementptr inbounds %FB, %FB* %0, i32 0, i32 2
      ret void
    }

    define void @__init_fb(%FB* %0) {
    entry:
      %self = alloca %FB*, align 8
      store %FB* %0, %FB** %self, align 8
      ret void
    }

    define void @__user_init_FB(%FB* %0) {
    entry:
      %self = alloca %FB*, align 8
      store %FB* %0, %FB** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }
    "#);
}

#[test]
#[ignore = "not working"]
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
    insta::assert_snapshot!(code, @r#""#);
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
    insta::assert_snapshot!(code, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    %fb = type {}

    @__fb__init = constant %fb zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___Test, i8* null }]

    define void @fb(%fb* %0) {
    entry:
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      ret void
    }

    define i16 @fb_foo(%fb* %0) {
    entry:
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %fb.foo = alloca i16, align 2
      store i16 0, i16* %fb.foo, align 2
      %deref = load %fb*, %fb** %this, align 8
      call void @fb(%fb* %deref)
      %fb_foo_ret = load i16, i16* %fb.foo, align 2
      ret i16 %fb_foo_ret
    }

    define void @__init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__user_init_fb(%fb* %0) {
    entry:
      %self = alloca %fb*, align 8
      store %fb* %0, %fb** %self, align 8
      ret void
    }

    define void @__init___Test() {
    entry:
      ret void
    }
    "#);
}
