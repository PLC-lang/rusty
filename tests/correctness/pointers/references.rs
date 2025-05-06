// Copyright (c) 2021 Daniel Schwenniger

use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTest {
    __vtable: usize,
    reference: usize,
    p: usize,
    in_out1: usize,
    in_out2: usize,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainType {
    test: FbTest,
    r_a: usize,
    r_b: usize,
    p_c: usize,
    p_d: usize,
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    b_result_a: bool,
    b_result_b: bool,
    b_result_c: bool,
    b_result_d: bool,
    b_result_e: bool,
    b_result_f: bool,
    b_result_g: bool,
    b_result_h: bool,
    b_result_i: bool,
    b_result_j: bool,
    b_result_k: bool,
}

fn new() -> MainType {
    MainType::default()
}

#[test]
fn reference_call() {
    let function = r"

    FUNCTION_BLOCK fbTest
        VAR_INPUT
            reference : REF_TO BOOL;  (* REF_TO *)
            p : POINTER TO BOOL;
        END_VAR
        VAR_OUTPUT
        END_VAR
        VAR_IN_OUT
            in_out1: POINTER TO BOOL;    (* REF_TO *)
            in_out2: POINTER TO BOOL;
        END_VAR
        VAR
        END_VAR

        reference^ := TRUE;
        p^ := TRUE;
        in_out1^ := TRUE;
        in_out2^ := TRUE;
    END_FUNCTION_BLOCK

     FUNCTION other : BOOL
        VAR
        END_VAR
        VAR_INPUT
            reference : REF_TO BOOL;  (* REF_TO *)
            p : POINTER TO BOOL;
        END_VAR
        VAR_IN_OUT
            in_out1: POINTER TO BOOL;    (* REF_TO *)
            in_out2: POINTER TO BOOL;
        END_VAR

        reference^ := TRUE;
        p^ := TRUE;
        in_out1^ := TRUE;
        in_out2^ := TRUE;

    END_FUNCTION

    PROGRAM main
    VAR
        test: fbTest;
        r_a : REF_TO BOOL;        (* REF_TO *)
        r_b : REF_TO BOOL;        (* REF_TO *)
        p_c : POINTER TO BOOL;
        p_d : POINTER TO BOOL;
        a : BOOL := FALSE;
        b : BOOL := FALSE;
        c : BOOL := FALSE;
        d : BOOL := FALSE;
        b_result_a : BOOL := FALSE;
        b_result_b : BOOL := FALSE;
        b_result_c : BOOL := FALSE;
        b_result_d : BOOL := FALSE;
        b_result_e : BOOL := FALSE;
        b_result_f : BOOL := FALSE;
        b_result_g: BOOL := FALSE;
        b_result_h: BOOL := FALSE;
        b_result_i: BOOL := FALSE;
        b_result_j: BOOL := FALSE;
        b_result_k: BOOL := FALSE;
    END_VAR
        r_a := REF(a); (* ADR *)
        p_c := r_a;
        r_a^ := TRUE;
        b_result_a := r_a^;
        b_result_b := p_c^;

        p_d := REF(d);  (* ADR *)
        p_d^ := TRUE;
        b_result_c := p_d^;

        r_b := REF(b);    (* ADR *)
        p_c := REF(c);    (* ADR *)
        r_a^ := FALSE; r_b^:= FALSE; p_c^ := FALSE; p_d^:= FALSE;
        test(reference := r_a, p := p_c, in_out1 := r_b, in_out2 := p_d);
        b_result_d := r_a^; b_result_e := r_b^; b_result_f := p_c^; b_result_g := p_d^;

        r_a^ := FALSE; r_b^:= FALSE; p_c^ := FALSE; p_d^:= FALSE;
        other(reference := r_a, p := p_c, in_out1 := r_b, in_out2 := p_d);
        b_result_h := r_a^; b_result_i := r_b^; b_result_j := p_c^; b_result_k := p_d^;
END_PROGRAM
    ";

    let mut maintype = new();

    let _: i32 = compile_and_run(function.to_string(), &mut maintype);

    assert!(maintype.b_result_a);
    assert!(maintype.b_result_b);
    assert!(maintype.b_result_c);
    assert!(maintype.b_result_d);
    assert!(maintype.b_result_e);
    assert!(maintype.b_result_f);
    assert!(maintype.b_result_g);
    assert!(maintype.b_result_h);
    assert!(maintype.b_result_i);
    assert!(maintype.b_result_j);
    assert!(maintype.b_result_k);
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTestStruct {
    __vtable: usize,
    reference: usize,
    p: usize,
    in_out1: usize,
    in_out2: usize,
}
#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MyStruct {
    field1: bool,
    field2: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]

struct MainTypeWithStruct {
    test: FbTestStruct,
    a: MyStruct,
    b: MyStruct,
    ref_a: usize,
    ref_b: usize,
    p_a: usize,
    p_b: usize,
    b_result_a: bool,
    b_result_b: i16,
    b_result_c: bool,
    b_result_d: i16,
    b_result_e: bool,
    b_result_f: i16,
    b_result_g: bool,
    b_result_h: i16,
}

#[test]
fn reference_call_struct() {
    let function = r"
    TYPE MyStruct:
    STRUCT
        field1 : BOOL;
        field2 : INT;
    END_STRUCT
    END_TYPE

    FUNCTION_BLOCK FbTestStruct
        VAR_INPUT
            reference : REF_TO MyStruct; (* REF_TO *)
            p : POINTER TO MyStruct;
        END_VAR
        VAR_OUTPUT
        END_VAR
        VAR_IN_OUT
            in_out1: REF_TO MyStruct;  (* REF_TO *)
            in_out2: POINTER TO MyStruct;
        END_VAR

        reference^.field1 := TRUE;
        p^.field2 := 100;
        in_out1^.field1 := TRUE;
        in_out2^.field2 := 100;
    END_FUNCTION_BLOCK

    FUNCTION other : BOOL
        VAR
        END_VAR
        VAR_INPUT
            reference: REF_TO  MyStruct;         (* REF_TO *)
            p: POINTER TO MyStruct;
        END_VAR

        VAR_IN_OUT
            in_out1: REF_TO MyStruct;    (* REF_TO *)
            in_out2: POINTER TO MyStruct;
        END_VAR

        reference^.field1 := TRUE;
        p^.field2 := 100;
        in_out1^.field1 := TRUE;
        in_out2^.field2 := 100;
    END_FUNCTION

    PROGRAM main
    VAR
        test: FbTestStruct;
        a : MyStruct;
        b : MyStruct;
        ref_a : REF_TO MyStruct; (* REF_TO *)
        ref_b : REF_TO MyStruct;     (* REF_TO *)
        p_a : POINTER TO MyStruct;
        p_b : POINTER TO MyStruct;
        b_result_a : BOOL := FALSE;
        b_result_b : INT := 0;
        b_result_c : BOOL := FALSE;
        b_result_d : INT := 0;
        b_result_e : BOOL := FALSE;
        b_result_f : INT := 0;
        b_result_g : BOOL := FALSE;
        b_result_h : INT := 0;
    END_VAR
        a.field1 := FALSE; a.field2 := 0;
        b.field1 := FALSE; b.field2 := 0;

        ref_a := REF(a);
        ref_b := REF(b);
        p_a := REF(a);
        p_b := REF(b);
        other(reference := ref_a, p:= p_a, in_out1 := ref_b, in_out2 := p_b);
        b_result_a := a.field1;
        b_result_b := a.field2;
        b_result_c := b.field1;
        b_result_d := b.field2;

        a.field1 := FALSE; a.field2 := 0;
        b.field1 := FALSE; b.field2 := 0;

        test(reference := ref_a, p := ref_a, in_out1 := ref_b, in_out2 := ref_b);
        b_result_e := a.field1;
        b_result_f := a.field2;
        b_result_g := b.field1;
        b_result_h := b.field2;

END_PROGRAM

    ";

    let mut new_with_struct: MainTypeWithStruct = MainTypeWithStruct::default();

    let _: i32 = compile_and_run(function.to_string(), &mut new_with_struct);

    assert!(new_with_struct.b_result_a);
    assert_eq!(100, new_with_struct.b_result_b);
    assert!(new_with_struct.b_result_c);
    assert_eq!(100, new_with_struct.b_result_d);
    assert!(new_with_struct.b_result_e);
    assert_eq!(100, new_with_struct.b_result_f);
    assert!(new_with_struct.b_result_g);
    assert_eq!(100, new_with_struct.b_result_h);
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTestArray {
    __vtable: usize,
    reference: usize,
    p: usize,
    in_out1: usize,
    in_out2: usize,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypeWithArray {
    test: FbTestArray,
    a: [i16; 2],
    b: [i16; 2],
    c: [i16; 2],
    d: [i16; 2],
    ref_a: usize,
    ref_b: usize,
    p_c: usize,
    p_d: usize,
    b_result_a: i16,
    b_result_b: i16,
    b_result_c: i16,
    b_result_d: i16,
    b_result_e: i16,
    b_result_f: i16,
    b_result_g: i16,
    b_result_h: i16,
    b_result_i: i16,
    b_result_j: i16,
    b_result_k: i16,
    b_result_l: i16,
    b_result_m: i16,
    b_result_n: i16,
    b_result_o: i16,
    b_result_p: i16,
}

#[test]
fn reference_call_array() {
    let function = r"
    FUNCTION_BLOCK FbTestArray
        VAR_INPUT
            reference : REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
            p : POINTER TO ARRAY[1..2] OF INT;
        END_VAR
        VAR_IN_OUT
            in_out1: REF_TO ARRAY[1..2] OF INT;  (* REF_TO *)
            in_out2: POINTER TO ARRAY[1..2] OF INT;
        END_VAR

        reference^[1] := 100;
        p^[2] := 100;
        in_out1^[1] := 100;
        in_out2^[2] := 100;
    END_FUNCTION_BLOCK

    FUNCTION other : BOOL
        VAR_INPUT
            reference : REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
            p : POINTER TO ARRAY[1..2] OF INT;
        END_VAR
        VAR_IN_OUT
            in_out1: REF_TO ARRAY[1..2] OF INT;  (* REF_TO *)
            in_out2: POINTER TO ARRAY[1..2] OF INT;
        END_VAR

        reference^[1] := 100;
        p^[2] := 100;
        in_out1^[1] := 100;
        in_out2^[2] := 100;
    END_FUNCTION

    PROGRAM main
        VAR
            test: FbTestArray;
            a : ARRAY[1..2] OF INT;
            b : ARRAY[1..2] OF INT;
            c : ARRAY[1..2] OF INT;
            d : ARRAY[1..2] OF INT;
            ref_a : REF_TO ARRAY[1..2] OF INT;    (* REF_TO *)
            ref_b : REF_TO ARRAY[1..2] OF INT;    (* REF_TO *)
            p_c : POINTER TO ARRAY[1..2] OF INT;

            p_d : POINTER TO ARRAY[1..2] OF INT;
            b_result_a : INT := 0;
            b_result_b : INT := 0;
            b_result_c : INT := 0;
            b_result_d : INT := 0;
            b_result_e : INT := 0;
            b_result_f : INT := 0;
            b_result_g : INT := 0;
            b_result_h : INT := 0;
            b_result_i : INT := 0;
            b_result_j : INT := 0;
            b_result_k : INT := 0;
            b_result_l : INT := 0;
            b_result_m : INT := 0;
            b_result_n : INT := 0;
            b_result_o : INT := 0;
            b_result_p : INT := 0;
        END_VAR

            ref_a := REF(a);
            ref_b := REF(b);
            p_c := REF(c);
            p_d := REF(d);

            a[1] := 0; a[2] := 0;
            b[1] := 0; b[2] := 0;
            c[1] := 0; c[2] := 0;
            d[1] := 0; d[2] := 0;

            other(reference := ref_a, p:= p_c, in_out1 := ref_b, in_out2 := p_d);
            b_result_a := a[1];
            b_result_b := a[2];
            b_result_c := b[1];
            b_result_d := b[2];
            b_result_e := c[1];
            b_result_f := c[2];
            b_result_g := d[1];
            b_result_h := d[2];

            a[1] := 0; a[2] := 0;
            b[1] := 0; b[2] := 0;
            c[1] := 0; c[2] := 0;
            d[1] := 0; d[2] := 0;

            test(reference := ref_a, p := p_c, in_out1 := ref_b, in_out2 := p_d);
            b_result_i := a[1];
            b_result_j := a[2];
            b_result_k := b[1];
            b_result_l := b[2];
            b_result_m := c[1];
            b_result_n := c[2];
            b_result_o := d[1];
            b_result_p := d[2];
    END_PROGRAM
  ";

    let mut new_with_array: MainTypeWithArray = MainTypeWithArray::default();

    let _: i32 = compile_and_run(function.to_string(), &mut new_with_array);

    assert_eq!(100, new_with_array.b_result_a);
    assert_eq!(0, new_with_array.b_result_b);
    assert_eq!(100, new_with_array.b_result_c);
    assert_eq!(0, new_with_array.b_result_d);
    assert_eq!(0, new_with_array.b_result_e);
    assert_eq!(100, new_with_array.b_result_f);
    assert_eq!(0, new_with_array.b_result_g);
    assert_eq!(100, new_with_array.b_result_h);
    assert_eq!(100, new_with_array.b_result_i);
    assert_eq!(0, new_with_array.b_result_j);
    assert_eq!(100, new_with_array.b_result_k);
    assert_eq!(0, new_with_array.b_result_l);
    assert_eq!(0, new_with_array.b_result_m);
    assert_eq!(100, new_with_array.b_result_n);
    assert_eq!(0, new_with_array.b_result_o);
    assert_eq!(100, new_with_array.b_result_p);
}

#[test]
fn multiple_pointer_dereference() {
    struct MainType {
        a: u8,
        b: u8,
    }

    let src = r#"
        PROGRAM main
        VAR
            a: BYTE;
            b: BYTE;
        END_VAR
        VAR_TEMP
            c: REF_TO BYTE;
            d: REF_TO REF_TO BYTE;
            e: BYTE;
        END_VAR
            c := REF(a);
            d := REF(c);
            b := d^^;
            e := (d^)^;
            a := e + 16#01;
        END_PROGRAM
    "#;

    let mut maintype = MainType { a: 0xFF, b: 0 };
    let _: i32 = compile_and_run(src, &mut maintype);

    assert_eq!(0x00_u8, maintype.a);
    assert_eq!(0xFF_u8, maintype.b);
}
