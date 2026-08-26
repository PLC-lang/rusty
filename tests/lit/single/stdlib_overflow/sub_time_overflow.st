// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that SUB with TIME and DATE wraps mod 2^32 on underflow, matching CODESYS

FUNCTION main : DINT
VAR
    a : TIME;
    b : TIME;
    c : TIME;
END_VAR
    a := SUB(TIME#1s, TIME#5s);
    // CHECK: 4294963296
    printf('%u$N', a);

    b := SUB_TIME(TIME#2s, TIME#5s);
    // CHECK: 4294964296
    printf('%u$N', b);

    // 152 days exceed the TIME range in milliseconds; the result wraps
    c := SUB_DATE_DATE(DATE#2024-06-01, DATE#2024-01-01);
    // CHECK: 247898112
    printf('%u$N', c);

    main := 0;
END_FUNCTION
