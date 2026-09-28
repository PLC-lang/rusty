// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// DIV gives the same result for the implicit (positional) and the explicit (named) call form, in
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

a := DIV(y, x);
b := DIV(IN1 := y, IN2 := x);
c := DIV(IN2 := x, IN1 := y);
// CHECK: DIV=814 814 814
printf('DIV=%d %d %d$N', a, b, c);

// named argument before the positional one, IN1 is filled by y
a := DIV(IN2 := x, y);
// CHECK: DIV-named-first=814
printf('DIV-named-first=%d$N', a);
END_FUNCTION
