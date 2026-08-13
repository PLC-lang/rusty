// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and REAL saturates on overflow instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    // The product saturates at the LINT maximum; its low 32 bits are all ones
    a := MUL(TIME#1s, REAL#3.40282347e38);
    // CHECK: result: 4294967295
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
