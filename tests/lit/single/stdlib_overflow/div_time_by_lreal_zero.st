// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that DIV with TIME by LREAL zero yields zero instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    a := DIV(TIME#10s, LREAL#0.0);
    // CHECK: result: 0
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
