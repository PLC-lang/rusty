// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE MyStruct:
STRUCT
    field1: BOOL;
    field2: INT;
END_STRUCT
END_TYPE

FUNCTION_BLOCK FbTestStruct
    VAR_INPUT
        reference: REF_TO MyStruct; (* REF_TO *)
        p: POINTER TO MyStruct;
    END_VAR
    VAR_OUTPUT
    END_VAR
    VAR_IN_OUT
        in_out1: REF_TO MyStruct; (* REF_TO *)
        in_out2: POINTER TO MyStruct;
    END_VAR

    reference^.field1 := TRUE;
    p^.field2 := 100;
    in_out1^.field1 := TRUE;
    in_out2^.field2 := 100;
END_FUNCTION_BLOCK

FUNCTION other: BOOL
    VAR
    END_VAR
    VAR_INPUT
        reference: REF_TO MyStruct; (* REF_TO *)
        p: POINTER TO MyStruct;
    END_VAR

    VAR_IN_OUT
        in_out1: REF_TO MyStruct; (* REF_TO *)
        in_out2: POINTER TO MyStruct;
    END_VAR

    reference^.field1 := TRUE;
    p^.field2 := 100;
    in_out1^.field1 := TRUE;
    in_out2^.field2 := 100;
END_FUNCTION

FUNCTION main
    VAR
        test: FbTestStruct;
        a: MyStruct;
        b: MyStruct;
        ref_a: REF_TO MyStruct; (* REF_TO *)
        ref_b: REF_TO MyStruct; (* REF_TO *)
        p_a: POINTER TO MyStruct;
        p_b: POINTER TO MyStruct;
        b_result_a: BOOL := FALSE;
        b_result_b: INT := 0;
        b_result_c: BOOL := FALSE;
        b_result_d: INT := 0;
        b_result_e: BOOL := FALSE;
        b_result_f: INT := 0;
        b_result_g: BOOL := FALSE;
        b_result_h: INT := 0;
    END_VAR
        a.field1 := FALSE; a.field2 := 0;
        b.field1 := FALSE; b.field2 := 0;

        ref_a := REF(a);
        ref_b := REF(b);
        p_a := REF(a);
        p_b := REF(b);
        other(reference := ref_a, p := p_a, in_out1 := ref_b, in_out2 := p_b);
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

    printf('%d$N', b_result_a); // CHECK: 1
    printf('%d$N', b_result_b); // CHECK: 100
    printf('%d$N', b_result_c); // CHECK: 1
    printf('%d$N', b_result_d); // CHECK: 100
    printf('%d$N', b_result_e); // CHECK: 1
    printf('%d$N', b_result_f); // CHECK: 100
    printf('%d$N', b_result_g); // CHECK: 1
    printf('%d$N', b_result_h); // CHECK: 100
END_FUNCTION