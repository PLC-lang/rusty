// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that ADD with TIME wraps on overflow instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    // 4294967294ms + 10000ms wraps modulo 2^32 to 9998ms
    a := ADD(TIME#49d17h2m47s294ms, TIME#10s);
    // CHECK: result: 9998
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
