// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that SUB with TIME wraps on underflow instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    // 1000ms - 5000ms wraps modulo 2^32 to 4294963296ms
    a := SUB(TIME#1s, TIME#5s);
    // CHECK: result: 4294963296
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
