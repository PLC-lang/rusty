// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and a signed integer wraps mod 2^64 on overflow

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    a := MUL(TIME#2ms, LINT#9223372036854775807);
    // CHECK: 4294967294
    printf('%u$N', a);

    main := 0;
END_FUNCTION
