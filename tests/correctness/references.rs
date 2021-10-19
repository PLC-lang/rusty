use crate::compile_and_run;

use super::arrays;

// Copyright (c) 2021 Daniel Schwenniger
#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTest {
    r: usize,
    p: usize,
    InOut1: usize,
    InOut2: usize,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainType {
    test: FbTest,
    rA: usize,
    rA_: usize,
    pA: usize,
    pA_: usize,
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    bResultA: bool,
    bResultB: bool,
    bResultC: bool,
    bResultD: bool,
    bResultE: bool,
    bResultF: bool,
    bResultG: bool,
    bResultH: bool,
    bResultI: bool,
    bResultJ: bool,
}

fn new() -> MainType {
    MainType::default()
}

#[test]
fn reference_call() {
    let function = r"
 
    (* TODO: Referenz auf Struktur/Array *)

    FUNCTION_BLOCK fbTest
        VAR_INPUT
            r : REF_TO BOOL;
            p : POINTER TO BOOL;
        END_VAR
        VAR_OUTPUT
        END_VAR
        VAR_IN_OUT
            InOut1: REF_TO BOOL;
            InOut2: POINTER TO BOOL;
        END_VAR
        VAR
        END_VAR

        r^ := TRUE;
        p^ := TRUE;
        InOut1^ := TRUE;
        InOut2^ := TRUE;
    END_FUNCTION_BLOCK

    FUNCTION other2 : BOOL
        VAR
        END_VAR
        VAR_INPUT
            ref : POINTER TO BOOL;
        END_VAR
        VAR_IN_OUT
            InOut1: REF_TO BOOL;
            InOut2: POINTER TO BOOL;
        END_VAR

        ref^ := TRUE;
        InOut1^ := TRUE;
        InOut2^ := TRUE;
    END_FUNCTION

    FUNCTION other : BOOL
        VAR
        END_VAR
        VAR_INPUT
            ref : REF_TO BOOL;
        END_VAR
        ;

        ref^ := TRUE;
    END_FUNCTION
    
    PROGRAM main
    VAR
        test: fbTest;
        rA : REF_TO BOOL;
        rA_ : REF_TO BOOL;
        pA : POINTER TO BOOL;
        pA_ : POINTER TO BOOL;
        a : BOOL := FALSE;
        b : BOOL := FALSE;
        c : BOOL := FALSE;
        d : BOOL := FALSE;
        bResultA : BOOL := FALSE;
        bResultB : BOOL := FALSE;
        bResultC : BOOL := FALSE;
        bResultD : BOOL := FALSE;
        bResultE : BOOL := FALSE;
        bResultF : BOOL := FALSE;
        bResultG: BOOL := FALSE;
        bResultH: BOOL := FALSE;
        bResultI: BOOL := FALSE;
        bResultJ: BOOL := FALSE;
    END_VAR
        rA := &a; (* REF_TO *)
        pA := rA;
        rA^ := TRUE;
        bResultA := rA^;
        bResultB := pA^;

        pA_ := &d;  (* ADR *)
        pa_^ := TRUE;
        bResultC := pA_^;

        rA_ := &b;
        pa := &c;
        rA^ := FALSE; pA^ := FALSE; rA_^:= FALSE; pA_^:= FALSE;
        test(R := rA, P := pA, INOUT1 := rA_, INOUT2 := pA_);
        bResultD := rA^; bResultE := pA^; bResultF := rA_^; bResultG := pA_^;
        
        other(ref := rA_); 
        bResultH := rA_^;
        
        pA^ := FALSE; rA^ := FALSE; pA_^ := FALSE;
        other2(REF := pA, INOUT1 := rA, INOUT2 := pA_);
        bResultH := pA^; bResultI := rA^; bResultJ := pA_^;
    END_PROGRAM
    ";

    let mut maintype = new();

    compile_and_run::<_, i32>(function.to_string(), &mut maintype);

    assert_eq!(true, maintype.bResultA);
    assert_eq!(true, maintype.bResultB);
    assert_eq!(true, maintype.bResultC);
    assert_eq!(true, maintype.bResultD);
    assert_eq!(true, maintype.bResultE);
    assert_eq!(true, maintype.bResultF);
    assert_eq!(true, maintype.bResultG);
    assert_eq!(true, maintype.bResultH);
    assert_eq!(true, maintype.bResultI);
    assert_eq!(true, maintype.bResultJ);

}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTestStruct {
    r: usize,
    p: usize,
    InOut1: usize,
    InOut2: usize,
}
#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MyStruct {
    Field1: bool,
    Field2: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct MainTypeWithStruct {
    test: FbTestStruct,
    a: MyStruct,
    b: MyStruct,
    refA: usize,
    refB: usize,
    pA : usize,
    pB : usize,
    bResultA: bool,
    bResultB: i16,
    bResultC: bool,
    bResultD: i16,
    bResultE: bool,
    bResultF: i16,
    bResultG: bool,
    bResultH: i16,
}

