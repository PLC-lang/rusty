// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that DIV with TIME by LREAL zero causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Dividing TIME by zero (LREAL) should panic
    a := DIV(TIME#10ns, LREAL#0.0);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
