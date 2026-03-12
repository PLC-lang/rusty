use plc_util::filtered_assert_snapshot;
use test_utils::codegen;
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
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @Data__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @Data__ctor(ptr @d)
      ret void
    }
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

    @d = global [10 x i32] zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @Data__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %tmpVar = getelementptr inbounds [10 x i32], ptr %deref, i32 0, i32 0
      store i32 0, ptr %tmpVar, align [filtered]
      %deref1 = load ptr, ptr %self, align [filtered]
      %tmpVar2 = getelementptr inbounds [10 x i32], ptr %deref1, i32 0, i32 1
      store i32 1, ptr %tmpVar2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %tmpVar4 = getelementptr inbounds [10 x i32], ptr %deref3, i32 0, i32 2
      store i32 2, ptr %tmpVar4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %tmpVar6 = getelementptr inbounds [10 x i32], ptr %deref5, i32 0, i32 3
      store i32 3, ptr %tmpVar6, align [filtered]
      %deref7 = load ptr, ptr %self, align [filtered]
      %tmpVar8 = getelementptr inbounds [10 x i32], ptr %deref7, i32 0, i32 4
      store i32 4, ptr %tmpVar8, align [filtered]
      %deref9 = load ptr, ptr %self, align [filtered]
      %tmpVar10 = getelementptr inbounds [10 x i32], ptr %deref9, i32 0, i32 5
      store i32 5, ptr %tmpVar10, align [filtered]
      %deref11 = load ptr, ptr %self, align [filtered]
      %tmpVar12 = getelementptr inbounds [10 x i32], ptr %deref11, i32 0, i32 6
      store i32 6, ptr %tmpVar12, align [filtered]
      %deref13 = load ptr, ptr %self, align [filtered]
      %tmpVar14 = getelementptr inbounds [10 x i32], ptr %deref13, i32 0, i32 7
      store i32 7, ptr %tmpVar14, align [filtered]
      %deref15 = load ptr, ptr %self, align [filtered]
      %tmpVar16 = getelementptr inbounds [10 x i32], ptr %deref15, i32 0, i32 8
      store i32 8, ptr %tmpVar16, align [filtered]
      %deref17 = load ptr, ptr %self, align [filtered]
      %tmpVar18 = getelementptr inbounds [10 x i32], ptr %deref17, i32 0, i32 9
      store i32 9, ptr %tmpVar18, align [filtered]
      ret void
    }

    define void @__unit___internal____ctor() {
    entry:
      call void @Data__ctor(ptr @d)
      ret void
    }
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

    @prg_instance = global %prg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prg(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      call void @llvm.memcpy.p0.p0.i64(ptr align [filtered] %a, ptr align [filtered] %b, i64 ptrtoint (ptr getelementptr ([10 x i32], ptr null, i32 1) to i64), i1 false)
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %a = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @Data__ctor(ptr %a)
      %deref1 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 1
      call void @Data__ctor(ptr %b)
      ret void
    }

    define void @Data__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %tmpVar = getelementptr inbounds [10 x i32], ptr %deref, i32 0, i32 0
      store i32 0, ptr %tmpVar, align [filtered]
      %deref1 = load ptr, ptr %self, align [filtered]
      %tmpVar2 = getelementptr inbounds [10 x i32], ptr %deref1, i32 0, i32 1
      store i32 1, ptr %tmpVar2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %tmpVar4 = getelementptr inbounds [10 x i32], ptr %deref3, i32 0, i32 2
      store i32 2, ptr %tmpVar4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %tmpVar6 = getelementptr inbounds [10 x i32], ptr %deref5, i32 0, i32 3
      store i32 3, ptr %tmpVar6, align [filtered]
      %deref7 = load ptr, ptr %self, align [filtered]
      %tmpVar8 = getelementptr inbounds [10 x i32], ptr %deref7, i32 0, i32 4
      store i32 4, ptr %tmpVar8, align [filtered]
      %deref9 = load ptr, ptr %self, align [filtered]
      %tmpVar10 = getelementptr inbounds [10 x i32], ptr %deref9, i32 0, i32 5
      store i32 5, ptr %tmpVar10, align [filtered]
      %deref11 = load ptr, ptr %self, align [filtered]
      %tmpVar12 = getelementptr inbounds [10 x i32], ptr %deref11, i32 0, i32 6
      store i32 6, ptr %tmpVar12, align [filtered]
      %deref13 = load ptr, ptr %self, align [filtered]
      %tmpVar14 = getelementptr inbounds [10 x i32], ptr %deref13, i32 0, i32 7
      store i32 7, ptr %tmpVar14, align [filtered]
      %deref15 = load ptr, ptr %self, align [filtered]
      %tmpVar16 = getelementptr inbounds [10 x i32], ptr %deref15, i32 0, i32 8
      store i32 8, ptr %tmpVar16, align [filtered]
      %deref17 = load ptr, ptr %self, align [filtered]
      %tmpVar18 = getelementptr inbounds [10 x i32], ptr %deref17, i32 0, i32 9
      store i32 9, ptr %tmpVar18, align [filtered]
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

    @prg_instance = global %prg zeroinitializer
    @llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @__unit___internal____ctor, ptr null }]

    define void @prg(ptr %0) {
    entry:
      %a = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 0
      %b = getelementptr inbounds nuw %prg, ptr %0, i32 0, i32 1
      %tmpVar = getelementptr inbounds [10 x i32], ptr %a, i32 0, i32 2
      %tmpVar1 = getelementptr inbounds [3 x i32], ptr %b, i32 0, i32 1
      %load_tmpVar = load i32, ptr %tmpVar1, align [filtered]
      store i32 %load_tmpVar, ptr %tmpVar, align [filtered]
      ret void
    }

    define void @prg__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %a = getelementptr inbounds nuw %prg, ptr %deref, i32 0, i32 0
      call void @Data__ctor(ptr %a)
      %deref1 = load ptr, ptr %self, align [filtered]
      %b = getelementptr inbounds nuw %prg, ptr %deref1, i32 0, i32 1
      call void @__prg_b__ctor(ptr %b)
      %deref2 = load ptr, ptr %self, align [filtered]
      %b3 = getelementptr inbounds nuw %prg, ptr %deref2, i32 0, i32 1
      %tmpVar = getelementptr inbounds [3 x i32], ptr %b3, i32 0, i32 0
      store i32 3, ptr %tmpVar, align [filtered]
      %deref4 = load ptr, ptr %self, align [filtered]
      %b5 = getelementptr inbounds nuw %prg, ptr %deref4, i32 0, i32 1
      %tmpVar6 = getelementptr inbounds [3 x i32], ptr %b5, i32 0, i32 1
      store i32 4, ptr %tmpVar6, align [filtered]
      %deref7 = load ptr, ptr %self, align [filtered]
      %b8 = getelementptr inbounds nuw %prg, ptr %deref7, i32 0, i32 1
      %tmpVar9 = getelementptr inbounds [3 x i32], ptr %b8, i32 0, i32 2
      store i32 5, ptr %tmpVar9, align [filtered]
      ret void
    }

    define void @Data__ctor(ptr %0) {
    entry:
      %self = alloca ptr, align [filtered]
      store ptr %0, ptr %self, align [filtered]
      %deref = load ptr, ptr %self, align [filtered]
      %tmpVar = getelementptr inbounds [10 x i32], ptr %deref, i32 0, i32 0
      store i32 0, ptr %tmpVar, align [filtered]
      %deref1 = load ptr, ptr %self, align [filtered]
      %tmpVar2 = getelementptr inbounds [10 x i32], ptr %deref1, i32 0, i32 1
      store i32 1, ptr %tmpVar2, align [filtered]
      %deref3 = load ptr, ptr %self, align [filtered]
      %tmpVar4 = getelementptr inbounds [10 x i32], ptr %deref3, i32 0, i32 2
      store i32 2, ptr %tmpVar4, align [filtered]
      %deref5 = load ptr, ptr %self, align [filtered]
      %tmpVar6 = getelementptr inbounds [10 x i32], ptr %deref5, i32 0, i32 3
      store i32 3, ptr %tmpVar6, align [filtered]
      %deref7 = load ptr, ptr %self, align [filtered]
      %tmpVar8 = getelementptr inbounds [10 x i32], ptr %deref7, i32 0, i32 4
      store i32 4, ptr %tmpVar8, align [filtered]
      %deref9 = load ptr, ptr %self, align [filtered]
      %tmpVar10 = getelementptr inbounds [10 x i32], ptr %deref9, i32 0, i32 5
      store i32 5, ptr %tmpVar10, align [filtered]
      %deref11 = load ptr, ptr %self, align [filtered]
      %tmpVar12 = getelementptr inbounds [10 x i32], ptr %deref11, i32 0, i32 6
      store i32 6, ptr %tmpVar12, align [filtered]
      %deref13 = load ptr, ptr %self, align [filtered]
      %tmpVar14 = getelementptr inbounds [10 x i32], ptr %deref13, i32 0, i32 7
      store i32 7, ptr %tmpVar14, align [filtered]
      %deref15 = load ptr, ptr %self, align [filtered]
      %tmpVar16 = getelementptr inbounds [10 x i32], ptr %deref15, i32 0, i32 8
      store i32 8, ptr %tmpVar16, align [filtered]
      %deref17 = load ptr, ptr %self, align [filtered]
      %tmpVar18 = getelementptr inbounds [10 x i32], ptr %deref17, i32 0, i32 9
      store i32 9, ptr %tmpVar18, align [filtered]
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
    "#);
}
