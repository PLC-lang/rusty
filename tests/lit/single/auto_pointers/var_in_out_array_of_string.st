// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo
VAR_IN_OUT
    in : ARRAY[0..1] OF STRING;
END_VAR
VAR_INPUT
    by_val: ARRAY[0..1] OF STRING;
END_VAR
    in[0] := 'hello';
    in[1] := 'world';
    by_val[0] := 'hello';
    by_val[1] := 'world';
END_FUNCTION

FUNCTION main
VAR
    arr: ARRAY[0..1] OF STRING;
END_VAR
    foo(arr, arr);
    // CHECK: hello world
    printf('%s %s$N', REF(arr[0]), REF(arr[1]));
END_FUNCTION
