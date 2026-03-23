use plc_util::filtered_assert_snapshot;
use test_utils::codegen;
/// # Architecture Design Record: Structs
/// ST supports C-like structures.
///
#[test]
fn declaring_a_struct() {
    // a struct type ...
    let src = r#"
        TYPE Person:
        STRUCT
            firstName   : STRING := 'Rusty';
            lastName    : STRING := 'User';
            yearOfBirth : INT := 2000;
            isLoggedIn  : BOOL := TRUE;
        END_STRUCT
        END_TYPE
        "#;

    // ... just translates to a llvm struct type (with a generated constructor)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %Person = type { [81 x i8], [81 x i8], i16, i8 }

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"Rusty\00"
    @utf08_literal_1 = private unnamed_addr constant [5 x i8] c"User\00"

    define void @Person__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %firstName = getelementptr inbounds nuw %Person, ptr %deref, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %firstName, ptr align [filtered] @utf08_literal_0, i32 6, i1 false)
      %deref1 = load ptr, ptr %self, align [filtered]
      %lastName = getelementptr inbounds nuw %Person, ptr %deref1, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %lastName, ptr align [filtered] @utf08_literal_1, i32 5, i1 false)
      %deref2 = load ptr, ptr %self, align [filtered]
      %yearOfBirth = getelementptr inbounds nuw %Person, ptr %deref2, i32 0, i32 2
      store i16 2000, ptr %yearOfBirth, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %isLoggedIn = getelementptr inbounds nuw %Person, ptr %deref3, i32 0, i32 3
      store i8 1, ptr %isLoggedIn, align [filtered]
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    // ... instances of this struct-type will be initialized accordingly (with constructor calls)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %Person = type { [6 x i8], [6 x i8], i16, i8 }

    @p = global %Person { [6 x i8] c"Jane\00\00", [6 x i8] c"Row\00\00\00", i16 1988, i8 0 }
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [5 x i8] c"Jane\00"
    @utf08_literal_1 = private unnamed_addr constant [4 x i8] c"Row\00"

    define void @Person__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %firstName = getelementptr inbounds nuw %Person, ptr %deref, i32 0, i32 0
      call void @__Person_firstName__ctor(ptr %firstName)
      %deref1 = load ptr, ptr %self, align [filtered]
      %firstName2 = getelementptr inbounds nuw %Person, ptr %deref1, i32 0, i32 0
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %firstName2, ptr align [filtered] @utf08_literal_0, i32 5, i1 false)
      %deref3 = load ptr, ptr %self, align [filtered]
      %lastName = getelementptr inbounds nuw %Person, ptr %deref3, i32 0, i32 1
      call void @__Person_lastName__ctor(ptr %lastName)
      %deref4 = load ptr, ptr %self, align [filtered]
      %lastName5 = getelementptr inbounds nuw %Person, ptr %deref4, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %lastName5, ptr align [filtered] @utf08_literal_1, i32 4, i1 false)
      %deref6 = load ptr, ptr %self, align [filtered]
      %yearOfBirth = getelementptr inbounds nuw %Person, ptr %deref6, i32 0, i32 2
      store i16 1988, ptr %yearOfBirth, align [filtered]
      %deref7 = load ptr, ptr %self, align [filtered]
      %isLoggedIn = getelementptr inbounds nuw %Person, ptr %deref7, i32 0, i32 3
      store i8 0, ptr %isLoggedIn, align [filtered]
      ret void
    }

    define void @__Person_firstName__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__Person_lastName__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @Person__ctor(ptr @p)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    // ... will be initialized via constructors that set up the values
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { %Rect, %Rect }
    %Rect = type { %Point, %Point }
    %Point = type { i16, i16 }

    @prg_instance = global %prg { %Rect { %Point { i16 1, i16 5 }, %Point { i16 10, i16 15 } }, %Rect { %Point { i16 4, i16 6 }, %Point { i16 16, i16 22 } } }
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @__prg.rect1__init = unnamed_addr constant %Rect { %Point { i16 1, i16 5 }, %Point { i16 10, i16 15 } }
    @__prg.rect2__init = unnamed_addr constant %Rect { %Point { i16 4, i16 6 }, %Point { i16 16, i16 22 } }
    @__prg.rect1__init.1 = unnamed_addr constant %Rect { %Point { i16 1, i16 5 }, %Point { i16 10, i16 15 } }
    @__prg.rect2__init.2 = unnamed_addr constant %Rect { %Point { i16 4, i16 6 }, %Point { i16 16, i16 22 } }

    define void @prg(ptr %0) {
    entry:
      %rect1 = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %rect2 = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %rect1 = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @Rect__ctor(ptr %rect1)
      %deref1 = load ptr, ptr %self, align [filtered]
      %rect12 = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 0
      %topLeft = getelementptr inbounds nuw %Rect, ptr %rect12, i32 0, i32 0
      %x = getelementptr inbounds nuw %Point, ptr %topLeft, i32 0, i32 0
      store i16 1, ptr %x, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %rect14 = getelementptr inbounds nuw %prg, ptr %deref3, i32 0, i32 0
      %topLeft5 = getelementptr inbounds nuw %Rect, ptr %rect14, i32 0, i32 0
      %y = getelementptr inbounds nuw %Point, ptr %topLeft5, i32 0, i32 1
      store i16 5, ptr %y, align [filtered]
      %deref6 = load ptr, ptr %self, align [filtered]
      %rect17 = getelementptr inbounds nuw %prg, ptr %deref6, i32 0, i32 0
      %bottomRight = getelementptr inbounds nuw %Rect, ptr %rect17, i32 0, i32 1
      %x8 = getelementptr inbounds nuw %Point, ptr %bottomRight, i32 0, i32 0
      store i16 10, ptr %x8, align [filtered]
      %deref9 = load ptr, ptr %self, align [filtered]
      %rect110 = getelementptr inbounds nuw %prg, ptr %deref9, i32 0, i32 0
      %bottomRight11 = getelementptr inbounds nuw %Rect, ptr %rect110, i32 0, i32 1
      %y12 = getelementptr inbounds nuw %Point, ptr %bottomRight11, i32 0, i32 1
      store i16 15, ptr %y12, align [filtered]
      %deref13 = load ptr, ptr %self, align [filtered]
      %rect2 = getelementptr inbounds nuw %prg, ptr %deref13, i32 0, i32 1
      call void @Rect__ctor(ptr %rect2)
      %deref14 = load ptr, ptr %self, align [filtered]
      %rect215 = getelementptr inbounds nuw %prg, ptr %deref14, i32 0, i32 1
      %topLeft16 = getelementptr inbounds nuw %Rect, ptr %rect215, i32 0, i32 0
      %x17 = getelementptr inbounds nuw %Point, ptr %topLeft16, i32 0, i32 0
      store i16 4, ptr %x17, align [filtered]
      %deref18 = load ptr, ptr %self, align [filtered]
      %rect219 = getelementptr inbounds nuw %prg, ptr %deref18, i32 0, i32 1
      %topLeft20 = getelementptr inbounds nuw %Rect, ptr %rect219, i32 0, i32 0
      %y21 = getelementptr inbounds nuw %Point, ptr %topLeft20, i32 0, i32 1
      store i16 6, ptr %y21, align [filtered]
      %deref22 = load ptr, ptr %self, align [filtered]
      %rect223 = getelementptr inbounds nuw %prg, ptr %deref22, i32 0, i32 1
      %bottomRight24 = getelementptr inbounds nuw %Rect, ptr %rect223, i32 0, i32 1
      %x25 = getelementptr inbounds nuw %Point, ptr %bottomRight24, i32 0, i32 0
      store i16 16, ptr %x25, align [filtered]
      %deref26 = load ptr, ptr %self, align [filtered]
      %rect227 = getelementptr inbounds nuw %prg, ptr %deref26, i32 0, i32 1
      %bottomRight28 = getelementptr inbounds nuw %Rect, ptr %rect227, i32 0, i32 1
      %y29 = getelementptr inbounds nuw %Point, ptr %bottomRight28, i32 0, i32 1
      store i16 22, ptr %y29, align [filtered]
      ret void
    }

    define void @Point__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @Rect__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %topLeft = getelementptr inbounds nuw %Rect, ptr %deref, i32 0, i32 0
      call void @Point__ctor(ptr %topLeft)
      %deref1 = load ptr, ptr %self, align [filtered]
      %bottomRight = getelementptr inbounds nuw %Rect, ptr %deref1, i32 0, i32 1
      call void @Point__ctor(ptr %bottomRight)
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @prg__ctor(ptr @prg_instance)
      ret void
    }
    "#);
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
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { %Point, %Point }
    %Point = type { i16, i16 }

    @prg_instance = global %prg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prg(ptr %0) {
    entry:
      %p1 = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %p2 = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %p1, ptr align [filtered] %p2, i64 ptrtoint (ptr getelementptr (%Point, ptr null, i32 1) to i64), i1 false)
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %p1 = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @Point__ctor(ptr %p1)
      %deref1 = load ptr, ptr %self, align [filtered]
      %p2 = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 1
      call void @Point__ctor(ptr %p2)
      ret void
    }

    define void @Point__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @prg__ctor(ptr @prg_instance)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
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

    // ... member access uses GEP, and constructors are generated for initialization
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { %Rect, %Rect }
    %Rect = type { %Point, %Point }
    %Point = type { i16, i16 }

    @prg_instance = global %prg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prg(ptr %0) {
    entry:
      %rect1 = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %rect2 = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      %topLeft = getelementptr inbounds nuw %Rect, ptr %rect1, i32 0, i32 0
      %x = getelementptr inbounds nuw %Point, ptr %topLeft, i32 0, i32 0
      %bottomRight = getelementptr inbounds nuw %Rect, ptr %rect2, i32 0, i32 1
      %x1 = getelementptr inbounds nuw %Point, ptr %bottomRight, i32 0, i32 0
      %load_x = load i16, ptr %x1, align [filtered]
      store i16 %load_x, ptr %x, align [filtered]
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %rect1 = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @Rect__ctor(ptr %rect1)
      %deref1 = load ptr, ptr %self, align [filtered]
      %rect2 = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 1
      call void @Rect__ctor(ptr %rect2)
      ret void
    }

    define void @Point__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @Rect__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %topLeft = getelementptr inbounds nuw %Rect, ptr %deref, i32 0, i32 0
      call void @Point__ctor(ptr %topLeft)
      %deref1 = load ptr, ptr %self, align [filtered]
      %bottomRight = getelementptr inbounds nuw %Rect, ptr %deref1, i32 0, i32 1
      call void @Point__ctor(ptr %bottomRight)
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @prg__ctor(ptr @prg_instance)
      ret void
    }
    "#);
}
