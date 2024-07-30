use crate::test_utils::tests::codegen;

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
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @d = global [10 x i32] zeroinitializer, section "var-$RUSTY$d:ai32"
    "###);
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
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    @d = global [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], section "var-$RUSTY$d:ai32"
    @__Data__init = unnamed_addr constant [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], section "var-$RUSTY$__Data__init:ai32"
    "###);
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
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { [10 x i32], [10 x i32] }

    @prg_instance = global %prg { [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9] }, section "var-$RUSTY$prg_instance:r2ai32ai32"
    @__Data__init = unnamed_addr constant [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], section "var-$RUSTY$__Data__init:ai32"

    define void @prg(%prg* %0) {
    entry:
      %a = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %b = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %1 = bitcast [10 x i32]* %a to i8*
      %2 = bitcast [10 x i32]* %b to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 %2, i64 ptrtoint ([10 x i32]* getelementptr ([10 x i32], [10 x i32]* null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
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
    insta::assert_snapshot!(codegen(src), @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %prg = type { [10 x i32], [3 x i32] }

    @prg_instance = global %prg { [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], [3 x i32] [i32 3, i32 4, i32 5] }, section "var-$RUSTY$prg_instance:r2ai32ai32"
    @__Data__init = unnamed_addr constant [10 x i32] [i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9], section "var-$RUSTY$__Data__init:ai32"
    @__prg.b__init = unnamed_addr constant [3 x i32] [i32 3, i32 4, i32 5]

    define void @prg(%prg* %0) {
    entry:
      %a = getelementptr inbounds %prg, %prg* %0, i32 0, i32 0
      %b = getelementptr inbounds %prg, %prg* %0, i32 0, i32 1
      %tmpVar = getelementptr inbounds [10 x i32], [10 x i32]* %a, i32 0, i32 2
      %tmpVar1 = getelementptr inbounds [3 x i32], [3 x i32]* %b, i32 0, i32 1
      %load_tmpVar = load i32, i32* %tmpVar1, align 4
      store i32 %load_tmpVar, i32* %tmpVar, align 4
      ret void
    }
    "###);
}
