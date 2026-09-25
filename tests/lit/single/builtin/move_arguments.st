// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// MOVE gives the same result for the implicit (positional) and the explicit (named) call form.

FUNCTION main
VAR
    y : DINT := 7331;
    a : DINT;
    b : DINT;
END_VAR

a := MOVE(y);
b := MOVE(in := y);
// CHECK: MOVE=7331 7331
printf('MOVE=%d %d$N', a, b);
END_FUNCTION
