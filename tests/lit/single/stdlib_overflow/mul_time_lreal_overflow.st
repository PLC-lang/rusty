// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and LREAL saturates on overflow instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    // The product saturates at the LINT maximum; its low 32 bits are all ones
    a := MUL(TIME#1s, LREAL#1.0E300);
    // CHECK: result: 4294967295
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
