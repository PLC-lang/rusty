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
fn temp_explicit() {
    let ir = codegen(
        r"
        FUNCTION_BLOCK FOO
            VAR_OUTPUT
                Q : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT 
            VAR
                error_bits : BYTE;
                f : FOO;
            END_VAR

            f(Q => error_bits.0);
        END_FUNCTION
        ",
    );

    // TODO: The OR here is wrong, what if foo(Q => bits.%X<index>) where Q is false and the <index> is true;
    //       the bit currently stay at 1 but should be 0
    insta::assert_snapshot!(ir, @r###"
    ; ModuleID = 'main'
    source_filename = "main"

    %FOO = type { i8 }

    @__FOO__init = unnamed_addr constant %FOO zeroinitializer

    define void @FOO(%FOO* %0) section "fn-FOO:v[u8]" {
    entry:
      %Q = getelementptr inbounds %FOO, %FOO* %0, i32 0, i32 0
      ret void
    }

    define i32 @main() section "fn-main:i32" {
    entry:
      %main = alloca i32, align 4
      %error_bits = alloca i8, align 1
      %f = alloca %FOO, align 8
      store i8 0, i8* %error_bits, align 1
      %0 = bitcast %FOO* %f to i8*
      call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %0, i8* align 1 getelementptr inbounds (%FOO, %FOO* @__FOO__init, i32 0, i32 0), i64 ptrtoint (%FOO* getelementptr (%FOO, %FOO* null, i32 1) to i64), i1 false)
      store i32 0, i32* %main, align 4
      call void @FOO(%FOO* %f)
      %aaa = load i8, i8* %error_bits, align 1
      %xxx = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %yyy = load i8, i8* %xxx, align 1
      %1 = shl i8 %yyy, 0
      %2 = or i8 %aaa, %1
      store i8 %2, i8* %error_bits, align 1
      %3 = getelementptr inbounds %FOO, %FOO* %f, i32 0, i32 0
      %main_ret = load i32, i32* %main, align 4
      ret i32 %main_ret
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    "###);
}

#[test]
fn temp_implicit() {
    let ir = codegen(
        r"
        FUNCTION_BLOCK FOO
            VAR_OUTPUT
                Q : BOOL;
            END_VAR
        END_FUNCTION_BLOCK

        FUNCTION main : DINT
            VAR
                error_bits : BYTE;
                f : FOO;
            END_VAR

            f(error_bits.0);
        END_FUNCTION
        ",
    );

    assert_snapshot!(ir, @r"");
}

#[test]
fn temp_output_and_normal_assignments() {
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

    assert_snapshot!(ir, @r"");
}

// TODO: Add correctness tests
#[test]
fn temp_complex_bit_access() {
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
            
            f(Q => foo.bar.baz.%W3.%X2);
        END_FUNCTION
        ",
    );

    assert_snapshot!(ir, @r"");
}
