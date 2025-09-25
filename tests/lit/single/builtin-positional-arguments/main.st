// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// Test cases for builtin functions with positional arguments
//
// This test verifies that builtin functions work correctly with positional arguments:
// - MOVE: positional arguments
// - SEL: positional (TRUE, b, c) arguments
// - MUX: positional arguments (NOTE: MUX does not support named arguments)
// - SIZEOF: positional (myarray) arguments
// - UPPER_BOUND: positional (vla, x) arguments
// - LOWER_BOUND: positional (vla, x) arguments
// - ADR: Both positional and named arguments (to verify equivalence)
// - REF: Both positional and named arguments (to verify equivalence)
// - DIV: positional arguments
// - SUB: positional arguments
//
// For ADR and REF, both calling conventions are tested to verify they produce equivalent results.

FUNCTION main : DINT
VAR
    result      : ULINT;
    x           : DINT := 9;
    y           : DINT := 7331;

    a           : DINT := 0;
    b           : DINT := 3;
    c           : DINT := 4;

    piAddress1  : REF_TO INT;
    piAddress2  : REF_TO INT;
    iVar1       : INT := 5;

    myarray     : ARRAY [0..9] OF BYTE := [0,1,2,3,4,5,6,7,8,9];

    dividend    : DINT := 20;
    divisor     : DINT := 4;
    subtrahend  : DINT := 7;
END_VAR

    // MOVE
    printf('%d$N', y);                      // CHECK: 7331
    x := MOVE(y);
    printf('%d$N', x);                      // CHECK: 7331

    // SEL
    a := SEL(TRUE, b, c);
    printf('%d$N', a);                      // CHECK: 4

    // MUX
    result := MUX(1, a, b, c);
    printf('%d$N', result);                 // CHECK: 3

    // SIZEOF
    result := SIZEOF(myarray);
    printf('%d$N', result);                 // CHECK: 10

    // ADR - test both positional and named to verify equivalence
    result := 0;
    printf('%d$N', result);                 // CHECK: 0
    piAddress1 := ADR(iVar1);
    piAddress2 := ADR(IN := iVar1);
    result := (piAddress1 = piAddress2);
    printf('%d$N', result);                 // CHECK: 1

    // REF - test both positional and named to verify equivalence
    result := 0;
    printf('%d$N', result);                 // CHECK: 0
    piAddress1 := REF(iVar1);               // positional
    piAddress2 := REF(in := iVar1);         // named argument
    result := (piAddress1 = piAddress2);
    printf('%d$N', result);                 // CHECK: 1

    // DIV
    result := DIV(dividend, divisor);
    printf('%d$N', result);                 // CHECK: 5

    // SUB
    result := SUB(dividend, subtrahend);
    printf('%d$N', result);                 // CHECK: 13

    test_bounds(myarray);

    main := 0;

END_FUNCTION

FUNCTION test_bounds : DINT
VAR_IN_OUT
    vla : ARRAY [*] OF BYTE;
END_VAR
VAR
    result  : DINT;
    x       : DINT := 1;
END_VAR

    // UPPER_BOUND
    result := UPPER_BOUND(vla, x);
    printf('%d$N', result);                 // CHECK: 9

    // LOWER_BOUND
    result := LOWER_BOUND(vla, x);
    printf('%d$N', result);                 // CHECK: 0

END_FUNCTION
