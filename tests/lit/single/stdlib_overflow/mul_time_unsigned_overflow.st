// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and an unsigned integer wraps mod 2^64 on overflow

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    a := MUL(TIME#1ms, ULINT#18446744073709551615);
    // CHECK: 4294967295
    printf('%u$N', a);

    main := 0;
END_FUNCTION
