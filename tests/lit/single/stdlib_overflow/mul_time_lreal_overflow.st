// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that MUL with TIME and LREAL overflow causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Multiplying TIME by large LREAL value should overflow and panic
    a := MUL(TIME#-2s700ms, LREAL#3.40282347e38);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
