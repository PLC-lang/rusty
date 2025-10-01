// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo
VAR_IN_OUT
    in_out : ARRAY[0..3] OF STRING;
END_VAR
VAR_INPUT {ref}
    in: ARRAY[0..3] OF STRING;
END_VAR
    in_out[0] := 'YOU';
    in_out[1] := 'SHALL';
    in[2] := 'NOT';
    in[3] := 'SEGFAULT';
END_FUNCTION

FUNCTION main
VAR
    arr: ARRAY[0..3] OF STRING;
END_VAR
    foo(arr, arr);
    // CHECK: YOU SHALL NOT SEGFAULT
    printf('%s %s %s %s$N', REF(arr[0]), REF(arr[1]), REF(arr[2]), REF(arr[3]));
END_FUNCTION
