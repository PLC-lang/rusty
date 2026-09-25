// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// SIZEOF gives the same result for the implicit (positional) and the explicit (named) call form.

FUNCTION main
VAR
    arr : ARRAY[0..9] OF BYTE := [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    a   : DINT;
    b   : DINT;
END_VAR

a := SIZEOF(arr);
b := SIZEOF(in := arr);
// CHECK: SIZEOF=10 10
printf('SIZEOF=%d %d$N', a, b);
END_FUNCTION
