// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// ABS gives the same result for the implicit (positional) and the explicit (named) call form.

FUNCTION main
VAR
    neg : DINT := -42;
    a   : DINT;
    b   : DINT;
END_VAR

a := ABS(neg);
b := ABS(IN := neg);
// CHECK: ABS=42 42
printf('ABS=%d %d$N', a, b);
END_FUNCTION
