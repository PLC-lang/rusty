use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn function_pointer_simple() {
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
                echoPtr : REF_TO echo;
            END_VAR

            echoPtr := REF(echo);
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
fn function_pointer_simple_method() {
    let result = codegen(
        r"
        TYPE VTable:
            STRUCT
                fbEcho : REF_TO fb.fbEcho := REF(fb.fbEcho);
            END_STRUCT
        END_TYPE

        FUNCTION_BLOCK fb
            METHOD fbEcho : DINT
                VAR_INPUT
                    value : INT;
                END_VAR

                fbEcho := value;
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION main
            VAR
                vt: VTable;
                instance : fb;
            END_VAR

            vt.fbEcho^(instance, INT#5);
        END_FUNCTION
        ",
    );

    filtered_assert_snapshot!(result, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %fb = type {}
    %VTable = type { i32 (%fb*, i16)* }

    @__fb__init = unnamed_addr constant %fb zeroinitializer
    @__VTable__init = unnamed_addr constant %VTable zeroinitializer

    define void @fb(%fb* %0) {
    entry:
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      ret void
    }

    define i32 @fb__fbEcho(%fb* %0, i16 %1) {
    entry:
      %this = alloca %fb*, align 8
      store %fb* %0, %fb** %this, align 8
      %fb.fbEcho = alloca i32, align 4
      %value = alloca i16, align 2
      store i16 %1, i16* %value, align 2
      store i32 0, i32* %fb.fbEcho, align 4
      %load_value = load i16, i16* %value, align 2
      %2 = sext i16 %load_value to i32
      store i32 %2, i32* %fb.fbEcho, align 4
      %fb__fbEcho_ret = load i32, i32* %fb.fbEcho, align 4
      ret i32 %fb__fbEcho_ret
    }

    define void @main() {
    entry:
      %vt = alloca %VTable, align 8
      %instance = alloca %fb, align 8
      %0 = bitcast %VTable* %vt to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%VTable* @__VTable__init to i8*), i64 ptrtoint (%VTable* getelementptr (%VTable, %VTable* null, i32 1) to i64), i1 false)
      %1 = bitcast %fb* %instance to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 bitcast (%fb* @__fb__init to i8*), i64 ptrtoint (%fb* getelementptr (%fb, %fb* null, i32 1) to i64), i1 false)
      %fbEcho = getelementptr inbounds %VTable, %VTable* %vt, i32 0, i32 0
      %2 = load i32 (%fb*, i16)*, i32 (%fb*, i16)** %fbEcho, align 8
      %call = call i32 %2(%fb* %instance, i16 5)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}
