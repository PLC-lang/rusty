// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A global constant whose initializer overflows its type aborts code generation.
VAR_GLOBAL CONSTANT
    c1 : USINT := 200;
    c2 : SINT := 0 - c1;
END_VAR

VAR_GLOBAL
    g : SINT := 0 - c1;
END_VAR

FUNCTION main : DINT
    printf('variable SINT := 0 - 200 = %d$N', g);           // CHECK: variable SINT := 0 - 200 = 56
    printf('constant SINT := 0 - 200 = %d$N', c2);          // CHECK-NEXT: constant SINT := 0 - 200 = 56
END_FUNCTION
