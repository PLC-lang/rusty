// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL 
    arr : ARRAY[1..3] OF DINT;
END_VAR

FUNCTION_BLOCK foo_t
VAR
    refArr AT arr : ARRAY[1..3] OF DINT;
END_VAR
    arr[1] := 3;
    arr[2] := 2;
    arr[3] := 1;

    // CHECK: 1, 2, 3
    printf('%d, %d, %d$N', refArr[3], refArr[2], refArr[1]);
END_FUNCTION_BLOCK

FUNCTION_BLOCK bar_t
VAR
    foo: foo_t;
END_VAR
    foo();
END_FUNCTION_BLOCK

PROGRAM baz
VAR
    refArr AT arr : ARRAY[1..3] OF DINT;
    bar: bar_t;
END_VAR
    arr[1] := 3;
    arr[2] := 2;
    arr[3] := 1;

    // CHECK: 1, 2, 3
    printf('%d, %d, %d$N', refArr[3], refArr[2], refArr[1]);

    bar();
END_PROGRAM

FUNCTION main: DINT
    __init___nested_pous();
    baz();
END_FUNCTION