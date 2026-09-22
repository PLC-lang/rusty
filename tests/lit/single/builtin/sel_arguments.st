// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// SEL gives the same result for the implicit (positional) and the explicit (named) call form, in
// declaration order and reordered. A call may also mix both forms: a named argument claims the slot
// its name declares, and the positional arguments fill the remaining slots in the order they are
// written.

FUNCTION main
VAR
    x : DINT := 9;
    y : DINT := 7331;
    a : DINT;
    b : DINT;
    c : DINT;
END_VAR

a := SEL(TRUE, x, y);
b := SEL(G := TRUE, IN0 := x, IN1 := y);
c := SEL(IN1 := y, IN0 := x, G := TRUE);
// CHECK: SEL=7331 7331 7331
printf('SEL=%d %d %d$N', a, b, c);

// named argument before the positional ones, G and IN0 are filled by FALSE and x
a := SEL(IN1 := y, FALSE, x);
// CHECK: SEL-mixed=9
printf('SEL-mixed=%d$N', a);
END_FUNCTION
