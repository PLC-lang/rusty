// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that a POU-local VAR CONSTANT can be used as the multiplier
// in an array initializer with the [(CONST)(value)] syntax.

FUNCTION main : DINT
VAR CONSTANT
    N     : DINT := 5;
    COUNT : DINT := 3;
END_VAR
VAR
    arr1 : ARRAY [0..4] OF DINT := [(N)(77)];
    arr2 : ARRAY [1..3] OF REAL := [(COUNT)(1.25)];
END_VAR
    printf('%d$N', arr1[0]);  // CHECK: 77
    printf('%d$N', arr1[2]);  // CHECK: 77
    printf('%d$N', arr1[4]);  // CHECK: 77

    printf('%.2f$N', arr2[1]);  // CHECK: 1.25
    printf('%.2f$N', arr2[3]);  // CHECK: 1.25

    main := 0;
END_FUNCTION
