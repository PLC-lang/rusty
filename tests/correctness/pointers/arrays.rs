use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
#[derive(Default)]
struct FbTestArray {
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

    let mut new_with_array: MainTypeWithArray = MainTypeWithArray::default();

    compile_and_run::<_, i32>(function.to_string(), &mut new_with_array);

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
