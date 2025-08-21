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
                fooPtr: FNPTR A.foo := ADR(A.foo);
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

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      ret void
    }

    define void @A__foo(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      ret void
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca void (%A*)*, align 8
      %0 = bitcast %A* %instanceA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%A* @__A__init to i8*), i64 ptrtoint (%A* getelementptr (%A, %A* null, i32 1) to i64), i1 false)
      store void (%A*)* @A__foo, void (%A*)** %fooPtr, align 8
      %1 = load void (%A*)*, void (%A*)** %fooPtr, align 8
      call void %1(%A* %instanceA)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
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
                fooPtr: FNPTR A.foo := ADR(A.foo);
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

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      ret void
    }

    define i32 @A__foo(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %A.foo = alloca i32, align 4
      store i32 0, i32* %A.foo, align 4
      %A__foo_ret = load i32, i32* %A.foo, align 4
      ret i32 %A__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca i32 (%A*)*, align 8
      %0 = bitcast %A* %instanceA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%A* @__A__init to i8*), i64 ptrtoint (%A* getelementptr (%A, %A* null, i32 1) to i64), i1 false)
      store i32 (%A*)* @A__foo, i32 (%A*)** %fooPtr, align 8
      %1 = load i32 (%A*)*, i32 (%A*)** %fooPtr, align 8
      %fnptr_call = call i32 %1(%A* %instanceA)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
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
                fooPtr: FNPTR A.foo := ADR(A.foo);
                barPtr: FNPTR A.bar := ADR(A.bar);
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

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      ret void
    }

    define void @A__foo(%A* %0, i8* %1) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %foo = alloca i8*, align 8
      store i8* %1, i8** %foo, align 8
      %deref = load i8*, i8** %foo, align 8
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %deref, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    define void @A__bar(%A* %0, i32* %1) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %bar = alloca i32*, align 8
      store i32* %1, i32** %bar, align 8
      %deref = load i32*, i32** %bar, align 8
      store [5 x i32] [i32 1, i32 2, i32 3, i32 4, i32 5], i32* %deref, align 4
      ret void
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca void (%A*, [81 x i8]*)*, align 8
      %barPtr = alloca void (%A*, [5 x i32]*)*, align 8
      %0 = bitcast %A* %instanceA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%A* @__A__init to i8*), i64 ptrtoint (%A* getelementptr (%A, %A* null, i32 1) to i64), i1 false)
      store void (%A*, [81 x i8]*)* bitcast (void (%A*, i8*)* @A__foo to void (%A*, [81 x i8]*)*), void (%A*, [81 x i8]*)** %fooPtr, align 8
      store void (%A*, [5 x i32]*)* bitcast (void (%A*, i32*)* @A__bar to void (%A*, [5 x i32]*)*), void (%A*, [5 x i32]*)** %barPtr, align 8
      %__0 = alloca [81 x i8], align 1
      %1 = bitcast [81 x i8]* %__0 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %2 = load void (%A*, [81 x i8]*)*, void (%A*, [81 x i8]*)** %fooPtr, align 8
      %3 = bitcast [81 x i8]* %__0 to i8*
      call void %2(%A* %instanceA, i8* %3)
      %load___0 = load [81 x i8], [81 x i8]* %__0, align 1
      %__1 = alloca [5 x i32], align 4
      %4 = bitcast [5 x i32]* %__1 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %4, i8 0, i64 ptrtoint ([5 x i32]* getelementptr ([5 x i32], [5 x i32]* null, i32 1) to i64), i1 false)
      %5 = load void (%A*, [5 x i32]*)*, void (%A*, [5 x i32]*)** %barPtr, align 8
      %6 = bitcast [5 x i32]* %__1 to i32*
      call void %5(%A* %instanceA, i32* %6)
      %load___1 = load [5 x i32], [5 x i32]* %__1, align 4
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
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
                fooPtr: FNPTR A.foo := ADR(A.foo);
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

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      ret void
    }

    define i32 @A__foo(%A* %0, i32 %1, [81 x i8]* %2, i32* %3) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %A.foo = alloca i32, align 4
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      %out = alloca [81 x i8]*, align 8
      store [81 x i8]* %2, [81 x i8]** %out, align 8
      %inout = alloca i32*, align 8
      store i32* %3, i32** %inout, align 8
      store i32 0, i32* %A.foo, align 4
      %A__foo_ret = load i32, i32* %A.foo, align 4
      ret i32 %A__foo_ret
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %fooPtr = alloca i32 (%A*, i32, [81 x i8]*, [5 x i32]*)*, align 8
      %localIn = alloca i32, align 4
      %localOut = alloca [81 x i8], align 1
      %localInOut = alloca [5 x i32], align 4
      %0 = bitcast %A* %instanceA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%A* @__A__init to i8*), i64 ptrtoint (%A* getelementptr (%A, %A* null, i32 1) to i64), i1 false)
      store i32 (%A*, i32, [81 x i8]*, [5 x i32]*)* bitcast (i32 (%A*, i32, [81 x i8]*, i32*)* @A__foo to i32 (%A*, i32, [81 x i8]*, [5 x i32]*)*), i32 (%A*, i32, [81 x i8]*, [5 x i32]*)** %fooPtr, align 8
      store i32 0, i32* %localIn, align 4
      %1 = bitcast [81 x i8]* %localOut to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %2 = bitcast [5 x i32]* %localInOut to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %2, i8 0, i64 ptrtoint ([5 x i32]* getelementptr ([5 x i32], [5 x i32]* null, i32 1) to i64), i1 false)
      %3 = load i32 (%A*, i32, [81 x i8]*, [5 x i32]*)*, i32 (%A*, i32, [81 x i8]*, [5 x i32]*)** %fooPtr, align 8
      %4 = bitcast [81 x i8]* %localOut to i8*
      %5 = bitcast [5 x i32]* %localInOut to i32*
      %load_localIn = load i32, i32* %localIn, align 4
      %fnptr_call = call i32 %3(%A* %instanceA, i32 %load_localIn, i8* %4, i32* %5)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    attributes #0 = { argmemonly nofree nounwind willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    "#);
}

