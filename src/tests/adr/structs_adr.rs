use crate::test_utils::tests::codegen;

/// # Architecture Design Record: Structs
/// ST supports C-like structures.
///
#[test]
fn declaring_a_struct() {
    // a struct type ...
    let src = r#"
        TYPE Person:
        STRUCT
            firstName   : STRING;
            lastName    : STRING;
            yearOfBirth : INT;
            isLoggedIn  : BOOL;
        END_STRUCT
        END_TYPE
        "#;

    // ... just translates to a llvm struct type
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %Person = type { [81 x i8], [81 x i8], i16, i8 }

    @__Person__init = unnamed_addr constant %Person zeroinitializer, section "var-$RUSTY$__Person__init:r4s8u81s8u81i16u8"
    "###);
}

/// Structs can have a default value by initializing its fields.
#[test]
fn default_values_of_a_struct() {
    // The fields of a struct can get default values ...
    let src = r#"
        TYPE Person:
        STRUCT
            firstName   : STRING[5] := 'Jane';
            lastName    : STRING[5] := 'Row';
            yearOfBirth : INT    := 1988;
            isLoggedIn  : BOOL   := FALSE;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
            p : Person;
        END_VAR
        "#;

    // ... instances of this struct-type will be initialized accordingly
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %Person = type { [6 x i8], [6 x i8], i16, i8 }

    @p = global %Person { [6 x i8] c"Jane\00\00", [6 x i8] c"Row\00\00\00", i16 1988, i8 0 }, section "var-$RUSTY$p:r4s8u6s8u6i16u8"
    @__Person__init = unnamed_addr constant %Person { [6 x i8] c"Jane\00\00", [6 x i8] c"Row\00\00\00", i16 1988, i8 0 }, section "var-$RUSTY$__Person__init:r4s8u6s8u6i16u8"
    "###);
}

/// a Struct can also be assigned a struct-literal.
#[test]
fn initializing_a_struct() {
    // two nested struct-variables that get initialized ...
    let src = r#"
        TYPE Point:
        STRUCT
            x, y   : INT;
        END_STRUCT
        END_TYPE

        TYPE Rect:
        STRUCT
            topLeft, bottomRight   : Point;
        END_STRUCT
        END_TYPE

        PROGRAM prg
            VAR
                rect1  : Rect := (
                            topLeft := (x := 1, y := 5),
                            bottomRight := (x := 10, y := 15));
                rect2  : Rect := (
                            topLeft := (x := 4, y := 6),
                            bottomRight := (x := 16, y := 22));
            END_VAR
        END_PROGRAM
        "#;

    // ... will be initialized directly in the variable's definition
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { %Rect, %Rect }
    %Rect = type { %Point, %Point }
    %Point = type { i16, i16 }

    @prg_instance = global %prg { %Rect { %Point { i16 1, i16 5 }, %Point { i16 10, i16 15 } }, %Rect { %Point { i16 4, i16 6 }, %Point { i16 16, i16 22 } } }, section "var-$RUSTY$prg_instance:r2r2r2i16i16r2i16i16r2r2i16i16r2i16i16"
    @__Rect__init = unnamed_addr constant %Rect zeroinitializer, section "var-$RUSTY$__Rect__init:r2r2i16i16r2i16i16"
    @__Point__init = unnamed_addr constant %Point zeroinitializer, section "var-$RUSTY$__Point__init:r2i16i16"
    @__prg.rect1__init = unnamed_addr constant %Rect { %Point { i16 1, i16 5 }, %Point { i16 10, i16 15 } }
    @__prg.rect2__init = unnamed_addr constant %Rect { %Point { i16 4, i16 6 }, %Point { i16 16, i16 22 } }

    define void @prg(%prg* %0) {
    entry:
      %rect1 = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %rect2 = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      ret void
    }
    "###);
}

/// Structs are aggregate types. This means that passing them to functions and assigning them
/// are expensive operations (when compared to passing ar assigning an INT). Aggregate types like
/// structs and arrays are assigned using memcpy.
#[test]
fn assigning_structs() {
    // two struct instances that get assigned and passed to a function ...
    let src = r#"
        TYPE Point:
        STRUCT
            x, y   : INT;
        END_STRUCT
        END_TYPE

        PROGRAM prg
            VAR
                p1  : Point;
                p2  : Point;
            END_VAR

            p1 := p2;
        END_PROGRAM
        "#;

    // ... the assignment p1 := p2 will be performed as a memcpy
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { %Point, %Point }
    %Point = type { i16, i16 }

    @prg_instance = global %prg zeroinitializer, section "var-$RUSTY$prg_instance:r2r2i16i16r2i16i16"
    @__Point__init = unnamed_addr constant %Point zeroinitializer, section "var-$RUSTY$__Point__init:r2i16i16"

    define void @prg(%prg* %0) {
    entry:
      %p1 = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %p2 = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %1 = bitcast %Point* %p1 to i8*
      %2 = bitcast %Point* %p2 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %2, i64 ptrtoint (%Point* getelementptr (%Point, %Point* null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

/// Accessing STRUCT's members uses the LLVM GEP statement to get a pointer
/// to the struct's elements.
#[test]
fn accessing_struct_members() {
    // two nested struct-variables that get initialized ...
    let src = r#"
        TYPE Point:
        STRUCT
            x, y   : INT;
        END_STRUCT
        END_TYPE

        TYPE Rect:
        STRUCT
            topLeft, bottomRight   : Point;
        END_STRUCT
        END_TYPE

        PROGRAM prg
            VAR
                rect1  : Rect;
                rect2  : Rect;
            END_VAR

            rect1.topLeft.x := rect2.bottomRight.x;
        END_PROGRAM
        "#;

    // ... will be initialized directly in the variable's definition
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { %Rect, %Rect }
    %Rect = type { %Point, %Point }
    %Point = type { i16, i16 }

    @prg_instance = global %prg zeroinitializer, section "var-$RUSTY$prg_instance:r2r2r2i16i16r2i16i16r2r2i16i16r2i16i16"
    @__Rect__init = unnamed_addr constant %Rect zeroinitializer, section "var-$RUSTY$__Rect__init:r2r2i16i16r2i16i16"
    @__Point__init = unnamed_addr constant %Point zeroinitializer, section "var-$RUSTY$__Point__init:r2i16i16"

    define void @prg(%prg* %0) {
    entry:
      %rect1 = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %rect2 = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %topLeft = getelementptr inbounds %Rect, %Rect* %rect1, i32 0, i32 0
      %x = getelementptr inbounds %Point, %Point* %topLeft, i32 0, i32 0
      %bottomRight = getelementptr inbounds %Rect, %Rect* %rect2, i32 0, i32 1
      %x1 = getelementptr inbounds %Point, %Point* %bottomRight, i32 0, i32 0
      %load_x = load i16, i16* %x1, align 2
      store i16 %load_x, i16* %x, align 2
      ret void
    }
    "###);
}
