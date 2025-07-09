use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn function_pointer() {
    let result = codegen(
        r"
        FUNCTION echo : DINT
            VAR_INPUT
                value : INT;
            END_VAR

            echo := value;
        END_FUNCTION

        FUNCTION main
            VAR
                echoPtr : POINTER TO echo;
            END_VAR

            echoPtr := ADR(echo);
            echoPtr^(12345);
        END_FUNCTION
    ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    define i32 @echo(i16 %0) {
    entry:
      %echo = alloca i32, align 4
      %value = alloca i16, align 2
      store i16 %0, i16* %value, align 2
      store i32 0, i32* %echo, align 4
      %load_value = load i16, i16* %value, align 2
      %1 = sext i16 %load_value to i32
      store i32 %1, i32* %echo, align 4
      %echo_ret = load i32, i32* %echo, align 4
      ret i32 %echo_ret
    }

    define void @main() {
    entry:
      %echoPtr = alloca i32 (i16)*, align 8
      store i32 (i16)* null, i32 (i16)** %echoPtr, align 8
      store i32 (i16)* @echo, i32 (i16)** %echoPtr, align 8
      %0 = load i32 (i16)*, i32 (i16)** %echoPtr, align 8
      %call = call i32 %0(i32 12345)
      ret void
    }
    "#);
}

#[test]
fn function_pointer_method() {
    let result = codegen(
        r"
        VAR_GLOBAL
            instanceVTableFbA: VTableFbA := (foo := ADR(FbA.foo));
        END_VAR

        TYPE VTableFbA:
            STRUCT
                foo: POINTER TO FbA.foo;
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK FbA
            VAR
                vtable: POINTER TO VTableFbA := ADR(instanceVTableFbA);
                localVariableInFbA: INT;
            END_VAR

            METHOD foo: INT
                // printf('Hello from FbA::foo$N');
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                instanceFbA: FbA;
            END_VAR

            instanceFbA.vtable^.foo^(instanceFbA);
        END_FUNCTION
        ",
    );

    // XXX: The `__init_globals` is missing here, but we're interested in the derefs here anyways
    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %VTableFbA = type { i16 (%FbA*)* }
    %FbA = type { %VTableFbA*, i16 }

    @instanceVTableFbA = global %VTableFbA zeroinitializer
    @__VTableFbA__init = unnamed_addr constant %VTableFbA zeroinitializer
    @__FbA__init = unnamed_addr constant %FbA zeroinitializer

    define i16 @FbA__foo(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %vtable = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %localVariableInFbA = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      %FbA.foo = alloca i16, align 2
      store i16 0, i16* %FbA.foo, align 2
      %FbA__foo_ret = load i16, i16* %FbA.foo, align 2
      ret i16 %FbA__foo_ret
    }

    define void @FbA(%FbA* %0) {
    entry:
      %this = alloca %FbA*, align 8
      store %FbA* %0, %FbA** %this, align 8
      %vtable = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 0
      %localVariableInFbA = getelementptr inbounds %FbA, %FbA* %0, i32 0, i32 1
      ret void
    }

    define void @main() {
    entry:
      %instanceFbA = alloca %FbA, align 8
      %0 = bitcast %FbA* %instanceFbA to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%FbA* @__FbA__init to i8*), i64 ptrtoint (%FbA* getelementptr (%FbA, %FbA* null, i32 1) to i64), i1 false)
      %vtable = getelementptr inbounds %FbA, %FbA* %instanceFbA, i32 0, i32 0
      %deref = load %VTableFbA*, %VTableFbA** %vtable, align 8
      %foo = getelementptr inbounds %VTableFbA, %VTableFbA* %deref, i32 0, i32 0
      %1 = load i16 (%FbA*)*, i16 (%FbA*)** %foo, align 8
      %call = call i16 %1(%FbA* %instanceFbA)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}