#[test]
fn function_block_call_simple() {
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
                bodyPtr: FNPTR A := ADR(A);

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

    %A = type { i32, i16, i32, i64* }

    @__A__init = unnamed_addr constant %A zeroinitializer

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      %local = getelementptr inbounds %A, %A* %0, i32 0, i32 0
      %in = getelementptr inbounds %A, %A* %0, i32 0, i32 1
      %out = getelementptr inbounds %A, %A* %0, i32 0, i32 2
      %inout = getelementptr inbounds %A, %A* %0, i32 0, i32 3
      ret void
    }

    define void @main() {
    entry:
      %instanceA = alloca %A, align 8
      %bodyPtr = alloca void (%A*)*, align 8
      %localIn = alloca i16, align 2
      %localOut = alloca i32, align 4
      %localInout = alloca i64, align 8
      %0 = bitcast %A* %instanceA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%A* @__A__init to i8*), i64 ptrtoint (%A* getelementptr (%A, %A* null, i32 1) to i64), i1 false)
      store void (%A*)* @A, void (%A*)** %bodyPtr, align 8
      store i16 0, i16* %localIn, align 2
      store i32 0, i32* %localOut, align 4
      store i64 0, i64* %localInout, align 8
      %1 = load void (%A*)*, void (%A*)** %bodyPtr, align 8
      %2 = getelementptr inbounds %A, %A* %instanceA, i32 0, i32 1
      %load_localIn = load i16, i16* %localIn, align 2
      store i16 %load_localIn, i16* %2, align 2
      %3 = getelementptr inbounds %A, %A* %instanceA, i32 0, i32 3
      store i64* %localInout, i64** %3, align 8
      call void %1(%A* %instanceA)
      %4 = getelementptr inbounds %A, %A* %instanceA, i32 0, i32 2
      %5 = load i32, i32* %4, align 4
      store i32 %5, i32* %localOut, align 4
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn function_block_call_implicit_arguments() {
    let result = codegen(
        r"
        TYPE Position2D:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            VAR
                localState: DINT := 0;
            END_VAR

            VAR_INPUT
                in1: DINT;
                in2: STRING;
                in3: Position2D;
                in4: ARRAY[1..2] OF STRING;
                in5: ARRAY[1..2] OF Position2D;
            END_VAR

            VAR_OUTPUT
                out1: DINT;
                out2: STRING;
                out3: Position2D;
                out4: ARRAY[1..2] OF STRING;
                out5: ARRAY[1..2] OF Position2D;
            END_VAR

            VAR_IN_OUT
                inout1: DINT;
                inout2: STRING;
                inout3: Position2D;
                inout4: ARRAY[1..2] OF STRING;
                inout5: ARRAY[1..2] OF Position2D;
            END_VAR

            METHOD increaseLocalState
                localState := localState + 1;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                bodyPtr: FNPTR FbA := ADR(FbA);

                localIn1: DINT;
                localIn2: STRING;
                localIn3: Position2D;
                localIn4: ARRAY[1..2] OF STRING;
                localIn5: ARRAY[1..2] OF Position2D;

                localOut1: DINT;
                localOut2: STRING;
                localOut3: Position2D;
                localOut4: ARRAY[1..2] OF STRING;
                localOut5: ARRAY[1..2] OF Position2D;

                localInout1: DINT;
                localInout2: STRING;
                localInout3: Position2D;
                localInout4: ARRAY[1..2] OF STRING;
                localInout5: ARRAY[1..2] OF Position2D;
            END_VAR

            instanceFbA.increaseLocalState();

            bodyPtr^(
                instanceFbA,
                localIn1,
                localIn2,
                localIn3,
                localIn4,
                localIn5,
                localOut1,
                localOut2,
                localOut3,
                localOut4,
                localOut5,
                localInout1,
                localInout2,
                localInout3,
                localInout4,
                localInout5
            );
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FbA = type { i32, i32, [81 x i8], %Position2D, [2 x [81 x i8]], [2 x %Position2D], i32, [81 x i8], %Position2D, [2 x [81 x i8]], [2 x %Position2D], i32*, [81 x i8]*, %Position2D*, [2 x [81 x i8]]*, [2 x %Position2D]* }
    %Position2D = type { i32, i32 }

    @__FbA__init = unnamed_addr constant %FbA zeroinitializer
    @__Position2D__init = unnamed_addr constant %Position2D zeroinitializer

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %localState = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %in1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      %in2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 2
      %in3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 3
      %in4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 4
      %in5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 5
      %out1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 6
      %out2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 7
      %out3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 8
      %out4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 9
      %out5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 10
      %inout1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 11
      %inout2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 12
      %inout3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 13
      %inout4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 14
      %inout5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 15
      ret void
    }

    define void @FbA__increaseLocalState(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %localState = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %in1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      %in2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 2
      %in3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 3
      %in4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 4
      %in5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 5
      %out1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 6
      %out2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 7
      %out3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 8
      %out4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 9
      %out5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 10
      %inout1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 11
      %inout2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 12
      %inout3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 13
      %inout4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 14
      %inout5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 15
      %load_localState = load i32, i32* %localState, align 4
      %tmpVar = add i32 %load_localState, 1
      store i32 %tmpVar, i32* %localState, align 4
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %bodyPtr = alloca void (%FbA*)*, align 8
      %localIn1 = alloca i32, align 4
      %localIn2 = alloca [81 x i8], align 1
      %localIn3 = alloca %Position2D, align 8
      %localIn4 = alloca [2 x [81 x i8]], align 1
      %localIn5 = alloca [2 x %Position2D], align 8
      %localOut1 = alloca i32, align 4
      %localOut2 = alloca [81 x i8], align 1
      %localOut3 = alloca %Position2D, align 8
      %localOut4 = alloca [2 x [81 x i8]], align 1
      %localOut5 = alloca [2 x %Position2D], align 8
      %localInout1 = alloca i32, align 4
      %localInout2 = alloca [81 x i8], align 1
      %localInout3 = alloca %Position2D, align 8
      %localInout4 = alloca [2 x [81 x i8]], align 1
      %localInout5 = alloca [2 x %Position2D], align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      store void (%FbA*)* @FbA, void (%FbA*)** %bodyPtr, align 8
      store i32 0, i32* %localIn1, align 4
      %1 = bitcast [81 x i8]* %localIn2 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %2 = bitcast %Position2D* %localIn3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 1 bitcast (%Position2D* @__Position2D__init to i8*), i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %3 = bitcast [2 x [81 x i8]]* %localIn4 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %3, i8 0, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %4 = bitcast [2 x %Position2D]* %localIn5 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %4, i8 0, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %localOut1, align 4
      %5 = bitcast [81 x i8]* %localOut2 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %5, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %6 = bitcast %Position2D* %localOut3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %6, i8* align 1 bitcast (%Position2D* @__Position2D__init to i8*), i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %7 = bitcast [2 x [81 x i8]]* %localOut4 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %7, i8 0, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %8 = bitcast [2 x %Position2D]* %localOut5 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %8, i8 0, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %localInout1, align 4
      %9 = bitcast [81 x i8]* %localInout2 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %9, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %10 = bitcast %Position2D* %localInout3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %10, i8* align 1 bitcast (%Position2D* @__Position2D__init to i8*), i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %11 = bitcast [2 x [81 x i8]]* %localInout4 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %11, i8 0, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %12 = bitcast [2 x %Position2D]* %localInout5 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %12, i8 0, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      call void @FbA__increaseLocalState(%FbA* %instanceFbA)
      %13 = load void (%FbA*)*, void (%FbA*)** %bodyPtr, align 8
      %14 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 1
      %load_localIn1 = load i32, i32* %localIn1, align 4
      store i32 %load_localIn1, i32* %14, align 4
      %15 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 2
      %16 = bitcast [81 x i8]* %15 to i8*
      %17 = bitcast [81 x i8]* %localIn2 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %16, i8* align 1 %17, i32 80, i1 false)
      %18 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 3
      %19 = bitcast %Position2D* %18 to i8*
      %20 = bitcast %Position2D* %localIn3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %19, i8* align 1 %20, i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %21 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 4
      %22 = bitcast [2 x [81 x i8]]* %21 to i8*
      %23 = bitcast [2 x [81 x i8]]* %localIn4 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %22, i8* align 1 %23, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %24 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 5
      %25 = bitcast [2 x %Position2D]* %24 to i8*
      %26 = bitcast [2 x %Position2D]* %localIn5 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %25, i8* align 1 %26, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      %27 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 11
      store i32* %localInout1, i32** %27, align 8
      %28 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 12
      store [81 x i8]* %localInout2, [81 x i8]** %28, align 8
      %29 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 13
      store %Position2D* %localInout3, %Position2D** %29, align 8
      %30 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 14
      store [2 x [81 x i8]]* %localInout4, [2 x [81 x i8]]** %30, align 8
      %31 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 15
      store [2 x %Position2D]* %localInout5, [2 x %Position2D]** %31, align 8
      call void %13(%FbA* %instanceFbA)
      %32 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 6
      %33 = load i32, i32* %32, align 4
      store i32 %33, i32* %localOut1, align 4
      %34 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 7
      %35 = bitcast [81 x i8]* %localOut2 to i8*
      %36 = bitcast [81 x i8]* %34 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %35, i8* align 1 %36, i32 80, i1 false)
      %37 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 8
      %38 = bitcast %Position2D* %localOut3 to i8*
      %39 = bitcast %Position2D* %37 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %38, i8* align 1 %39, i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %40 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 9
      %41 = bitcast [2 x [81 x i8]]* %localOut4 to i8*
      %42 = bitcast [2 x [81 x i8]]* %40 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %41, i8* align 1 %42, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %43 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 10
      %44 = bitcast [2 x %Position2D]* %localOut5 to i8*
      %45 = bitcast [2 x %Position2D]* %43 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %44, i8* align 1 %45, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    "#);
}

