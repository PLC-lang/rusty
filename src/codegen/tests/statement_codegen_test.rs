use insta::assert_snapshot;

// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn bitaccess_generated_as_rsh_and_trunc_i1() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BOOL;
x : DWORD;
y : DINT;
END_VAR
a := x.2;
a := y.%X4;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn byteaccess_generated_as_rsh_and_trunc_i8() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BYTE;
x : DWORD;
y : DINT;
END_VAR
a := x.%B0;
a := x.%B1;
a := y.%B3;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn wordaccess_generated_as_rsh_and_trunc_i16() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : WORD;
x : DWORD;
y : DINT;
END_VAR
a := x.%W0;
a := x.%W1;
a := y.%W1;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn dwordaccess_generated_as_rsh_and_trunc_i32() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : DWORD;
x : LWORD;
y : LINT;
END_VAR
a := x.%D0;
a := x.%D1;
a := y.%D1;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn nested_bitwise_access() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BOOL;
x : LWORD;
END_VAR
(* Second bit of the second byte of the second word of the second dword of an lword*)
a := x.%D1.%W1.%B1.%X1;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn variable_based_bitwise_access() {
    let result = codegen(
        r#"PROGRAM prg
VAR
a : BOOL;
b : BYTE;
x : INT;
y : INT;
END_VAR
a := x.%Xy;
b := x.%By;
END_PROGRAM
"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn function_result_assignment_on_string() {
    let result = codegen(
        r#"
        @EXTERNAL
        FUNCTION CONCAT : STRING[1024]
        VAR_INPUT a,b : STRING[1024]; END_VAR
        END_FUNCTION

        FUNCTION LIST_ADD : BOOL
        VAR_INPUT
            INS : STRING[1000];
            sx : STRING[1] := ' ';
        END_VAR

        INS := CONCAT(sx, INS);
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn function_result_assignment_on_aliased_string() {
    let result = codegen(
        r#"
        TYPE MyStr : STRING[1000]; END_TYPE
        TYPE LongStr : STRING[1024]; END_TYPE

        @EXTERNAL
        FUNCTION CONCAT : LongStr
        VAR_INPUT a,b : LongStr; END_VAR
        END_FUNCTION

        FUNCTION LIST_ADD : BOOL
        VAR_INPUT
            INS : MyStr;
            sx : STRING[1] := ' ';
        END_VAR

        INS := CONCAT(sx, INS);
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn floating_point_type_casting() {
    let result = codegen(
        r#"
        FUNCTION fn : DINT
            VAR
                a : REAL  :=       7 / 2; // => 3.0 (because we do a integer division first and only then cast the result)
                b : REAL  :=  REAL#7 / 2; // => 3.5 (because we first cast then divide)
                c : REAL  := LREAL#7 / 2; // => 3.5 ^

                d : LREAL :=       7 / 2;  // => 3.0 (because we do a integer division first and only then cast the result)
                e : LREAL :=  REAL#7 / 2;  // => 3.5 (because we first cast then divide)
                f : LREAL := LREAL#7 / 2;  // => 3.5 ^
            END_VAR

            // Same reasoning as above
            a :=       7 / 2;
            b :=  REAL#7 / 2;
            c := LREAL#7 / 2;

            d :=       7 / 2;
            e :=  REAL#7 / 2;
            f := LREAL#7 / 2;
        END_FUNCTION
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn ref_assignment() {
    let result = codegen(
        r#"
        FUNCTION main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a REF= b;
        END_PROGRAM
        "#,
    );

    insta::assert_snapshot!(result, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    define void @main() {
    entry:
      %a = alloca i32*, align 8
      %b = alloca i32, align 4
      store i32* null, i32** %a, align 8
      store i32 0, i32* %b, align 4
      store i32* %b, i32** %a, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "###);
}

#[test]
fn reference_to_assignment() {
    let auto_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REFERENCE TO DINT;
            END_VAR
            a := 5;
        END_FUNCTION
        "#,
    );

    let manual_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REF_TO DINT;
            END_VAR
            a^ := 5;
        END_FUNCTION
        "#,
    );

    // We want to assert that `a := 5` and `a^ := 5` yield identical IR
    assert_eq!(auto_deref, manual_deref);

    insta::assert_snapshot!(auto_deref, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    define void @main() {
    entry:
      %a = alloca i32*, align 8
      store i32* null, i32** %a, align 8
      %deref = load i32*, i32** %a, align 8
      store i32 5, i32* %deref, align 4
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "###);
}

#[test]
fn reference_to_string_assignment() {
    let auto_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REFERENCE TO STRING;
            END_VAR

            a := 'hello';
        END_FUNCTION
        "#,
    );

    let manual_deref = codegen(
        r#"
        FUNCTION main
            VAR
                a : REF_TO STRING;
            END_VAR
            a^ := 'hello';
        END_FUNCTION
        "#,
    );

    // We want to assert that `a := 'hello'` and `a^ := 'hello'` yield identical IR
    assert_eq!(auto_deref, manual_deref);

    insta::assert_snapshot!(auto_deref, @r###"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    @utf08_literal_0 = private unnamed_addr constant [6 x i8] c"hello\00"

    define void @main() {
    entry:
      %a = alloca [81 x i8]*, align 8
      store [81 x i8]* null, [81 x i8]** %a, align 8
      %deref = load [81 x i8]*, [81 x i8]** %a, align 8
      %0 = bitcast [81 x i8]* %deref to i8*
      call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 1 %0, i8* align 1 getelementptr inbounds ([6 x i8], [6 x i8]* @utf08_literal_0, i32 0, i32 0), i32 6, i1 false)
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn
    declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "###);
}

#[test]
fn local_alias() {
    let content = codegen(
        r#"
        FUNCTION main
            VAR
                foo AT bar : DINT;
                bar : DINT;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(content, @r#"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    define void @main() {
    entry:
      %foo = alloca i32*, align 8
      %bar = alloca i32, align 4
      store i32* null, i32** %foo, align 8
      store i32 0, i32* %bar, align 4
      store i32* %bar, i32** %foo, align 8
      ret void
    }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "#);
}

#[test]
fn local_string_alias() {
    let content = codegen(
        r#"
        FUNCTION main
            VAR
                foo AT bar : STRING;
                bar : STRING;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(content, @r##"
    ; ModuleID = '<internal>'
    source_filename = "<internal>"

    define void @main() {
    entry:
      %foo = alloca [81 x i8]*, align 8
      %bar = alloca [81 x i8], align 1
      store [81 x i8]* null, [81 x i8]** %foo, align 8
      %0 = bitcast [81 x i8]* %bar to i8*
      call void @llvm.memset.p0i8.i64(i8* align 1 %0, i8 0, i64 ptrtoint ([81 x i8]* getelementptr ([81 x i8], [81 x i8]* null, i32 1) to i64), i1 false)
      store [81 x i8]* %bar, [81 x i8]** %foo, align 8
      ret void
    }

    ; Function Attrs: argmemonly nofree nounwind willreturn writeonly
    declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #0

    attributes #0 = { argmemonly nofree nounwind willreturn writeonly }
    ; ModuleID = '__init___testproject'
    source_filename = "__init___testproject"

    @llvm.global_ctors = appending global [1 x { i32, void ()*, i8* }] [{ i32, void ()*, i8* } { i32 0, void ()* @__init___testproject, i8* null }]

    define void @__init___testproject() {
    entry:
      ret void
    }
    "##);
}

#[test]
#[ignore = "stack overflow regression with 28b6b41"]
fn local_struct_alias() {
    let content = codegen(
        r#"
        TYPE Node : STRUCT
            id      : DINT;
            child   : REF_TO Node;
            parent  : REF_TO Node;
        END_STRUCT END_TYPE

        FUNCTION main
            VAR
                foo AT bar : Node;
                bar : Node;
            END_VAR
        END_FUNCTION
        "#,
    );

    assert_snapshot!(content, @r"");
}

#[test]
#[ignore = "REF(x) initializers where x is a member variable are not yet implemented (https://github.com/PLC-lang/rusty/issues/1286)"]
fn stateful_local() {
    let content = codegen(
        r#"
        FUNCTION_BLOCK foo
            VAR
                foo AT bar : STRING;
                bar : STRING;
            END_VAR
        END_FUNCTION_BLOCK
        "#,
    );

    assert_snapshot!(content, @r"");
}
