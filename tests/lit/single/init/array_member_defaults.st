// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Struct defaults must be applied to every element of arrays of structs,
// including nested and multi-dimensional arrays.
TYPE Point : STRUCT
    x : DINT := 11;
    y : DINT := 22;
END_STRUCT END_TYPE

PROGRAM prog
VAR
    plain  : ARRAY[0..3] OF Point;
    nested : ARRAY[0..1] OF ARRAY[0..2] OF Point;
    multi  : ARRAY[0..1, 0..2] OF Point;
END_VAR
    printf('%d$N', plain[0].x);      // CHECK: 11
    printf('%d$N', plain[3].y);      // CHECK: 22
    printf('%d$N', nested[1][2].x);  // CHECK: 11
    printf('%d$N', nested[0][1].y);  // CHECK: 22
    printf('%d$N', multi[1, 2].x);   // CHECK: 11
    printf('%d$N', multi[0, 1].y);   // CHECK: 22
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
