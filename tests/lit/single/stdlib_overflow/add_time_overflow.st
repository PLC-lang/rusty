// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that ADD with TIME wraps mod 2^64 on overflow.
// The maximum TIME value is built at runtime because TIME literals saturate
// at 4294967295ns in this parser.

FUNCTION main : DINT
VAR
    max : TIME;
    a : TIME;
    b : TIME;
END_VAR
    max := MUL(TIME#1ns, LINT#9223372036854775807);
    a := ADD(max, TIME#1ns);
    // CHECK: -9223372036854775808
    printf('%lld$N', a);

    b := ADD_TIME(max, TIME#10ns);
    // CHECK: -9223372036854775799
    printf('%lld$N', b);

    main := 0;
END_FUNCTION
