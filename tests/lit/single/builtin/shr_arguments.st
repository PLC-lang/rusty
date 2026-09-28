// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// SHR gives the same result for the implicit (positional) and the explicit (named) call form, in
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

a := SHR(x, 2);
b := SHR(IN := x, n := 2);
c := SHR(n := 2, IN := x);
// CHECK: SHR=2 2 2
printf('SHR=%d %d %d$N', a, b, c);

// named argument before the positional one, IN is filled by x
a := SHR(n := 2, x);
// CHECK: SHR-named-first=2
printf('SHR-named-first=%d$N', a);
END_FUNCTION
