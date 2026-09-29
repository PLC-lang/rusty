// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// LOWER_BOUND gives the same result for the implicit (positional) and the explicit (named) call form,
// in declaration order and reordered.

FUNCTION main
VAR
    arr : ARRAY[0..9] OF BYTE := [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
END_VAR

bounds(arr);
END_FUNCTION

FUNCTION bounds : DINT
VAR_IN_OUT
    vla : ARRAY[*] OF BYTE;
END_VAR
VAR
    dim : DINT := 1;
    a   : DINT;
    b   : DINT;
    c   : DINT;
END_VAR

a := LOWER_BOUND(vla, dim);
b := LOWER_BOUND(arr := vla, dim := dim);
c := LOWER_BOUND(dim := dim, arr := vla);
// CHECK: LOWER_BOUND=0 0 0
printf('LOWER_BOUND=%d %d %d$N', a, b, c);
END_FUNCTION
