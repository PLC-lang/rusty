// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

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

    filtered_assert_snapshot!(prog);
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

    filtered_assert_snapshot!(prog);
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

    filtered_assert_snapshot!(prog);
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

    filtered_assert_snapshot!(prog);
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

    filtered_assert_snapshot!(prog);
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

    filtered_assert_snapshot!(prog);
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
    filtered_assert_snapshot!(prog);
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

    filtered_assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FOO = type { i8, i8 }

    @__FOO__init = unnamed_addr constant %FOO zeroinitializer

    define void @FOO(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %X = getelementptr inbounds nuw %FOO, ptr %0, i32 0, i32 0
      %Y = getelementptr inbounds nuw %FOO, ptr %0, i32 0, i32 1
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 0, ptr %error_bits, align 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %f, ptr align 1 @__FOO__init, i64 ptrtoint (ptr getelementptr (%FOO, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      %0 = getelementptr inbounds %FOO, ptr %f, i32 0, i32 0
      %load_error_bits = load i8, ptr %error_bits, align 1
      %shift = lshr i8 %load_error_bits, 0
      %1 = and i8 %shift, 1
      store i8 %1, ptr %0, align 1
      call void @FOO(ptr %f)
      %2 = getelementptr inbounds nuw %FOO, ptr %f, i32 0, i32 1
      %3 = load i8, ptr %error_bits, align 1
      %4 = load i8, ptr %2, align 1
      %erase = and i8 %3, -2
      %value = shl i8 %4, 0
      %or = or i8 %erase, %value
      store i8 %or, ptr %error_bits, align 1
      %5 = getelementptr inbounds %FOO, ptr %f, i32 0, i32 0
      %load_error_bits1 = load i8, ptr %error_bits, align 1
      %shift2 = lshr i8 %load_error_bits1, 0
      %6 = and i8 %shift2, 1
      store i8 %6, ptr %5, align 1
      call void @FOO(ptr %f)
      %7 = getelementptr inbounds nuw %FOO, ptr %f, i32 0, i32 1
      %8 = load i8, ptr %error_bits, align 1
      %9 = load i8, ptr %7, align 1
      %erase3 = and i8 %8, -2
      %value4 = shl i8 %9, 0
      %or5 = or i8 %erase3, %value4
      store i8 %or5, ptr %error_bits, align 1
      %10 = getelementptr inbounds %FOO, ptr %f, i32 0, i32 0
      %load_error_bits6 = load i8, ptr %error_bits, align 1
      %shift7 = lshr i8 %load_error_bits6, 0
      %11 = and i8 %shift7, 1
      store i8 %11, ptr %10, align 1
      call void @FOO(ptr %f)
      %12 = getelementptr inbounds nuw %FOO, ptr %f, i32 0, i32 1
      %13 = load i8, ptr %error_bits, align 1
      %14 = load i8, ptr %12, align 1
      %erase8 = and i8 %13, -2
      %value9 = shl i8 %14, 0
      %or10 = or i8 %erase8, %value9
      store i8 %or10, ptr %error_bits, align 1
      %15 = getelementptr inbounds %FOO, ptr %f, i32 0, i32 0
      %load_error_bits11 = load i8, ptr %error_bits, align 1
      %shift12 = lshr i8 %load_error_bits11, 0
      %16 = and i8 %shift12, 1
      store i8 %16, ptr %15, align 1
      call void @FOO(ptr %f)
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    filtered_assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FOO = type { i8 }

    @__FOO__init = unnamed_addr constant %FOO { i8 1 }

    define void @FOO(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %Q = getelementptr inbounds nuw %FOO, ptr %0, i32 0, i32 0
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 -17, ptr %error_bits, align 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %f, ptr align 1 @__FOO__init, i64 ptrtoint (ptr getelementptr (%FOO, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      call void @FOO(ptr %f)
      %0 = getelementptr inbounds nuw %FOO, ptr %f, i32 0, i32 0
      %1 = load i8, ptr %error_bits, align 1
      %2 = load i8, ptr %0, align 1
      %erase = and i8 %1, -17
      %value = shl i8 %2, 4
      %or = or i8 %erase, %value
      store i8 %or, ptr %error_bits, align 1
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    filtered_assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %FOO = type { i8 }

    @__FOO__init = unnamed_addr constant %FOO { i8 1 }

    define void @FOO(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %Q = getelementptr inbounds nuw %FOO, ptr %0, i32 0, i32 0
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 -17, ptr %error_bits, align 1
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %f, ptr align 1 @__FOO__init, i64 ptrtoint (ptr getelementptr (%FOO, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      call void @FOO(ptr %f)
      %0 = getelementptr inbounds nuw %FOO, ptr %f, i32 0, i32 0
      %1 = load i8, ptr %error_bits, align 1
      %2 = load i8, ptr %0, align 1
      %erase = and i8 %1, -17
      %value = shl i8 %2, 4
      %or = or i8 %erase, %value
      store i8 %or, ptr %error_bits, align 1
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
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

    filtered_assert_snapshot!(ir, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"
    target datalayout = "[filtered]"
    target triple = "[filtered]"

    %QUUX = type { i8 }
    %foo_struct = type { %bar_struct }
    %bar_struct = type { i64 }

    @__QUUX__init = unnamed_addr constant %QUUX zeroinitializer
    @__foo_struct__init = unnamed_addr constant %foo_struct zeroinitializer
    @__bar_struct__init = unnamed_addr constant %bar_struct zeroinitializer

    define void @QUUX(ptr %0) {
    entry:
      %this = alloca ptr, align 8
      store ptr %0, ptr %this, align 8
      %Q = getelementptr inbounds nuw %QUUX, ptr %0, i32 0, i32 0
      ret void
    }

    define i32 @main() {
    entry:
      %main = alloca i32, align 4
      %foo = alloca %foo_struct, align 8
      %f = alloca %QUUX, align 8
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %foo, ptr align 1 @__foo_struct__init, i64 ptrtoint (ptr getelementptr (%foo_struct, ptr null, i32 1) to i64), i1 false)
      call void @llvm.memcpy.p0.p0.i64(ptr align 1 %f, ptr align 1 @__QUUX__init, i64 ptrtoint (ptr getelementptr (%QUUX, ptr null, i32 1) to i64), i1 false)
      store i32 0, ptr %main, align 4
      call void @QUUX(ptr %f)
      %bar = getelementptr inbounds nuw %foo_struct, ptr %foo, i32 0, i32 0
      %baz = getelementptr inbounds nuw %bar_struct, ptr %bar, i32 0, i32 0
      %0 = getelementptr inbounds nuw %QUUX, ptr %f, i32 0, i32 0
      %1 = load i64, ptr %baz, align 8
      %2 = load i8, ptr %0, align 1
      %erase = and i64 %1, -281474976710657
      %3 = zext i8 %2 to i64
      %value = shl i64 %3, 48
      %or = or i64 %erase, %value
      store i64 %or, ptr %baz, align 8
      call void @QUUX(ptr %f)
      %bar1 = getelementptr inbounds nuw %foo_struct, ptr %foo, i32 0, i32 0
      %baz2 = getelementptr inbounds nuw %bar_struct, ptr %bar1, i32 0, i32 0
      %4 = getelementptr inbounds nuw %QUUX, ptr %f, i32 0, i32 0
      %5 = load i64, ptr %baz2, align 8
      %6 = load i8, ptr %4, align 1
      %erase3 = and i64 %5, -1125899906842625
      %7 = zext i8 %6 to i64
      %value4 = shl i64 %7, 50
      %or5 = or i64 %erase3, %value4
      store i64 %or5, ptr %baz2, align 8
      %main_ret = load i32, ptr %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
    declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #0

    attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
    "#);
}
