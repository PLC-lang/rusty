// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK FbTestArray
    VAR_INPUT
        reference: REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
        p: POINTER TO ARRAY[1..2] OF INT;
    END_VAR
    VAR_IN_OUT
        in_out1: REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
        in_out2: POINTER TO ARRAY[1..2] OF INT;
    END_VAR

    reference^[1] := 100;
    p^[2] := 100;
    in_out1^[1] := 100;
    in_out2^[2] := 100;
END_FUNCTION_BLOCK

FUNCTION other: BOOL
    VAR_INPUT
        reference: REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
        p: POINTER TO ARRAY[1..2] OF INT;
    END_VAR
    VAR_IN_OUT
        in_out1: REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
        in_out2: POINTER TO ARRAY[1..2] OF INT;
    END_VAR

    reference^[1] := 100;
    p^[2] := 100;
    in_out1^[1] := 100;
    in_out2^[2] := 100;
END_FUNCTION

FUNCTION main
    VAR
        test: FbTestArray;
        a: ARRAY[1..2] OF INT;
        b: ARRAY[1..2] OF INT;
        c: ARRAY[1..2] OF INT;
        d: ARRAY[1..2] OF INT;
        ref_a: REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
        ref_b: REF_TO ARRAY[1..2] OF INT; (* REF_TO *)
        p_c: POINTER TO ARRAY[1..2] OF INT;

        p_d: POINTER TO ARRAY[1..2] OF INT;
        b_result_a: INT := 0;
        b_result_b: INT := 0;
        b_result_c: INT := 0;
        b_result_d: INT := 0;
        b_result_e: INT := 0;
        b_result_f: INT := 0;
        b_result_g: INT := 0;
        b_result_h: INT := 0;
        b_result_i: INT := 0;
        b_result_j: INT := 0;
        b_result_k: INT := 0;
        b_result_l: INT := 0;
        b_result_m: INT := 0;
        b_result_n: INT := 0;
        b_result_o: INT := 0;
        b_result_p: INT := 0;
    END_VAR

        ref_a := REF(a);
        ref_b := REF(b);
        p_c := REF(c);
        p_d := REF(d);

        a[1] := 0; a[2] := 0;
        b[1] := 0; b[2] := 0;
        c[1] := 0; c[2] := 0;
        d[1] := 0; d[2] := 0;

        other(reference := ref_a, p := p_c, in_out1 := ref_b, in_out2 := p_d);
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


    printf('%d$N', b_result_a); // CHECK: 100
    printf('%d$N', b_result_b); // CHECK: 0
    printf('%d$N', b_result_c); // CHECK: 100
    printf('%d$N', b_result_d); // CHECK: 0
    printf('%d$N', b_result_e); // CHECK: 0
    printf('%d$N', b_result_f); // CHECK: 100
    printf('%d$N', b_result_g); // CHECK: 0
    printf('%d$N', b_result_h); // CHECK: 100
    printf('%d$N', b_result_i); // CHECK: 100
    printf('%d$N', b_result_j); // CHECK: 0
    printf('%d$N', b_result_k); // CHECK: 100
    printf('%d$N', b_result_l); // CHECK: 0
    printf('%d$N', b_result_m); // CHECK: 0
    printf('%d$N', b_result_n); // CHECK: 100
    printf('%d$N', b_result_o); // CHECK: 0
    printf('%d$N', b_result_p); // CHECK: 100
END_FUNCTION
