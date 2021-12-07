// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;
use pretty_assertions::assert_eq;

use super::generate_program_boiler_plate;

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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1", "a"), ("i32", "x"), ("i32", "y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x, align 4
  %shift = lshr i32 %load_x, 2
  %1 = trunc i32 %shift to i1
  store i1 %1, i1* %a, align 1
  %load_y = load i32, i32* %y, align 4
  %shift1 = ashr i32 %load_y, 4
  %2 = trunc i32 %shift1 to i1
  store i1 %2, i1* %a, align 1
  ret void
"#,
    );

    assert_eq!(result, expected);
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i8", "a"), ("i32", "x"), ("i32", "y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x, align 4
  %shift = lshr i32 %load_x, 0
  %1 = trunc i32 %shift to i8
  store i8 %1, i8* %a, align 1
  %load_x1 = load i32, i32* %x, align 4
  %shift2 = lshr i32 %load_x1, 8
  %2 = trunc i32 %shift2 to i8
  store i8 %2, i8* %a, align 1
  %load_y = load i32, i32* %y, align 4
  %shift3 = ashr i32 %load_y, 24
  %3 = trunc i32 %shift3 to i8
  store i8 %3, i8* %a, align 1
  ret void
"#,
    );

    assert_eq!(result, expected);
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i16", "a"), ("i32", "x"), ("i32", "y")],
        "void",
        "",
        "",
        r#"%load_x = load i32, i32* %x, align 4
  %shift = lshr i32 %load_x, 0
  %1 = trunc i32 %shift to i16
  store i16 %1, i16* %a, align 2
  %load_x1 = load i32, i32* %x, align 4
  %shift2 = lshr i32 %load_x1, 16
  %2 = trunc i32 %shift2 to i16
  store i16 %2, i16* %a, align 2
  %load_y = load i32, i32* %y, align 4
  %shift3 = ashr i32 %load_y, 16
  %3 = trunc i32 %shift3 to i16
  store i16 %3, i16* %a, align 2
  ret void
"#,
    );

    assert_eq!(result, expected);
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i32", "a"), ("i64", "x"), ("i64", "y")],
        "void",
        "",
        "",
        r#"%load_x = load i64, i64* %x, align 4
  %shift = lshr i64 %load_x, 0
  %1 = trunc i64 %shift to i32
  store i32 %1, i32* %a, align 4
  %load_x1 = load i64, i64* %x, align 4
  %shift2 = lshr i64 %load_x1, 32
  %2 = trunc i64 %shift2 to i32
  store i32 %2, i32* %a, align 4
  %load_y = load i64, i64* %y, align 4
  %shift3 = ashr i64 %load_y, 32
  %3 = trunc i64 %shift3 to i32
  store i32 %3, i32* %a, align 4
  ret void
"#,
    );

    assert_eq!(result, expected);
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1", "a"), ("i64", "x")],
        "void",
        "",
        "",
        r#"%load_x = load i64, i64* %x, align 4
  %shift = lshr i64 %load_x, 32
  %1 = trunc i64 %shift to i32
  %shift1 = lshr i32 %1, 16
  %2 = trunc i32 %shift1 to i16
  %shift2 = lshr i16 %2, 8
  %3 = trunc i16 %shift2 to i8
  %shift3 = lshr i8 %3, 1
  %4 = trunc i8 %shift3 to i1
  store i1 %4, i1* %a, align 1
  ret void
"#,
    );

    assert_eq!(result, expected);
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
    let expected = generate_program_boiler_plate(
        "prg",
        &[("i1", "a"), ("i8", "b"), ("i16", "x"), ("i16", "y")],
        "void",
        "",
        "",
        r#"%load_x = load i16, i16* %x, align 2
  %load_y = load i16, i16* %y, align 2
  %shift = ashr i16 %load_x, %load_y
  %1 = trunc i16 %shift to i1
  store i1 %1, i1* %a, align 1
  %load_x1 = load i16, i16* %x, align 2
  %load_y2 = load i16, i16* %y, align 2
  %2 = mul i16 %load_y2, 8
  %shift3 = ashr i16 %load_x1, %2
  %3 = trunc i16 %shift3 to i8
  store i8 %3, i8* %b, align 1
  ret void
"#,
    );

    assert_eq!(result, expected);
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
