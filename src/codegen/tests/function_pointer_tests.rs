use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn function_pointer_method_no_parameters() {
    let result = codegen(
        r"
        FUNCTION_BLOCK FbA
            METHOD foo
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                fnPtr: POINTER TO FbA.foo := ADR(FbA.foo);
            END_VAR

            fnPtr^(instanceFbA);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FbA = type {}

    @__FbA__init = unnamed_addr constant %FbA zeroinitializer

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      ret void
    }

    define void @FbA__foo(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %fnPtr = alloca void (%FbA*)*, align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      store void (%FbA*)* @FbA__foo, void (%FbA*)** %fnPtr, align 8
      %1 = load void (%FbA*)*, void (%FbA*)** %fnPtr, align 8
      call void %1(%FbA* %instanceFbA)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn function_pointer_method_with_input_output_inout() {
    let result = codegen(
        r"
        FUNCTION_BLOCK FbA
            METHOD foo
                VAR_INPUT
                    in: INT;
                END_VAR

                VAR_OUTPUT
                    out: INT;
                END_VAR

                VAR_IN_OUT
                    inout: INT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                localInVariable: INT;
                localOutVariable: INT;
                localInOutVariable: INT;

                fnPtr: POINTER TO FbA.foo := ADR(FbA.foo);
            END_VAR

            fnPtr^(instanceFbA, 12345, localOutVariable, localInOutVariable);
            fnPtr^(instanceFbA, localInVariable, localOutVariable, localInOutVariable);
            fnPtr^(instanceFbA, in := localInVariable, out => localOutVariable, inout := localInOutVariable);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FbA = type {}

    @__FbA__init = unnamed_addr constant %FbA zeroinitializer

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      ret void
    }

    define void @FbA__foo(%FbA* %0, i16 %1, i16* %2, i16* %3) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %in = alloca i16, align 2
      store i16 %1, i16* %in, align 2
      %out = alloca i16*, align 8
      store i16* %2, i16** %out, align 8
      %inout = alloca i16*, align 8
      store i16* %3, i16** %inout, align 8
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %localInVariable = alloca i16, align 2
      %localOutVariable = alloca i16, align 2
      %localInOutVariable = alloca i16, align 2
      %fnPtr = alloca void (%FbA*, i16, i16*, i16*)*, align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      store i16 0, i16* %localInVariable, align 2
      store i16 0, i16* %localOutVariable, align 2
      store i16 0, i16* %localInOutVariable, align 2
      store void (%FbA*, i16, i16*, i16*)* @FbA__foo, void (%FbA*, i16, i16*, i16*)** %fnPtr, align 8
      %1 = load void (%FbA*, i16, i16*, i16*)*, void (%FbA*, i16, i16*, i16*)** %fnPtr, align 8
      call void %1(%FbA* %instanceFbA, i16 12345, i16* %localOutVariable, i16* %localInOutVariable)
      %2 = load void (%FbA*, i16, i16*, i16*)*, void (%FbA*, i16, i16*, i16*)** %fnPtr, align 8
      %load_localInVariable = load i16, i16* %localInVariable, align 2
      call void %2(%FbA* %instanceFbA, i16 %load_localInVariable, i16* %localOutVariable, i16* %localInOutVariable)
      %3 = load void (%FbA*, i16, i16*, i16*)*, void (%FbA*, i16, i16*, i16*)** %fnPtr, align 8
      %load_localInVariable1 = load i16, i16* %localInVariable, align 2
      call void %3(%FbA* %instanceFbA, i16 %load_localInVariable1, i16* %localOutVariable, i16* %localInOutVariable)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn function_pointer_method_with_input_output_inout_shifted_position_to_right_by_one() {
    let result = codegen(
        r"
        FUNCTION_BLOCK FbA
            METHOD foo
                VAR_INPUT
                    in: INT;
                END_VAR

                VAR_OUTPUT
                    out: INT;
                END_VAR

                VAR_IN_OUT
                    inout: INT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                localInVariable: INT;
                localOutVariable: INT;
                localInOutVariable: INT;

                fnPtr: POINTER TO FbA.foo := ADR(FbA.foo);
            END_VAR

            // The order of the parameters is shifted to the right by one position, in circular fashion.
            // Codegen MUST pass the arguments to the function pointer call correctly however, i.e. localInVariable must still be at the first position (after the instance)
            fnPtr^(instanceFbA, inout := localInOutVariable, in := localInVariable, out => localOutVariable);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FbA = type {}

    @__FbA__init = unnamed_addr constant %FbA zeroinitializer

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      ret void
    }

    define void @FbA__foo(%FbA* %0, i16 %1, i16* %2, i16* %3) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %in = alloca i16, align 2
      store i16 %1, i16* %in, align 2
      %out = alloca i16*, align 8
      store i16* %2, i16** %out, align 8
      %inout = alloca i16*, align 8
      store i16* %3, i16** %inout, align 8
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %localInVariable = alloca i16, align 2
      %localOutVariable = alloca i16, align 2
      %localInOutVariable = alloca i16, align 2
      %fnPtr = alloca void (%FbA*, i16, i16*, i16*)*, align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      store i16 0, i16* %localInVariable, align 2
      store i16 0, i16* %localOutVariable, align 2
      store i16 0, i16* %localInOutVariable, align 2
      store void (%FbA*, i16, i16*, i16*)* @FbA__foo, void (%FbA*, i16, i16*, i16*)** %fnPtr, align 8
      %1 = load void (%FbA*, i16, i16*, i16*)*, void (%FbA*, i16, i16*, i16*)** %fnPtr, align 8
      %load_localInVariable = load i16, i16* %localInVariable, align 2
      call void %1(%FbA* %instanceFbA, i16 %load_localInVariable, i16* %localOutVariable, i16* %localInOutVariable)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn void_pointer_casting() {
    let result = codegen(
        r"
        TYPE UserDefinedVirtualTable:
            STRUCT
                methodPtr: POINTER TO FbA.foo := ADR(FbA.foo);
            END_STRUCT
        END_TYPE

        VAR_GLOBAL
            userDefinedVirtualTableInstance: UserDefinedVirtualTable;
        END_VAR

        FUNCTION_BLOCK FbA
            VAR
                vt: POINTER TO __VOID := ADR(userDefinedVirtualTableInstance);
            END_VAR

            METHOD foo: DINT
                VAR_INPUT
                    in: INT;
                END_VAR

                VAR_OUTPUT
                    out: INT;
                END_VAR

                VAR_IN_OUT
                    inout: INT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                refInstanceFbA: POINTER TO FbA;
                localIn: INT := 123;
                localOut: INT := 456;
                localInOut: INT := 789;
            END_VAR

            refInstanceFbA := ADR(instanceFbA);
            UserDefinedVirtualTable#(refInstanceFbA^.vt^).methodPtr^(refInstanceFbA^, localIn, localOut, localInOut);
        END_FUNCTION
        ",
    );

    // Lots of yada yada, the `bitcast ... to %UserDefinedVirtualTable` is the interesting part though
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %UserDefinedVirtualTable = type { i32 (%FbA*, i16, i16*, i16*)* }
    %FbA = type { i32* }

    @userDefinedVirtualTableInstance = global %UserDefinedVirtualTable zeroinitializer
    @__UserDefinedVirtualTable__init = unnamed_addr constant %UserDefinedVirtualTable zeroinitializer
    @__FbA__init = unnamed_addr constant %FbA zeroinitializer

    define i32 @FbA__foo(%FbA* %0, i16 %1, i16* %2, i16* %3) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %vt = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %FbA.foo = alloca i32, align 4
      %in = alloca i16, align 2
      store i16 %1, i16* %in, align 2
      %out = alloca i16*, align 8
      store i16* %2, i16** %out, align 8
      %inout = alloca i16*, align 8
      store i16* %3, i16** %inout, align 8
      store i32 0, i32* %FbA.foo, align 4
      %FbA__foo_ret = load i32, i32* %FbA.foo, align 4
      ret i32 %FbA__foo_ret
    }

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %vt = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %refInstanceFbA = alloca %FbA*, align 8
      %localIn = alloca i16, align 2
      %localOut = alloca i16, align 2
      %localInOut = alloca i16, align 2
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      store %FbA* null, %FbA** %refInstanceFbA, align 8
      store i16 123, i16* %localIn, align 2
      store i16 456, i16* %localOut, align 2
      store i16 789, i16* %localInOut, align 2
      store %FbA* %instanceFbA, %FbA** %refInstanceFbA, align 8
      %deref = load %FbA*, %FbA** %refInstanceFbA, align 8
      %vt = getelementptr inbounds %FbA, %FbA* %deref, i32 0, i32 0
      %deref1 = load i32*, i32** %vt, align 8
      %cast = bitcast i32* %deref1 to %UserDefinedVirtualTable*
      %methodPtr = getelementptr inbounds %UserDefinedVirtualTable, %UserDefinedVirtualTable* %cast, i32 0, i32 0
      %1 = load i32 (%FbA*, i16, i16*, i16*)*, i32 (%FbA*, i16, i16*, i16*)** %methodPtr, align 8
      %load_localIn = load i16, i16* %localIn, align 2
      %deref2 = load %FbA*, %FbA** %refInstanceFbA, align 8
      %call = call i32 %1(%FbA* %deref2, i16 %load_localIn, i16* %localOut, i16* %localInOut)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}
