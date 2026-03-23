use plc_util::filtered_assert_snapshot;
use test_utils::codegen;
/// # Architecture Design Record: Strings
/// ST supports two types of Strings: UTF8 and UTF16 Strings. Strings are fixed size and
/// stored using i8-arrays for utf8 strings and i16-arrays for utf16 strings.
#[test]
fn declaring_a_string() {
    // two string variables
    let src = r#"
        VAR_GLOBAL
            myUtf8  : STRING[20];
            myUtf16 : WSTRING[20];
        END_VAR
        "#;

    // ... are stored as i8/i16 arrays and get initialized to blank (0)
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myUtf8 = global [21 x i8] zeroinitializer
    @myUtf16 = global [21 x i16] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @__global_myUtf8__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_myUtf16__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__global_myUtf8__ctor(ptr @myUtf8)
      call void @__global_myUtf16__ctor(ptr @myUtf16)
      ret void
    }
    "#);
}

/// rusty treats strings like C-strings (char arrays) so the interoperability
/// with C-libraries is seamless.
#[test]
fn strings_are_terminated_with_0byte() {
    // two strings with an initial value ...
    let src = r#"
        VAR_GLOBAL
            myUtf8  : STRING[5]  := 'Hello';
            myUtf16 : WSTRING[5] := "World";
        END_VAR
        "#;

    // ... get stored as c-like char arrays with 0-terminators
    // ... offer one extra entry (length 21 while only 20 were declared) for a terminator
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @myUtf8 = global [6 x i8] c"Hello\00"
    @myUtf16 = global [6 x i16] [i16 87, i16 111, i16 114, i16 108, i16 100, i16 0]
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"Hello\00"
    @utf16_literal_0 = private unnamed_addr constant [6 x i16] [i16 87, i16 111, i16 114, i16 108, i16 100, i16 0]

    define void @__global_myUtf8__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__global_myUtf16__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @__global_myUtf8__ctor(ptr @myUtf8)
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @myUtf8, ptr align [filtered] @utf08_literal_0, i32 5, i1 false)
      call void @__global_myUtf16__ctor(ptr @myUtf16)
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] @myUtf16, ptr align [filtered] @utf16_literal_0, i32 10, i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

/// Strings are aggregate types. This means that passing them to functions and assigning them
/// are expensive operations (when compared to passing ar assigning an INT). Aggregate types
/// are assigned using memcpy.
#[test]
fn assigning_strings() {
    // two strings get assigned ...
    let src = r#"
        PROGRAM prg
            VAR
                a,b : STRING[10];
            END_VAR
             a := b;
        END_PROGRAM
        "#;

    // ... the assignments will be performed as a memcpy
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { [11 x i8], [11 x i8] }

    @prg_instance = global %prg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prg(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %a, ptr align [filtered] %b, i32 10, i1 false)
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %a = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @__prg_a__ctor(ptr %a)
      %deref1 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 1
      call void @__prg_b__ctor(ptr %b)
      ret void
    }

    define void @__prg_a__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__prg_b__ctor(ptr %0) {
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
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

/// STRING literals will be generated as global constants, so they can be used as a source of a memcpy
/// operation
#[test]
fn assigning_string_literals() {
    // two nested struct-variables that get initialized ...
    let src = r#"
        PROGRAM prg
            VAR
                a,b : STRING[10];
            END_VAR
             a := 'hello';
             b := 'world';
        END_PROGRAM
        "#;

    // ... will be initialized directly in the variable's definition
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { [11 x i8], [11 x i8] }

    @prg_instance = global %prg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"
    @utf08_literal_1 = private unnamed_addr constant [6 x i8] c"world\00"

    define void @prg(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %a, ptr align [filtered] @utf08_literal_0, i32 6, i1 false)
      call void @llvm.memcpy.p0.p0.i32(ptr align [filtered] %b, ptr align [filtered] @utf08_literal_1, i32 6, i1 false)
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %a = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @__prg_a__ctor(ptr %a)
      %deref1 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 1
      call void @__prg_b__ctor(ptr %b)
      ret void
    }

    define void @__prg_a__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__prg_b__ctor(ptr %0) {
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
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
