use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;
/// # Architecture Design Record: Arrays
/// ST supports C-like arrays
#[test]
fn declaring_an_array() {
    // an array type ...
    let src = r#"
        TYPE Data: ARRAY[0..9] OF DINT; END_TYPE

        VAR_GLOBAL
            d : Data;
        END_VAR
        "#;

    // ... just translates to a llvm array type
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @d = global [10 x i32] zeroinitializer
    "#);
}

/// Arrays can have a default value by initializing it. Variables of this
/// type will be initialized automatically.
#[test]
fn initializing_an_array() {
    // The fields of a struct can get default values ...
    let src = r#"
        TYPE Data: ARRAY[0..9] OF DINT := [0,1,2,3,4,5,6,7,8,9]; END_TYPE

        VAR_GLOBAL
            d : Data;
        END_VAR
        "#;

    // ... Instances of this struct will be initialized accordingly
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    @d = global [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9]
    @__Data__init = unnamed_addr constant [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9]
    "#);
}

/// Arrays are aggregate types. This means that passing them to functions and assigning them
/// are expensive operations (when compared to passing ar assigning an INT). Aggregate types like
/// structs and arrays are assigned using memcpy.
#[test]
fn assigning_full_arrays() {
    // two struct instances that get assigned...
    let src = r#"
        TYPE Data: ARRAY[0..9] OF DINT := [0,1,2,3,4,5,6,7,8,9]; END_TYPE

        PROGRAM prg
            VAR
                a,b : Data;
            END_VAR

             a := b;
        END_PROGRAM
        "#;

    // ... the assignment a := b will be performed as a memcpy
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { [10 x i32], [10 x i32] }

    @prg_instance = global %prg { [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9] }
    @__Data__init = unnamed_addr constant [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9]

    define void @prg(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %a, ptr align 1 %b, i64 ptrtoint (ptr getelementptr ([10 x i32], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

/// Accessing ARRAY's members uses the LLVM GEP statement to get a pointer
/// to its elements.
#[test]
fn accessing_array_elements() {
    // an array 'a' that is initialized ...
    // an array 'b' that is defined from 2..5
    let src = r#"
        TYPE Data: ARRAY[0..9] OF DINT := [0,1,2,3,4,5,6,7,8,9]; END_TYPE

        PROGRAM prg
            VAR
                // array with default value
                a : Data;

                // array with 3-based index
                b : ARRAY[3..5] OF DINT := [3,4,5];
            END_VAR

            a[2] := b[4];
        END_PROGRAM
        "#;

    // ... both will use 0-based indexing internally, although one is 0-based and the other is 3-based
    // ... note that the b[4] access is generated as a gep-expression at index 1
    // .   %tmpVar1 = getelementptr inbounds [3 x i32], [3 x i32]* %b, i32 0, i32 1
    // .                                                                      ^^^^^
    filtered_assert_snapshot!(codegen(src), @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %prg = type { [10 x i32], [3 x i32] }

    @prg_instance = global %prg { [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], [3 x i32] [i32 3, i32 4, i32 5] }
    @__Data__init = unnamed_addr constant [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9]
    @__prg.b__init = unnamed_addr constant [3 x i32] [i32 3, i32 4, i32 5]

    define void @prg(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      %tmpVar = getelementptr inbounds [10 x i32], ptr %a, i32 0, i32 2
      %tmpVar1 = getelementptr inbounds [3 x i32], ptr %b, i32 0, i32 1
      %load_tmpVar = load i32, ptr %tmpVar1, align 4
      store i32 %load_tmpVar, ptr %tmpVar, align 4
      ret void
    }
    "#);
}
