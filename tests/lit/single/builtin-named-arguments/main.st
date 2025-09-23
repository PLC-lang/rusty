// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// Test cases for builtin functions with both named and positional arguments
//
// NOTE: `MUX` does currently not support named arguments
//
// This test verifies that builtin functions work correctly with:
// - MOVE: Both named (IN := x) and positional arguments
// - SEL: Both named (G := TRUE, IN0 := b, IN1 := c) and positional (TRUE, b, c) arguments
// - SIZEOF: Both named (in := myarray) and positional (myarray) arguments
// - UPPER_BOUND: Both named (arr := vla, dim := x) and positional (vla, x) arguments
// - LOWER_BOUND: Both named (arr := vla, dim := x) and positional (vla, x) arguments
// - ADR: Both named (in := variable) and positional (variable) arguments
// - REF: Both named (in := variable) and positional (variable) arguments
//
// The test ensures that generic type resolution works properly for both calling conventions.

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
    y := MOVE(IN := x);
    printf('%d$N', y);                      // CHECK: 9
    y := 7331;
    x := MOVE(y);
    printf('%d$N', y);                      // CHECK: 7331

    // SEL
    a := SEL(G := TRUE, IN0 := b, IN1 := c);
    printf('%d$N', a);                      // CHECK: 4
    a := 0;
    a := SEL(TRUE, b, c);
    printf('%d$N', a);                      // CHECK: 4

    result := MUX(1, a, b, c);
    printf('%d$N', result);                 // CHECK: 3

    // SIZEOF
    result := SIZEOF(in := myarray);
    printf('%d$N', result);                 // CHECK: 10
    result := SIZEOF(myarray);
    printf('%d$N', result);                 // CHECK: 10

    // ADR
    result := 0;
    printf('%d$N', result);                 // CHECK: 0
    piAddress1 := ADR(iVar1);
    piAddress2 := ADR(IN := iVar1);
    result := (piAddress1 = piAddress2);
    printf('%d$N', result);                 // CHECK: 1

    // REF
    result := 0;
    printf('%d$N', result);                 // CHECK: 0
    piAddress1 := REF(iVar1);               // positional
    piAddress2 := REF(in := iVar1);         // named argument
    result := (piAddress1 = piAddress2);
    printf('%d$N', result);                 // CHECK: 1

    // DIV
    result := DIV(IN1 := dividend, IN2 := divisor);
    printf('%d$N', result);                 // CHECK: 5
    result := DIV(dividend, divisor);
    printf('%d$N', result);                 // CHECK: 5

    // SUB
    result := SUB(IN1 := dividend, IN2 := subtrahend);
    printf('%d$N', result);                 // CHECK: 13
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

    // check with positional arguments
    result := UPPER_BOUND(vla, x);
    printf('%d$N', result);                 // CHECK: 9

    result := 999;

    // and with named arguments
    result := UPPER_BOUND(arr := vla, dim := x);
    printf('%d$N', result);                 // CHECK: 9

    // check with positional arguments
    result := LOWER_BOUND(vla, x);
    printf('%d$N', result);                 // CHECK: 0

    result := 999;

    // and with named arguments
    result := LOWER_BOUND(arr := vla, dim := x);
    printf('%d$N', result);                 // CHECK: 0

END_FUNCTION
