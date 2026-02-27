// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// XFAIL: *
// Test that MUL with TIME and signed integer overflow causes a panic

PROGRAM main
VAR
    a : TIME;
END_VAR
    // Multiplying TIME by max LINT value should overflow and panic
    a := MUL(TIME#10ns, LINT#9223372036854775807);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
