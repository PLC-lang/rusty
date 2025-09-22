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
    result  : ULINT;
    x       : DINT := 9;
    y       : DINT := 7331;

    a : DINT := 0;
    b : DINT := 3;
    c : DINT := 4;

    piAddress1: REF_TO INT;
    iVar1 : INT := 5;

    myarray     : ARRAY [0..9] OF BYTE := [0,1,2,3,4,5,6,7,8,9];
END_VAR

printf('%d$N', y);        // CHECK: 7331
y := MOVE(IN := x);
printf('%d$N', y);        // CHECK: 9
x := MOVE(y);
printf('%d$N', y);        // CHECK: 9


a := SEL(G := TRUE, IN0 := b, IN1 := c);
printf('%d$N', a);        // CHECK: 4
a := 0;
a := SEL(TRUE, b, c);
printf('%d$N', a);        // CHECK: 4

result := MUX(1, a, b, c);

result := SIZEOF(in := myarray);
printf('%d$N', result);        // CHECK: 10

result := SIZEOF(myarray);
printf('%d$N', result);        // CHECK: 10

piAddress1 := ADR(iVar1);
piAddress1 := ADR(IN := iVar1);

// Test REF builtin with both positional and named arguments
piAddress1 := REF(iVar1);           // positional
piAddress1 := REF(in := iVar1);     // named argument

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
result := UPPER_BOUND( vla, x);
printf('%d$N', result);        // CHECK: 9

result := 999;

// and with named arguments
result := UPPER_BOUND(arr := vla, dim := x);
printf('%d$N', result);        // CHECK: 9

// check with positional arguments
result := LOWER_BOUND( vla, x);
printf('%d$N', result);        // CHECK: 0

result := 999;

// and with named arguments
result := LOWER_BOUND(arr := vla, dim := x);
printf('%d$N', result);        // CHECK: 0
END_FUNCTION
