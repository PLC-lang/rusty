// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and an unsigned integer beyond the LINT range wraps instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    // The factor wraps to LINT minimum in modular 64-bit arithmetic; the low
    // 32 bits of the product are zero
    a := MUL(TIME#1ms, ULINT#9223372036854775808);
    // CHECK: result: 0
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
