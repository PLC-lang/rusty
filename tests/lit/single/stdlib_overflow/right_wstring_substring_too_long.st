
// RUN: (%COMPILE %s && %RUN 2>&1) | %CHECK %s
// XFAIL: *
// Test that RIGHT with substring length > string length causes a panic
// Expected failure message: Requested substring length exceeds string length.
// CHECK: Requested substring length exceeds string length.

PROGRAM main
VAR
    s : WSTRING;
END_VAR
    s := "sa𝄞ple text";
    // Length 12 exceeds string length 11 -> should panic
    s := RIGHT(s, 12);
    // CHECK: Should not reach here
    printf('Should not reach here$N');
END_PROGRAM
