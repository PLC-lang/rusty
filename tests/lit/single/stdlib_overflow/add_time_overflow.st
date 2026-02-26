// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that ADD with TIME overflow causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Adding 1ms to max TIME value should overflow and panic
    a := ADD(TIME#9223372036854775807ms, TIME#1ms);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
