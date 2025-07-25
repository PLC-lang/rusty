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

#[test]
fn user_defined_virtual_table_calls() {
    let result = codegen(
        r"
        // Virtual Table Definitions
        TYPE UserVT_FbA:
            STRUCT
                printNumber: POINTER TO FbA.printNumber := ADR(FbA.printNumber);
            END_STRUCT
        END_TYPE

        TYPE UserVT_FbB:
            STRUCT
                // No override, hence the ADR(FbA.<...>)
                printNumber: POINTER TO FbA.printNumber := ADR(FbA.printNumber);
            END_STRUCT
        END_TYPE

        TYPE UserVT_FbC:
            STRUCT
                // override, hence the ADR(FbC.<...>)
                printNumber: POINTER TO FbA.printNumber := ADR(FbC.printNumber);
            END_STRUCT
        END_TYPE

        // Virtual Table Instances
        VAR_GLOBAL
            UserVT_FbA_instance: UserVT_FbA;
            UserVT_FbB_instance: UserVT_FbB;
            UserVT_FbC_instance: UserVT_FbC;
        END_VAR

        FUNCTION_BLOCK FbA
            VAR
                vt: POINTER TO __VOID;
                localStateA: DINT := 5;
            END_VAR

            METHOD printNumber
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbB EXTENDS FbA
            VAR
                localStateB: DINT := 10;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbC EXTENDS FbA
            VAR
                localStateC: DINT := 15;
            END_VAR

            METHOD printNumber
                VAR_INPUT
                    in: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
                instanceFbB: FbB;
                instanceFbC: FbC;
                refInstanceFbA: POINTER TO FbA;
            END_VAR

            // Virtual Table Initialization
            instanceFbA.vt := ADR(UserVT_FbA_instance);
            instanceFbB.vt := ADR(UserVT_FbB_instance);
            instanceFbC.vt := ADR(UserVT_FbC_instance);

            refInstanceFbA := ADR(instanceFbA);
            UserVT_FbA#(refInstanceFbA^.vt^).printNumber^(refInstanceFbA^, 5);
            
            refInstanceFbA := ADR(instanceFbB);
            UserVT_FbA#(refInstanceFbA^.vt^).printNumber^(refInstanceFbA^, 10);

            refInstanceFbA := ADR(instanceFbC);
            UserVT_FbA#(refInstanceFbA^.vt^).printNumber^(refInstanceFbA^, 15);
        END_FUNCTION
    ",
    );

    // Lots of yada yada, the interesting part happens in the `main` function
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %UserVT_FbA = type { void (%FbA*, i32)* }
    %FbA = type { i32*, i32 }
    %UserVT_FbB = type { void (%FbA*, i32)* }
    %UserVT_FbC = type { void (%FbA*, i32)* }
    %FbB = type { i32 }
    %FbC = type { i32 }

    @UserVT_FbA_instance = global %UserVT_FbA zeroinitializer
    @__UserVT_FbA__init = unnamed_addr constant %UserVT_FbA zeroinitializer
    @__FbA__init = unnamed_addr constant %FbA { i32* null, i32 5 }
    @UserVT_FbB_instance = global %UserVT_FbB zeroinitializer
    @__UserVT_FbB__init = unnamed_addr constant %UserVT_FbB zeroinitializer
    @UserVT_FbC_instance = global %UserVT_FbC zeroinitializer
    @__UserVT_FbC__init = unnamed_addr constant %UserVT_FbC zeroinitializer
    @__FbB__init = unnamed_addr constant %FbB { i32 10 }
    @__FbC__init = unnamed_addr constant %FbC { i32 15 }

    define void @FbA__printNumber(%FbA* %0, i32 %1) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %vt = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %localStateA = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      ret void
    }

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %vt = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %localStateA = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      ret void
    }

    define void @FbB(%FbB* %0) {
    entry:
      %this = alloca %FbB*, align 8
      store %FbB* %0, %FbB** %this, align 8
      %localStateB = getelementptr inbounds %FbB, %FbB* %0, i32 0, i32 0
      ret void
    }

    define void @FbC(%FbC* %0) {
    entry:
      %this = alloca %FbC*, align 8
      store %FbC* %0, %FbC** %this, align 8
      %localStateC = getelementptr inbounds %FbC, %FbC* %0, i32 0, i32 0
      ret void
    }

    define void @FbC__printNumber(%FbC* %0, i32 %1) {
    entry:
      %this = alloca %FbC*, align 8
      store %FbC* %0, %FbC** %this, align 8
      %localStateC = getelementptr inbounds %FbC, %FbC* %0, i32 0, i32 0
      %in = alloca i32, align 4
      store i32 %1, i32* %in, align 4
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %instanceFbB = alloca %FbB, align 8
      %instanceFbC = alloca %FbC, align 8
      %refInstanceFbA = alloca %FbA*, align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      %1 = bitcast %FbB* %instanceFbB to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%FbB* @__FbB__init to i8*), i64 ptrtoint (%FbB* getelementptr (%FbB, %FbB* null, i32 1) to i64), i1 false)
      %2 = bitcast %FbC* %instanceFbC to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %2, i8* align 1 bitcast (%FbC* @__FbC__init to i8*), i64 ptrtoint (%FbC* getelementptr (%FbC, %FbC* null, i32 1) to i64), i1 false)
      store %FbA* null, %FbA** %refInstanceFbA, align 8
      %vt = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 0
      store i32* bitcast (%UserVT_FbA* @UserVT_FbA_instance to i32*), i32** %vt, align 8
      %vt1 = getelementptr inbounds %FbB, %FbB* %instanceFbB, i32 0, i32 0
      store i32* bitcast (%UserVT_FbB* @UserVT_FbB_instance to i32*), i32* %vt1, align 8
      %vt2 = getelementptr inbounds %FbC, %FbC* %instanceFbC, i32 0, i32 0
      store i32* bitcast (%UserVT_FbC* @UserVT_FbC_instance to i32*), i32* %vt2, align 8
      store %FbA* %instanceFbA, %FbA** %refInstanceFbA, align 8
      %deref = load %FbA*, %FbA** %refInstanceFbA, align 8
      %vt3 = getelementptr inbounds %FbA, %FbA* %deref, i32 0, i32 0
      %deref4 = load i32*, i32** %vt3, align 8
      %cast = bitcast i32* %deref4 to %UserVT_FbA*
      %printNumber = getelementptr inbounds %UserVT_FbA, %UserVT_FbA* %cast, i32 0, i32 0
      %3 = load void (%FbA*, i32)*, void (%FbA*, i32)** %printNumber, align 8
      %deref5 = load %FbA*, %FbA** %refInstanceFbA, align 8
      call void %3(%FbA* %deref5, i32 5)
      %4 = bitcast %FbB* %instanceFbB to %FbA*
      store %FbA* %4, %FbA** %refInstanceFbA, align 8
      %deref6 = load %FbA*, %FbA** %refInstanceFbA, align 8
      %vt7 = getelementptr inbounds %FbA, %FbA* %deref6, i32 0, i32 0
      %deref8 = load i32*, i32** %vt7, align 8
      %cast9 = bitcast i32* %deref8 to %UserVT_FbA*
      %printNumber10 = getelementptr inbounds %UserVT_FbA, %UserVT_FbA* %cast9, i32 0, i32 0
      %5 = load void (%FbA*, i32)*, void (%FbA*, i32)** %printNumber10, align 8
      %deref11 = load %FbA*, %FbA** %refInstanceFbA, align 8
      call void %5(%FbA* %deref11, i32 10)
      %6 = bitcast %FbC* %instanceFbC to %FbA*
      store %FbA* %6, %FbA** %refInstanceFbA, align 8
      %deref12 = load %FbA*, %FbA** %refInstanceFbA, align 8
      %vt13 = getelementptr inbounds %FbA, %FbA* %deref12, i32 0, i32 0
      %deref14 = load i32*, i32** %vt13, align 8
      %cast15 = bitcast i32* %deref14 to %UserVT_FbA*
      %printNumber16 = getelementptr inbounds %UserVT_FbA, %UserVT_FbA* %cast15, i32 0, i32 0
      %7 = load void (%FbA*, i32)*, void (%FbA*, i32)** %printNumber16, align 8
      %deref17 = load %FbA*, %FbA** %refInstanceFbA, align 8
      call void %7(%FbA* %deref17, i32 15)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}
