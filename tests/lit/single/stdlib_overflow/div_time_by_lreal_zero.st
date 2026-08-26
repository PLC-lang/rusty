// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that DIV with TIME by LREAL zero saturates at the TIME range
// and a zero dividend yields zero

FUNCTION main : DINT
VAR
    a : TIME;
    b : TIME;
END_VAR
    a := DIV(TIME#1s, LREAL#0.0);
    // CHECK: 9223372036854775807
    printf('%lld$N', a);

    b := DIV(TIME#0s, LREAL#0.0);
    // CHECK: 0
    printf('%lld$N', b);

    main := 0;
END_FUNCTION
