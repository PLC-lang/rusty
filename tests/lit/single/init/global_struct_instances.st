// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    arr : ARRAY[1..3] OF DINT := [3, 2, 1];
    foo: foo_t;
END_VAR

FUNCTION_BLOCK foo_t
VAR
    refArr AT arr : ARRAY[1..3] OF DINT;
END_VAR
    // CHECK: 1, 2, 3
    printf('%d, %d, %d$N', refArr[3], refArr[2], refArr[1]);
END_FUNCTION_BLOCK

FUNCTION main: DINT
    foo();
END_FUNCTION
