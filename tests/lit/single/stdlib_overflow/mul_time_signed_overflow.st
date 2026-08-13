// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that MUL with TIME and a signed integer wraps on overflow instead of panicking

FUNCTION main : DINT
VAR
    a : TIME;
END_VAR
    // 1728000000ms * 200 = 345600000000ms wraps modulo 2^32 to 2002616320ms
    a := MUL(TIME#20d, LINT#200);
    // CHECK: result: 2002616320
    printf('result: %u$N', a);
    main := 0;
END_FUNCTION
