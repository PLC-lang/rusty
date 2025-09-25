// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// failing because of #1508
FUNCTION foo
VAR_IN_OUT
    in : ARRAY[0..1] OF ARRAY[0..1] OF INT;
END_VAR
    in[0][0] := 1;
    in[0][1] := 2;
    in[1][0] := 3;
    in[1][1] := 4;
END_FUNCTION

FUNCTION main
VAR
    arr: ARRAY[0..1] OF ARRAY[0..1] OF INT;
END_VAR
    foo(arr);
    // CHECK: 1 2 3 4
    printf('%d %d %d %d$N', arr[0][0], arr[0][1], arr[1][0], arr[1][1]);
END_FUNCTION
