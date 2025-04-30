use insta::assert_snapshot;
// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn bitaccess_assignment() {
    let prog = codegen(
        "
    FUNCTION main : INT
    VAR
        a : BYTE;
        b : INT := 1;
    END_VAR
    a.1 := TRUE;
    a.%X2 := FALSE;
    a.%Xb := FALSE;
    END_FUNCTION",
    );

    insta::assert_snapshot!(prog);
}

#[test]
fn byteaccess_assignment() {
    let prog = codegen(
        "
    FUNCTION main : INT
    VAR
        b : WORD := 0;
    END_VAR
    b.%B0 := 2;
    END_FUNCTION",
    );

    insta::assert_snapshot!(prog);
}

#[test]
fn wordaccess_assignment() {
    let prog = codegen(
        "
    FUNCTION main : INT
    VAR
        c : DWORD := 0;
    END_VAR
    c.%W0 := 256;
    END_FUNCTION",
    );

    insta::assert_snapshot!(prog);
}

#[test]
fn dwordaccess_assignment() {
    let prog = codegen(
        "
    FUNCTION main : INT
    VAR
        d : LWORD := 0;
    END_VAR
    d.%D0 := 16#AB_CD_EF;
    END_FUNCTION",
    );

    insta::assert_snapshot!(prog);
}

#[test]
fn lwordaccess_assignment() {
    let prog = codegen(
        "
    FUNCTION main : INT
    VAR
        d : LWORD := 0;
    END_VAR
    d.%L1 := 16#AB_CD_EF;
    END_FUNCTION",
    );

    insta::assert_snapshot!(prog);
}

#[test]
fn chained_bit_assignment() {
    let prog = codegen(
        "
    FUNCTION main : INT
    VAR
        d : LWORD := 0;
    END_VAR
    d.%D1.%X1 := TRUE;
    END_FUNCTION",
    );

    insta::assert_snapshot!(prog);
}

#[test]
fn qualified_reference_assignment() {
    let prog = codegen(
        "
        TYPE myStruct : STRUCT x : BYTE := 1; END_STRUCT END_TYPE

        FUNCTION main : INT
        VAR
            str : myStruct;
        END_VAR
        str.x.%X0 := FALSE;
        str.x.%X1 := TRUE;
        END_FUNCTION

        ",
    );
    insta::assert_snapshot!(prog);
}

