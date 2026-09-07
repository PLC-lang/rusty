// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that SUB with TIME wraps mod 2^64 on underflow and SUB_DATE_DATE
// yields the plain signed difference.
// The maximum TIME value is built at runtime because TIME literals saturate
// at 4294967295ns in this parser.

FUNCTION main : DINT
VAR
    max : TIME;
    a : TIME;
    b : TIME;
    c : TIME;
END_VAR
    // TIME is signed, so small deficits stay plain negative values
    a := SUB(TIME#1s, TIME#5s);
    // CHECK: -4000000000
    printf('%lld$N', a);

    // pushing below the range minimum wraps to the top of the range
    max := MUL(TIME#1ns, LINT#9223372036854775807);
    b := SUB_TIME(SUB(TIME#0s, max), TIME#2ns);
    // CHECK: 9223372036854775807
    printf('%lld$N', b);

    c := SUB_DATE_DATE(DATE#2024-06-01, DATE#2024-01-01);
    // CHECK: 13132800000000000
    printf('%lld$N', c);

    main := 0;
END_FUNCTION
