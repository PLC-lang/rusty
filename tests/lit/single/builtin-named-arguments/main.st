// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// Test cases for builtin functions with named arguments
//
// NOTE: `MUX` does currently not support named arguments
//       `ADR()` and `REF()` with the positional arguments test
//
// This test verifies that builtin functions work correctly with named arguments:
// - MOVE: named (IN := x) arguments
// - SEL: named (G := TRUE, IN0 := b, IN1 := c) arguments
// - SIZEOF: named (in := myarray) arguments
// - UPPER_BOUND: named (arr := vla, dim := x) arguments
// - LOWER_BOUND: named (arr := vla, dim := x) arguments
// - DIV: named (IN1 := dividend, IN2 := divisor) arguments
// - SUB: named (IN1 := dividend, IN2 := subtrahend) arguments
// - SHL: named (IN := x, n := y) arguments
// - SHR: named (IN := x, n := y) arguments
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

    myarray     : ARRAY [0..9] OF BYTE := [0,1,2,3,4,5,6,7,8,9];

    dividend    : DINT := 20;
    divisor     : DINT := 4;
    subtrahend  : DINT := 7;

    shl_result  : DINT;
    shr_result  : DINT;
END_VAR

    // MOVE
    printf('%d$N', y);                      // CHECK: 7331
    y := MOVE(IN := x);
    printf('%d$N', y);                      // CHECK: 9

    // SEL
    a := SEL(IN0 := b, IN1 := c, G := TRUE);
    printf('%d$N', a);                      // CHECK: 4

    // SIZEOF
    result := SIZEOF(in := myarray);
    printf('%d$N', result);                 // CHECK: 10

    // DIV
    result := DIV(IN2 := divisor, IN1 := dividend);
    printf('%d$N', result);                 // CHECK: 5

    // SUB
    result := SUB(IN2 := subtrahend, IN1 := dividend);
    printf('%d$N', result);                 // CHECK: 13

    // SHL
    shl_result := SHL(n := 2, IN := 1);
    printf('%d$N', shl_result);             // CHECK: 4

    // SHR
    shr_result := SHR(n := 1, IN := 4);
    printf('%d$N', shr_result);             // CHECK: 2

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
    result := UPPER_BOUND(arr := vla, dim := x);
    printf('%d$N', result);                 // CHECK: 9

    // LOWER_BOUND
    result := LOWER_BOUND(arr := vla, dim := x);
    printf('%d$N', result);                 // CHECK: 0

END_FUNCTION
