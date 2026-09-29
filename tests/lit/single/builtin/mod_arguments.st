// RUN: %COMPILE %s -lm && %RUN | %CHECK %s
//
// MOD gives the same result for the implicit (positional) and the explicit (named) call form, in
// declaration order and reordered. MOD gives the same result as the `MOD` operator, for integer as
// well as for REAL values. Without optimization a REAL remainder calls `fmodf`, thus the test links libm.

FUNCTION main
VAR
    x : DINT := 9;
    y : DINT := 7331;
    a : DINT;
    b : DINT;
    c : DINT;
    r : REAL := 7.5;
    s : REAL := 2.0;
    q : REAL;
END_VAR

a := MOD(y, x);
b := MOD(IN1 := y, IN2 := x);
c := MOD(IN2 := x, IN1 := y);
// CHECK: MOD=5 5 5 5
printf('MOD=%d %d %d %d$N', a, b, c, y MOD x);

q := MOD(r, s);
// CHECK: MOD-real=1.5 1.5
printf('MOD-real=%.1f %.1f$N', q, r MOD s);
END_FUNCTION
