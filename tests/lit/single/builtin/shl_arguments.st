// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// SHL gives the same result for the implicit (positional) and the explicit (named) call form, in
// declaration order and reordered. A call may also mix both forms: a named argument claims the slot
// its name declares, and the positional arguments fill the remaining slots in the order they are
// written.

FUNCTION main
VAR
    x : DINT := 9;
    a : DINT;
    b : DINT;
    c : DINT;
END_VAR

a := SHL(x, 2);
b := SHL(IN := x, n := 2);
c := SHL(n := 2, IN := x);
// CHECK: SHL=36 36 36
printf('SHL=%d %d %d$N', a, b, c);

// named argument before the positional one, IN is filled by x
a := SHL(n := 2, x);
// CHECK: SHL-named-first=36
printf('SHL-named-first=%d$N', a);
END_FUNCTION
