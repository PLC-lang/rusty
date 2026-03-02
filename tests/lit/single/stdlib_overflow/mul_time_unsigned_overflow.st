// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that MUL with TIME and unsigned integer overflow causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Multiplying TIME by max ULINT value should overflow and panic
    a := MUL(TIME#1ns, ULINT#9223372036854775808);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
