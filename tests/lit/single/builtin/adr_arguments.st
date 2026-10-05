// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// ADR returns the same address for the implicit (positional) and the explicit (named) call form.

FUNCTION main
VAR
    i : INT := 5;
    p : BOOL;
END_VAR

p := ADR(i) = ADR(in := i);
// CHECK: ADR=1
printf('ADR=%d$N', p);
END_FUNCTION
