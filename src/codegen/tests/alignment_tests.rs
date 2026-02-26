use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot_with_alignments;

// Tests for aligment of datatypes on x86_64 and aarch64
#[test]
#[cfg(target_os = "macos")]
fn test_datatype_alignment() {
    let result = codegen(
        r#"FUNCTION main
VAR
a : BYTE;      // 1 byte
b : WORD;      // 2 bytes
c : DWORD;     // 4 bytes
d : LWORD;     // 8 bytes
e : INT;       // 2 bytes
f : DINT;      // 4 bytes
g : LINT;      // 8 bytes
h : REAL;      // 4 bytes
i : LREAL;     // 8 bytes
j : BOOL;      // 1 byte
END_VAR
END_FUNCTION
"#,
    );

    // Arm assertion
    #[cfg(target_arch = "aarch64")]
    filtered_assert_snapshot_with_alignments!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %a = alloca i8, align 1
      %b = alloca i16, align 2
      %c = alloca i32, align 4
      %d = alloca i64, align 8
      %e = alloca i16, align 2
      %f = alloca i32, align 4
      %g = alloca i64, align 8
      %h = alloca float, align 4
      %i = alloca double, align 8
      %j = alloca i8, align 1
      store i8 0, ptr %a, align 1
      store i16 0, ptr %b, align 2
      store i32 0, ptr %c, align 4
      store i64 0, ptr %d, align 8
      store i16 0, ptr %e, align 2
      store i32 0, ptr %f, align 4
      store i64 0, ptr %g, align 8
      store float 0.000000e+00, ptr %h, align 4
      store double 0.000000e+00, ptr %i, align 8
      store i8 0, ptr %j, align 1
      ret void
    }
    "#);

    // x86_64 assertion
    #[cfg(target_arch = "x86_64")]
    filtered_assert_snapshot_with_alignments!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define void @main() {
    entry:
      %a = alloca i8, align 1
      %b = alloca i16, align 2
      %c = alloca i32, align 4
      %d = alloca i64, align 8
      %e = alloca i16, align 2
      %f = alloca i32, align 4
      %g = alloca i64, align 8
      %h = alloca float, align 4
      %i = alloca double, align 8
      %j = alloca i8, align 1
      store i8 0, ptr %a, align 1
      store i16 0, ptr %b, align 2
      store i32 0, ptr %c, align 4
      store i64 0, ptr %d, align 8
      store i16 0, ptr %e, align 2
      store i32 0, ptr %f, align 4
      store i64 0, ptr %g, align 8
      store float 0.000000e+00, ptr %h, align 4
      store double 0.000000e+00, ptr %i, align 8
      store i8 0, ptr %j, align 1
      ret void
    }
    "#);
}

#[test]
#[cfg(target_os = "macos")]
fn test_struct_alignment() {
    let result = codegen(
        r#"FUNCTION main
                VAR
                x : MyStruct;
                y : MyStruct;
                a : BOOL;
                END_VAR
                END_FUNCTION
                TYPE
                MyStruct : STRUCT
                    a : BYTE;      // 1 byte
                    b : WORD;      // 2 bytes
                    c : DWORD;     // 4 bytes
                    d : LWORD;     // 8 bytes
                    e : INT;       // 2 bytes
                    f : DINT;      // 4 bytes
                    g : LINT;      // 8 bytes
                    h : REAL;      // 4 bytes
                    i : LREAL;     // 8 bytes
                    j : BOOL;      // 1 byte
                END_STRUCT
                END_TYPE
        "#,
    );

    // Arm assertion
    #[cfg(target_arch = "aarch64")]
    filtered_assert_snapshot_with_alignments!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %MyStruct = type { i8, i16, i32, i64, i16, i32, i64, float, double, i8 }

    @__MyStruct__init = unnamed_addr constant %MyStruct zeroinitializer

    define void @main() {
    entry:
      %x = alloca %MyStruct, align 8
      %y = alloca %MyStruct, align 8
      %a = alloca i8, align 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %x, ptr align 1 @__MyStruct__init, i64 ptrtoint (ptr getelementptr (%MyStruct, ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %y, ptr align 1 @__MyStruct__init, i64 ptrtoint (ptr getelementptr (%MyStruct, ptr null, i32 1) to i64), i1 false)
      store i8 0, ptr %a, align 1
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);

    // x86_64 assertion
    #[cfg(target_arch = "x86_64")]
    filtered_assert_snapshot_with_alignments!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %MyStruct = type { i8, i16, i32, i64, i16, i32, i64, float, double, i8 }

    @__MyStruct__init = unnamed_addr constant %MyStruct zeroinitializer

    define void @main() {
    entry:
      %x = alloca %MyStruct, align 8
      %y = alloca %MyStruct, align 8
      %a = alloca i8, align 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %x, ptr align 1 @__MyStruct__init, i64 ptrtoint (ptr getelementptr (%MyStruct, ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %y, ptr align 1 @__MyStruct__init, i64 ptrtoint (ptr getelementptr (%MyStruct, ptr null, i32 1) to i64), i1 false)
      store i8 0, ptr %a, align 1
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