fn newWithStruct() -> MainTypeWithStruct {
    MainTypeWithStruct::default()
}

#[test]
fn reference_call_struct() {
    let function = r"
    (* TODO: Referenz auf Struktur/Array *)
    TYPE MyStruct:
    STRUCT
        Field1 : BOOL;
        Field2 : INT;
    END_STRUCT
    END_TYPE

    FUNCTION_BLOCK FbTestStruct
        VAR_INPUT
            ref : REF_TO MyStruct; (* REF_TO *)
            p : POINTER TO MyStruct;
        END_VAR
        VAR_OUTPUT
        END_VAR
        VAR_IN_OUT
            InOut1: REF_TO MyStruct;  (* REF_TO *)
            InOut2: POINTER TO MyStruct;
        END_VAR

        ref^.Field1 := TRUE;
        p^.Field2 := 100;
        InOut1^.Field1 := TRUE;
        InOut2^.Field2 := 100;
    END_FUNCTION_BLOCK

    FUNCTION other : BOOL
        VAR
        END_VAR
        VAR_INPUT
            ref: REF_TO  MyStruct;         (* REF_TO *)
            p: POINTER TO MyStruct;
        END_VAR

        VAR_IN_OUT
            InOut1: REF_TO MyStruct;    (* REF_TO *)
            InOut2: POINTER TO MyStruct;
        END_VAR

        ref^.Field1 := TRUE;
        p^.Field2 := 100;
        InOut1^.Field1 := TRUE;
        InOut2^.Field2 := 100;
    END_FUNCTION

    PROGRAM main
    VAR
        test: FbTestStruct;
        a : MyStruct;
        b : MyStruct;
        refA : REF_TO MyStruct; (* REF_TO *)
        refB : REF_TO MyStruct;     (* REF_TO *)
        pA : POINTER TO MyStruct;
        pB : POINTER TO MyStruct; 
        bResultA : BOOL := FALSE;
        bResultB : INT := 0;
        bResultC : BOOL := FALSE;
        bResultD : INT := 0;
        bResultE : BOOL := FALSE;
        bResultF : INT := 0;
        bResultG : BOOL := FALSE;
        bResultH : INT := 0;
    END_VAR
        a.Field1 := FALSE; a.Field2 := 0;
        b.Field1 := FALSE; b.Field2 := 0;
        
        refA := &a; refB := &b;
        pA := &a; pB := &b;
        other(ref := refA, p:= pA, InOut1 := refB, InOut2 := pB);
        bResultA := a.Field1; 
        bResultB := a.Field2;
        bResultC := b.Field1; 
        bResultD := b.Field2;

        a.Field1 := FALSE; a.Field2 := 0;
        b.Field1 := FALSE; b.Field2 := 0;

        test(ref := refA, p := refA, InOut1 := refB, InOut2 := refB);
        bResultE := a.Field1; 
        bResultF := a.Field2;
        bResultG := b.Field1; 
        bResultH := b.Field2;

END_PROGRAM

    ";

    let mut newWithStruct: MainTypeWithStruct = newWithStruct();

    compile_and_run::<_, i32>(function.to_string(), &mut newWithStruct);

    assert_eq!(true, newWithStruct.bResultA);
    assert_eq!(100, newWithStruct.bResultB);
    assert_eq!(true, newWithStruct.bResultC);
    assert_eq!(100, newWithStruct.bResultD);
    assert_eq!(true, newWithStruct.bResultE);
    assert_eq!(100, newWithStruct.bResultF);
    assert_eq!(true, newWithStruct.bResultG);
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTestArray {
    reference: usize,
    p: usize,
    InOut1: usize,
    InOut2: usize,
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
    refA: usize,
    refB: usize,
    pC: usize,
    pD: usize,
    bResultA: i16,
    bResultB: i16,
    bResultC: i16,
    bResultD: i16,
    bResultE: i16,
    bResultF: i16,
    bResultG: i16,
    bResultH: i16,
    bResultI: i16,
    bResultJ: i16,
    bResultK: i16,
    bResultL: i16,
    bResultM: i16,
    bResultN: i16,
    bResultO: i16,
    bResultP: i16,
}

fn newWithArray() -> MainTypeWithArray {
    MainTypeWithArray::default()
}

#[test]
fn reference_call_array() {
    let function = r"
    (* TODO: Referenz auf Struktur/Array *)
    FUNCTION_BLOCK FbTestArray
        VAR_INPUT
            reference : REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
            p : POINTER TO ARRAY[1..2] OF INT;
        END_VAR
        VAR_IN_OUT
            InOut1: REF_TO ARRAY[1..2] OF INT;  (* REF_TO *)
            InOut2: POINTER TO ARRAY[1..2] OF INT;
        END_VAR

        reference^[1] := 100;
        p^[2] := 100;
        InOut1^[1] := 100;
        InOut2^[2] := 100;
    END_FUNCTION_BLOCK

    FUNCTION other : BOOL
        VAR_INPUT
            reference : REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
            p : POINTER TO ARRAY[1..2] OF INT;
        END_VAR
        VAR_IN_OUT
            InOut1: REF_TO ARRAY[1..2] OF INT;  (* REF_TO *)
            InOut2: POINTER TO ARRAY[1..2] OF INT;
        END_VAR

        reference^[1] := 100;
        p^[2] := 100;
        InOut1^[1] := 100;
        InOut2^[2] := 100;
    END_FUNCTION

    PROGRAM main
        VAR
            test: FbTestArray;
            a : ARRAY[1..2] OF INT;
            b : ARRAY[1..2] OF INT;
            c : ARRAY[1..2] OF INT;
            d : ARRAY[1..2] OF INT;
            refA : REF_TO ARRAY[1..2] OF INT;    (* REF_TO *)
            refB : REF_TO ARRAY[1..2] OF INT;    (* REF_TO *)
            pC : POINTER TO ARRAY[1..2] OF INT;
            pD : POINTER TO ARRAY[1..2] OF INT; 
            bResultA : INT := 0;
            bResultB : INT := 0;
            bResultC : INT := 0;
            bResultD : INT := 0;
            bResultE : INT := 0;
            bResultF : INT := 0;
            bResultG : INT := 0;
            bResultH : INT := 0;
            bResultI	: INT := 0;
            bResultJ	: INT := 0;
            bResultK	: INT := 0;
            bResultL	: INT := 0;
            bResultM	: INT := 0;
            bResultN	: INT := 0;
            bResultO	: INT := 0;
            bResultP	: INT := 0;
        END_VAR

            refA := &a; refB := &b;
            pC := &c; pD := &d;
            
            a[1] := 0; a[2] := 0;
            b[1] := 0; b[2] := 0;
            c[1] := 0; c[2] := 0;
            d[1] := 0; d[2] := 0;
            
            other(reference := refA, p:= pC, InOut1 := refB, InOut2 := pD);
            bResultA := a[1];
            bResultB := a[2];
            bResultC := b[1]; 
            bResultD := b[2];
            bResultE := c[1];
            bResultF := c[2];
            bResultG := d[1];
            bResultH := d[2];

            a[1] := 0; a[2] := 0;
            b[1] := 0; b[2] := 0;
            c[1] := 0; c[2] := 0;
            d[1] := 0; d[2] := 0;
            
            test(reference := refA, p := pC, InOut1 := refB, InOut2 := pD);
            bResultI := a[1];
            bResultJ := a[2];
            bResultK := b[1]; 
            bResultL := b[2];
            bResultM := c[1];
            bResultN := c[2];
            bResultO := d[1];
            bResultP := d[2];
    END_PROGRAM
  ";

    let mut newWithArray: MainTypeWithArray = newWithArray();

    compile_and_run::<_, i32>(function.to_string(), &mut newWithArray);

    assert_eq!(100, newWithArray.bResultA);
    assert_eq!(0, newWithArray.bResultB);
    assert_eq!(100, newWithArray.bResultC);
    assert_eq!(0, newWithArray.bResultD);
    assert_eq!(0, newWithArray.bResultE);
    assert_eq!(100, newWithArray.bResultF);
    assert_eq!(0, newWithArray.bResultG);
    assert_eq!(100, newWithArray.bResultH);
    assert_eq!(100, newWithArray.bResultI);
    assert_eq!(0, newWithArray.bResultJ);
    assert_eq!(100, newWithArray.bResultK);
    assert_eq!(0, newWithArray.bResultL);
    assert_eq!(0, newWithArray.bResultM);
    assert_eq!(100, newWithArray.bResultN);
    assert_eq!(0, newWithArray.bResultO);
    assert_eq!(100, newWithArray.bResultP);
}