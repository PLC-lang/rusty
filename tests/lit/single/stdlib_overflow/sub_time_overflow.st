// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that SUB with TIME overflow causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Subtracting from minimum TIME value should overflow and panic
    a := SUB(TIME#-9223372036854775807ms, TIME#1ms);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
