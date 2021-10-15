use crate::compile_and_run;

#[allow(dead_code)]
#[repr(C)]
#[derive(Default, Debug)]
struct FbTestStruct {
    r: usize,
    p: usize,
    in_out1: usize,
    in_out2: usize,
}
#[allow(dead_code)]
#[repr(C)]
#[derive(Default, Debug)]
struct MyStruct {
    field1: bool,
    field2: i16,
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Default, Debug)]
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

    let mut new_with_struct: MainTypeWithStruct = MainTypeWithStruct::default();

    compile_and_run::<_, i32>(function.to_string(), &mut new_with_struct);

    let new_with_struct = dbg!(new_with_struct);
    assert_eq!(true, new_with_struct.b_result_a);
    assert_eq!(100, new_with_struct.b_result_b);
    assert_eq!(true, new_with_struct.b_result_c);
    assert_eq!(100, new_with_struct.b_result_d);
    assert_eq!(true, new_with_struct.b_result_e);
    assert_eq!(100, new_with_struct.b_result_f);
    assert_eq!(true, new_with_struct.b_result_g);
}
