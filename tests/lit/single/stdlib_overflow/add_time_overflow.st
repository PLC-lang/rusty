// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that ADD with TIME wraps mod 2^32 on overflow, matching CODESYS

FUNCTION main : DINT
VAR
    a : TIME;
    b : TIME;
END_VAR
    a := ADD(TIME#4294967295ms, TIME#1ms);
    // CHECK: 0
    printf('%u$N', a);

    b := ADD_TIME(TIME#49d17h2m47s294ms, TIME#10s);
    // CHECK: 9998
    printf('%u$N', b);

    main := 0;
END_FUNCTION