#[test]
fn direct_acess_in_output_assignment_implicit_explicit_and_mixed() {
    let ir = codegen(
        r"
        FUNCTION_BLOCK FOO
            VAR_INPUT
                X : BOOL;
            END_VAR
            VAR_OUTPUT
                Y : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                error_bits : BYTE;
                f : FOO;
            END_VAR

            f(X := error_bits.0, Y => error_bits.0);
            f(Y => error_bits.0, x := error_bits.0);
            f(error_bits.0, error_bits.0);
            f(X := error_bits.0, Y =>);
        END_FUNCTION
        ",
    );

    assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %FOO = type { i8, i8 }

    @__FOO__init = unnamed_addr constant %FOO zeroinitializer

    define void @FOO(%FOO* %0) {
    entry:
      %X = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 0
      %Y = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 1
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 0, i8* %error_bits, align 1
      %0 = bitcast %FOO* %f to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%FOO, %FOO* @__FOO__init, i32 0, i32 0), i64 ptrtoint (%FOO* getelementptr (%FOO, %FOO* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      %1 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %load_error_bits = load i8, i8* %error_bits, align 1
      %shift = lshr i8 %load_error_bits, 0
      %2 = and i8 %shift, 1
      store i8 %2, i8* %1, align 1
      call void @FOO(%FOO* %f)
      %3 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 1
      %4 = load i8, i8* %error_bits, align 1
      %5 = load i8, i8* %3, align 1
      %erase = and i8 %4, -2
      %value = shl i8 %5, 0
      %or = or i8 %erase, %value
      store i8 %or, i8* %error_bits, align 1
      %6 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %load_error_bits1 = load i8, i8* %error_bits, align 1
      %shift2 = lshr i8 %load_error_bits1, 0
      %7 = and i8 %shift2, 1
      store i8 %7, i8* %6, align 1
      call void @FOO(%FOO* %f)
      %8 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 1
      %9 = load i8, i8* %error_bits, align 1
      %10 = load i8, i8* %8, align 1
      %erase3 = and i8 %9, -2
      %value4 = shl i8 %10, 0
      %or5 = or i8 %erase3, %value4
      store i8 %or5, i8* %error_bits, align 1
      %11 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %load_error_bits6 = load i8, i8* %error_bits, align 1
      %shift7 = lshr i8 %load_error_bits6, 0
      %12 = and i8 %shift7, 1
      store i8 %12, i8* %11, align 1
      call void @FOO(%FOO* %f)
      %13 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 1
      %14 = load i8, i8* %error_bits, align 1
      %15 = load i8, i8* %13, align 1
      %erase8 = and i8 %14, -2
      %value9 = shl i8 %15, 0
      %or10 = or i8 %erase8, %value9
      store i8 %or10, i8* %error_bits, align 1
      %16 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %load_error_bits11 = load i8, i8* %error_bits, align 1
      %shift12 = lshr i8 %load_error_bits11, 0
      %17 = and i8 %shift12, 1
      store i8 %17, i8* %16, align 1
      call void @FOO(%FOO* %f)
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn direct_acess_in_output_assignment_with_simple_expression() {
    let ir = codegen(
        r"
        FUNCTION_BLOCK FOO
            VAR_OUTPUT
                Q : BOOL := TRUE;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                error_bits : BYTE := 2#1110_1111;
                f : FOO;
            END_VAR

            f(Q => error_bits.4);
        END_FUNCTION
        ",
    );

    assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %FOO = type { i8 }

    @__FOO__init = unnamed_addr constant %FOO { i8 1 }

    define void @FOO(%FOO* %0) {
    entry:
      %Q = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 0
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 -17, i8* %error_bits, align 1
      %0 = bitcast %FOO* %f to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%FOO, %FOO* @__FOO__init, i32 0, i32 0), i64 ptrtoint (%FOO* getelementptr (%FOO, %FOO* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      call void @FOO(%FOO* %f)
      %1 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %2 = load i8, i8* %error_bits, align 1
      %3 = load i8, i8* %1, align 1
      %erase = and i8 %2, -17
      %value = shl i8 %3, 4
      %or = or i8 %erase, %value
      store i8 %or, i8* %error_bits, align 1
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn direct_acess_in_output_assignment_with_simple_expression_implicit() {
    let ir = codegen(
        r"
        FUNCTION_BLOCK FOO
            VAR_OUTPUT
                Q : BOOL := TRUE;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                error_bits : BYTE := 2#1110_1111;
                f : FOO;
            END_VAR

            f(error_bits.4);
        END_FUNCTION
        ",
    );

    assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %FOO = type { i8 }

    @__FOO__init = unnamed_addr constant %FOO { i8 1 }

    define void @FOO(%FOO* %0) {
    entry:
      %Q = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 0
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 -17, i8* %error_bits, align 1
      %0 = bitcast %FOO* %f to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%FOO, %FOO* @__FOO__init, i32 0, i32 0), i64 ptrtoint (%FOO* getelementptr (%FOO, %FOO* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      call void @FOO(%FOO* %f)
      %1 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %2 = load i8, i8* %error_bits, align 1
      %3 = load i8, i8* %1, align 1
      %erase = and i8 %2, -17
      %value = shl i8 %3, 4
      %or = or i8 %erase, %value
      store i8 %or, i8* %error_bits, align 1
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}

#[test]
fn direct_acess_in_output_assignment_with_complexe_expression() {
    let ir = codegen(
        r"
        TYPE foo_struct : STRUCT
            bar : bar_struct;
        END_STRUCT END_TYPE
        
        TYPE bar_struct : STRUCT
            baz : LWORD;
        END_STRUCT END_TYPE

        FUNCTION_BLOCK QUUX
            VAR_OUTPUT
                Q : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                foo : foo_struct;
                f : QUUX;
            END_VAR
            
            f(Q => foo.bar.baz.%W3);
            f(Q => foo.bar.baz.%W3.%B0.%X2);
        END_FUNCTION
        ",
    );

    assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
    target triple = "x86_64-pc-linux-gnu"

    %QUUX = type { i8 }
    %foo_struct = type { %bar_struct }
    %bar_struct = type { i64 }

    @__QUUX__init = unnamed_addr constant %QUUX zeroinitializer
    @__foo_struct__init = unnamed_addr constant %foo_struct zeroinitializer
    @__bar_struct__init = unnamed_addr constant %bar_struct zeroinitializer

    define void @QUUX(%QUUX* %0) {
    entry:
      %Q = getelementptr inbounds %QUUX, %QUUX* %0, i32 0, i32 0
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %foo = alloca %foo_struct, align 8
      %f = alloca %QUUX, align 8
      %0 = bitcast %foo_struct* %foo to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 bitcast (%foo_struct* @__foo_struct__init to i8*), i64 ptrtoint (%foo_struct* getelementptr (%foo_struct, %foo_struct* null, i32 1) to i64), i1 false)
      %1 = bitcast %QUUX* %f to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %1, i8* align 1 getelementptr inbounds (%QUUX, %QUUX* @__QUUX__init, i32 0, i32 0), i64 ptrtoint (%QUUX* getelementptr (%QUUX, %QUUX* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      call void @QUUX(%QUUX* %f)
      %bar = getelementptr inbounds %foo_struct, %foo_struct* %foo, i32 0, i32 0
      %baz = getelementptr inbounds %bar_struct, %bar_struct* %bar, i32 0, i32 0
      %2 = getelementptr inbounds %QUUX, %QUUX* %f, i32 0, i32 0
      %3 = load i64, i64* %baz, align 8
      %4 = load i8, i8* %2, align 1
      %erase = and i64 %3, -281474976710657
      %5 = zext i8 %4 to i64
      %value = shl i64 %5, 48
      %or = or i64 %erase, %value
      store i64 %or, i64* %baz, align 8
      call void @QUUX(%QUUX* %f)
      %bar1 = getelementptr inbounds %foo_struct, %foo_struct* %foo, i32 0, i32 0
      %baz2 = getelementptr inbounds %bar_struct, %bar_struct* %bar1, i32 0, i32 0
      %6 = getelementptr inbounds %QUUX, %QUUX* %f, i32 0, i32 0
      %7 = load i64, i64* %baz2, align 8
      %8 = load i8, i8* %6, align 1
      %erase3 = and i64 %7, -1125899906842625
      %9 = zext i8 %8 to i64
      %value4 = shl i64 %9, 50
      %or5 = or i64 %erase3, %value4
      store i64 %or5, i64* %baz2, align 8
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "#);
}
