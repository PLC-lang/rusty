use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn function_pointer_method_no_parameters() {
    let result = codegen(
        r"
        FUNCTION_BLOCK A
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                fooPtr: __FPOINTER A.foo := ADR(A.foo);
            END_VAR

            fooPtr^(instanceA);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %A = type {}

    @__A__init = unnamed_addr constant %A zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define void @A__foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca ptr, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceA, ptr align 1 @__A__init, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      store ptr @A__foo, ptr %fooPtr, align 8
      %0 = load ptr, ptr %fooPtr, align 8
      call void %0(ptr %instanceA)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn function_pointer_method_with_return_type() {
    let result = codegen(
        r"
        FUNCTION_BLOCK A
            METHOD foo: DINT
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                fooPtr: __FPOINTER A.foo := ADR(A.foo);
            END_VAR

            fooPtr^(instanceA);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %A = type {}

    @__A__init = unnamed_addr constant %A zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define i32 @A__foo(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %A.foo = alloca i32, align 4
      store i32 0, ptr %A.foo, align 4
      %A__foo_ret = load i32, ptr %A.foo, align 4
      ret i32 %A__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca ptr, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceA, ptr align 1 @__A__init, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      store ptr @A__foo, ptr %fooPtr, align 8
      %0 = load ptr, ptr %fooPtr, align 8
      %fnptr_call = call i32 %0(ptr %instanceA)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn function_pointer_method_with_return_type_aggregate() {
    let result = codegen(
        r"
        FUNCTION_BLOCK A
            METHOD foo: STRING
              foo := 'aaaaa';
            END_METHOD

            METHOD bar: ARRAY[1..5] OF DINT
              bar := [1, 2, 3, 4, 5];
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                fooPtr: __FPOINTER A.foo := ADR(A.foo);
                barPtr: __FPOINTER A.bar := ADR(A.bar);
            END_VAR

            fooPtr^(instanceA);
            barPtr^(instanceA);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %A = type {}

    @__A__init = unnamed_addr constant %A zeroinitializer
    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"aaaaa\00"

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define void @A__foo(ptr %0, ptr %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %foo = alloca ptr, align 8
      store ptr %1, ptr %foo, align 8
      %deref = load ptr, ptr %foo, align 8
      call void @llvm.memcpy.p0.p0.i32(ptr align 1 %deref, ptr align 1 @utf08_literal_0, i32 6, i1 false)
      ret void
    }

    define void @A__bar(ptr %0, ptr %1) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %bar = alloca ptr, align 8
      store ptr %1, ptr %bar, align 8
      %deref = load ptr, ptr %bar, align 8
      store [5 x i32] [i32 1, i32 2, i32 3, i32 4, i32 5], ptr %deref, align 4
      ret void
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca ptr, align 8
      %barPtr = alloca ptr, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceA, ptr align 1 @__A__init, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      store ptr @A__foo, ptr %fooPtr, align 8
      store ptr @A__bar, ptr %barPtr, align 8
      %__0 = alloca [81 x i8], align 1
      call void @llvm.memset.p0.i64(ptr align 1 %__0, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      %0 = load ptr, ptr %fooPtr, align 8
      call void %0(ptr %instanceA, ptr %__0)
      %load___0 = load [81 x i8], ptr %__0, align 1
      %__1 = alloca [5 x i32], align 4
      call void @llvm.memset.p0.i64(ptr align 1 %__1, i8 0, i64 ptrtoint (ptr getelementptr ([5 x i32], ptr null, i32 1) to i64), i1 false)
      %1 = load ptr, ptr %barPtr, align 8
      call void %1(ptr %instanceA, ptr %__1)
      %load___1 = load [5 x i32], ptr %__1, align 4
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i32(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i32, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

#[test]
fn function_pointer_method_with_all_variable_parameter_types() {
    let result = codegen(
        r"
        FUNCTION_BLOCK A
            METHOD foo: DINT
              VAR_INPUT
                in: DINT;
              END_VAR

              VAR_OUTPUT
                out: STRING;
              END_VAR

              VAR_IN_OUT
                inout: ARRAY[1..5] OF DINT;
              END_VAR

            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                fooPtr: __FPOINTER A.foo := ADR(A.foo);
                localIn: DINT;
                localOut: STRING;
                localInOut: ARRAY[1..5] OF DINT;
            END_VAR

            // fooPtr^(instanceA, localIn, localOut, localInOut);
            fooPtr^(instanceA, out => localOut, inout := localInOut, in := localIn); // Arguments shifted by one to the right
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %A = type {}

    @__A__init = unnamed_addr constant %A zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define i32 @A__foo(ptr %0, i32 %1, ptr %2, ptr %3) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %A.foo = alloca i32, align 4
      %in = alloca i32, align 4
      store i32 %1, ptr %in, align 4
      %out = alloca ptr, align 8
      store ptr %2, ptr %out, align 8
      %inout = alloca ptr, align 8
      store ptr %3, ptr %inout, align 8
      store i32 0, ptr %A.foo, align 4
      %A__foo_ret = load i32, ptr %A.foo, align 4
      ret i32 %A__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca ptr, align 8
      %localIn = alloca i32, align 4
      %localOut = alloca [81 x i8], align 1
      %localInOut = alloca [5 x i32], align 4
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceA, ptr align 1 @__A__init, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      store ptr @A__foo, ptr %fooPtr, align 8
      store i32 0, ptr %localIn, align 4
      call void @llvm.memset.p0.i64(ptr align 1 %localOut, i8 0, i64 ptrtoint (ptr getelementptr ([81 x i8], ptr null, i32 1) to i64), i1 false)
      call void @llvm.memset.p0.i64(ptr align 1 %localInOut, i8 0, i64 ptrtoint (ptr getelementptr ([5 x i32], ptr null, i32 1) to i64), i1 false)
      %0 = load ptr, ptr %fooPtr, align 8
      %load_localIn = load i32, ptr %localIn, align 4
      %fnptr_call = call i32 %0(ptr %instanceA, i32 %load_localIn, ptr %localOut, ptr %localInOut)
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
    declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #1

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: write) }
    "#);
}

#[test]
fn function_block_body() {
    let result = codegen(
        r"
        FUNCTION_BLOCK A
            VAR
                local: DINT;
            END_VAR

            VAR_INPUT
                in: INT;
            END_VAR

            VAR_OUTPUT
                out: DINT;
            END_VAR

            VAR_IN_OUT
                inout: LINT;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceA: A;
                bodyPtr: __FPOINTER A := ADR(A);

                localIn: INT;
                localOut: DINT;
                localInout: LINT;
            END_VAR

            bodyPtr^(instanceA, localIn, localOut, localInout);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %A = type { i32, i16, i32, ptr }

    @__A__init = unnamed_addr constant %A zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %local = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 0
      %in = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 1
      %out = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 2
      %inout = getelementptr inbounds nuw %A, ptr %0, i32 0, i32 3
      ret void
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %bodyPtr = alloca ptr, align 8
      %localIn = alloca i16, align 2
      %localOut = alloca i32, align 4
      %localInout = alloca i64, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %instanceA, ptr align 1 @__A__init, i64 ptrtoint (ptr getelementptr (%A, ptr null, i32 1) to i64), i1 false)
      store ptr @A, ptr %bodyPtr, align 8
      store i16 0, ptr %localIn, align 2
      store i32 0, ptr %localOut, align 4
      store i64 0, ptr %localInout, align 8
      %0 = load ptr, ptr %bodyPtr, align 8
      %1 = getelementptr inbounds %A, ptr %instanceA, i32 0, i32 1
      %load_localIn = load i16, ptr %localIn, align 2
      store i16 %load_localIn, ptr %1, align 2
      %2 = getelementptr inbounds %A, ptr %instanceA, i32 0, i32 3
      store ptr %localInout, ptr %2, align 8
      call void %0(ptr %instanceA)
      %3 = getelementptr inbounds %A, ptr %instanceA, i32 0, i32 2
      %4 = load i32, ptr %3, align 4
      store i32 %4, ptr %localOut, align 4
      ret void
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}

#[test]
fn regular_pointers_to_function_blocks_are_called_directly() {
    let result = codegen(
        r"
        FUNCTION_BLOCK A
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                fooPtr: POINTER TO A;
            END_VAR

            fooPtr^();
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %A = type {}

    @__A__init = unnamed_addr constant %A zeroinitializer

    define void @A(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      ret void
    }

    define void @main() {
    entry:
      %fooPtr = alloca ptr, align 8
      store ptr null, ptr %fooPtr, align 8
      %deref = load ptr, ptr %fooPtr, align 8
      call void @A(ptr %deref)
      ret void
    }
    "#);
}
