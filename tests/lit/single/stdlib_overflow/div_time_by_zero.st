// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that DIV with TIME by zero causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Dividing TIME by zero should panic
    a := DIV(TIME#10ns, LINT#0);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
