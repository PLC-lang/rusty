// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fbTest
    VAR_INPUT
        reference: REF_TO BOOL; (* REF_TO *)
        p: POINTER TO BOOL;
    END_VAR
    VAR_OUTPUT
    END_VAR
    VAR_IN_OUT
        in_out1: POINTER TO BOOL; (* REF_TO *)
        in_out2: POINTER TO BOOL;
    END_VAR
    VAR
    END_VAR

    reference^ := TRUE;
    p^ := TRUE;
    in_out1^ := TRUE;
    in_out2^ := TRUE;
END_FUNCTION_BLOCK

    FUNCTION other: BOOL
    VAR
    END_VAR
    VAR_INPUT
        reference: REF_TO BOOL; (* REF_TO *)
        p: POINTER TO BOOL;
    END_VAR
    VAR_IN_OUT
        in_out1: POINTER TO BOOL; (* REF_TO *)
        in_out2: POINTER TO BOOL;
    END_VAR

    reference^ := TRUE;
    p^ := TRUE;
    in_out1^ := TRUE;
    in_out2^ := TRUE;

END_FUNCTION

FUNCTION main
VAR
    test: fbTest;
    r_a: REF_TO BOOL; (* REF_TO *)
    r_b: REF_TO BOOL; (* REF_TO *)
    p_c: POINTER TO BOOL;
    p_d: POINTER TO BOOL;
    a: BOOL := FALSE;
    b: BOOL := FALSE;
    c: BOOL := FALSE;
    d: BOOL := FALSE;
    b_result_a: BOOL := FALSE;
    b_result_b: BOOL := FALSE;
    b_result_c: BOOL := FALSE;
    b_result_d: BOOL := FALSE;
    b_result_e: BOOL := FALSE;
    b_result_f: BOOL := FALSE;
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

    p_d := REF(d); (* ADR *)
    p_d^ := TRUE;
    b_result_c := p_d^;

    r_b := REF(b); (* ADR *)
    p_c := REF(c); (* ADR *)
    r_a^ := FALSE; r_b^ := FALSE; p_c^ := FALSE; p_d^ := FALSE;
    test(reference := r_a, p := p_c, in_out1 := r_b, in_out2 := p_d);
    b_result_d := r_a^; b_result_e := r_b^; b_result_f := p_c^; b_result_g := p_d^;

    r_a^ := FALSE; r_b^ := FALSE; p_c^ := FALSE; p_d^ := FALSE;
    other(reference := r_a, p := p_c, in_out1 := r_b, in_out2 := p_d);
    b_result_h := r_a^; b_result_i := r_b^; b_result_j := p_c^; b_result_k := p_d^;


    printf('%d$N', b_result_a); // CHECK: 1
    printf('%d$N', b_result_b); // CHECK: 1
    printf('%d$N', b_result_c); // CHECK: 1
    printf('%d$N', b_result_d); // CHECK: 1
    printf('%d$N', b_result_e); // CHECK: 1
    printf('%d$N', b_result_f); // CHECK: 1
    printf('%d$N', b_result_g); // CHECK: 1
    printf('%d$N', b_result_h); // CHECK: 1
    printf('%d$N', b_result_i); // CHECK: 1
    printf('%d$N', b_result_j); // CHECK: 1
    printf('%d$N', b_result_k); // CHECK: 1
END_FUNCTION