#[test]
fn function_block_call_explicit_arguments() {
    let result = codegen(
        r"
        TYPE Position2D:
            STRUCT
                x: DINT;
                y: DINT;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            VAR
                localState: DINT := 0;
            END_VAR

            VAR_INPUT
                in1: DINT;
                in2: STRING;
                in3: Position2D;
                in4: ARRAY[1..2] OF STRING;
                in5: ARRAY[1..2] OF Position2D;
            END_VAR

            VAR_OUTPUT
                out1: DINT;
                out2: STRING;
                out3: Position2D;
                out4: ARRAY[1..2] OF STRING;
                out5: ARRAY[1..2] OF Position2D;
            END_VAR

            VAR_IN_OUT
                inout1: DINT;
                inout2: STRING;
                inout3: Position2D;
                inout4: ARRAY[1..2] OF STRING;
                inout5: ARRAY[1..2] OF Position2D;
            END_VAR

            METHOD increaseLocalState
                localState := localState + 1;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                bodyPtr: FNPTR FbA := ADR(FbA);

                localIn1: DINT;
                localIn2: STRING;
                localIn3: Position2D;
                localIn4: ARRAY[1..2] OF STRING;
                localIn5: ARRAY[1..2] OF Position2D;

                localOut1: DINT;
                localOut2: STRING;
                localOut3: Position2D;
                localOut4: ARRAY[1..2] OF STRING;
                localOut5: ARRAY[1..2] OF Position2D;

                localInout1: DINT;
                localInout2: STRING;
                localInout3: Position2D;
                localInout4: ARRAY[1..2] OF STRING;
                localInout5: ARRAY[1..2] OF Position2D;
            END_VAR

            instanceFbA.increaseLocalState();

            bodyPtr^(
                instanceFbA,
                inout5 := localInout5,
                in1 := localIn1,
                in2 := localIn2,
                in3 := localIn3,
                in4 := localIn4,
                in5 := localIn5,
                out1 => localOut1,
                out2 => localOut2,
                out3 => localOut3,
                out4 => localOut4,
                out5 => localOut5,
                inout1 := localInout1,
                inout2 := localInout2,
                inout3 := localInout3,
                inout4 := localInout4
            );
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FbA = type { i32, i32, [81 x i8], %Position2D, [2 x [81 x i8]], [2 x %Position2D], i32, [81 x i8], %Position2D, [2 x [81 x i8]], [2 x %Position2D], i32*, [81 x i8]*, %Position2D*, [2 x [81 x i8]]*, [2 x %Position2D]* }
    %Position2D = type { i32, i32 }

    @__FbA__init = unnamed_addr constant %FbA zeroinitializer
    @__Position2D__init = unnamed_addr constant %Position2D zeroinitializer

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %localState = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %in1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      %in2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 2
      %in3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 3
      %in4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 4
      %in5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 5
      %out1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 6
      %out2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 7
      %out3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 8
      %out4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 9
      %out5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 10
      %inout1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 11
      %inout2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 12
      %inout3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 13
      %inout4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 14
      %inout5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 15
      ret void
    }

    define void @FbA__increaseLocalState(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %localState = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %in1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      %in2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 2
      %in3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 3
      %in4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 4
      %in5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 5
      %out1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 6
      %out2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 7
      %out3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 8
      %out4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 9
      %out5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 10
      %inout1 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 11
      %inout2 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 12
      %inout3 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 13
      %inout4 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 14
      %inout5 = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 15
      %load_localState = load i32, i32* %localState, align 4
      %tmpVar = add i32 %load_localState, 1
      store i32 %tmpVar, i32* %localState, align 4
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %bodyPtr = alloca void (%FbA*)*, align 8
      %localIn1 = alloca i32, align 4
      %localIn2 = alloca [81 x i8], align 1
      %localIn3 = alloca %Position2D, align 8
      %localIn4 = alloca [2 x [81 x i8]], align 1
      %localIn5 = alloca [2 x %Position2D], align 8
      %localOut1 = alloca i32, align 4
      %localOut2 = alloca [81 x i8], align 1
      %localOut3 = alloca %Position2D, align 8
      %localOut4 = alloca [2 x [81 x i8]], align 1
      %localOut5 = alloca [2 x %Position2D], align 8
      %localInout1 = alloca i32, align 4
      %localInout2 = alloca [81 x i8], align 1
      %localInout3 = alloca %Position2D, align 8
      %localInout4 = alloca [2 x [81 x i8]], align 1
      %localInout5 = alloca [2 x %Position2D], align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      store void (%FbA*)* @FbA, void (%FbA*)** %bodyPtr, align 8
      store i32 0, i32* %localIn1, align 4
      %1 = bitcast [81 x i8]* %localIn2 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %1, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %2 = bitcast %Position2D* %localIn3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 1 bitcast (%Position2D* @__Position2D__init to i8*), i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %3 = bitcast [2 x [81 x i8]]* %localIn4 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %3, i8 0, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %4 = bitcast [2 x %Position2D]* %localIn5 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %4, i8 0, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %localOut1, align 4
      %5 = bitcast [81 x i8]* %localOut2 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %5, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %6 = bitcast %Position2D* %localOut3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %6, i8* align 1 bitcast (%Position2D* @__Position2D__init to i8*), i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %7 = bitcast [2 x [81 x i8]]* %localOut4 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %7, i8 0, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %8 = bitcast [2 x %Position2D]* %localOut5 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %8, i8 0, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      store i32 0, i32* %localInout1, align 4
      %9 = bitcast [81 x i8]* %localInout2 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %9, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      %10 = bitcast %Position2D* %localInout3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %10, i8* align 1 bitcast (%Position2D* @__Position2D__init to i8*), i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %11 = bitcast [2 x [81 x i8]]* %localInout4 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %11, i8 0, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %12 = bitcast [2 x %Position2D]* %localInout5 to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %12, i8 0, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      call void @FbA__increaseLocalState(%FbA* %instanceFbA)
      %13 = load void (%FbA*)*, void (%FbA*)** %bodyPtr, align 8
      %14 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 15
      store [2 x %Position2D]* %localInout5, [2 x %Position2D]** %14, align 8
      %15 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 1
      %load_localIn1 = load i32, i32* %localIn1, align 4
      store i32 %load_localIn1, i32* %15, align 4
      %16 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 2
      %17 = bitcast [81 x i8]* %16 to i8*
      %18 = bitcast [81 x i8]* %localIn2 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %17, i8* align 1 %18, i32 80, i1 false)
      %19 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 3
      %20 = bitcast %Position2D* %19 to i8*
      %21 = bitcast %Position2D* %localIn3 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %20, i8* align 1 %21, i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %22 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 4
      %23 = bitcast [2 x [81 x i8]]* %22 to i8*
      %24 = bitcast [2 x [81 x i8]]* %localIn4 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %23, i8* align 1 %24, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %25 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 5
      %26 = bitcast [2 x %Position2D]* %25 to i8*
      %27 = bitcast [2 x %Position2D]* %localIn5 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %26, i8* align 1 %27, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      %28 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 11
      store i32* %localInout1, i32** %28, align 8
      %29 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 12
      store [81 x i8]* %localInout2, [81 x i8]** %29, align 8
      %30 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 13
      store %Position2D* %localInout3, %Position2D** %30, align 8
      %31 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 14
      store [2 x [81 x i8]]* %localInout4, [2 x [81 x i8]]** %31, align 8
      call void %13(%FbA* %instanceFbA)
      %32 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 6
      %33 = load i32, i32* %32, align 4
      store i32 %33, i32* %localOut1, align 4
      %34 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 7
      %35 = bitcast [81 x i8]* %localOut2 to i8*
      %36 = bitcast [81 x i8]* %34 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %35, i8* align 1 %36, i32 80, i1 false)
      %37 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 8
      %38 = bitcast %Position2D* %localOut3 to i8*
      %39 = bitcast %Position2D* %37 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %38, i8* align 1 %39, i64 ptrtoint (%Position2D* getelementptr (%Position2D, %Position2D* null, i32 1) to i64), i1 false)
      %40 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 9
      %41 = bitcast [2 x [81 x i8]]* %localOut4 to i8*
      %42 = bitcast [2 x [81 x i8]]* %40 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %41, i8* align 1 %42, i64 ptrtoint ([2 x [81 x i8]]* getelementptr ([2 x [81 x i8]], [2 x [81 x i8]]* null, i32 1) to i64), i1 false)
      %43 = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 10
      %44 = bitcast [2 x %Position2D]* %localOut5 to i8*
      %45 = bitcast [2 x %Position2D]* %43 to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %44, i8* align 1 %45, i64 ptrtoint ([2 x %Position2D]* getelementptr ([2 x %Position2D], [2 x %Position2D]* null, i32 1) to i64), i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #1

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    attributes #1 = { argmemonly nofree nounwind willreturn writeonly }
    "#);
}

#[test]
fn regular_function_block_pointers_are_called_directly() {
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

    define void @A(%A* %0) {
    entry:
      %this = alloca %A*, align 8
      store %A* %0, %A** %this, align 8
      ret void
    }

    define void @main() {
    entry:
      %fooPtr = alloca %A*, align 8
      store %A* null, %A** %fooPtr, align 8
      %deref = load %A*, %A** %fooPtr, align 8
      call void @A(%A* %deref)
      ret void
    }
    "#);
}
