// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and an oversized LREAL factor saturates at the TIME range
// and a NaN factor yields zero

FUNCTION main : DINT
VAR
    a : TIME;
    b : TIME;
    lr : LREAL;
END_VAR
    a := MUL(TIME#1000d, LREAL#1.7e308);
    // CHECK: 9223372036854775807
    printf('%lld$N', a);

    lr := 0.0;
    b := MUL(TIME#1s, lr / lr);
    // CHECK: 0
    printf('%lld$N', b);

    main := 0;
END_FUNCTION
