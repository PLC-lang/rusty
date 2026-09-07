// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and an oversized REAL factor saturates at the TIME range
// and a NaN factor yields zero

FUNCTION main : DINT
VAR
    a : TIME;
    b : TIME;
    r : REAL;
END_VAR
    a := MUL(TIME#1000d, REAL#3.4e38);
    // CHECK: 9223372036854775807
    printf('%lld$N', a);

    r := 0.0;
    b := MUL(TIME#1s, r / r);
    // CHECK: 0
    printf('%lld$N', b);

    main := 0;
END_FUNCTION
