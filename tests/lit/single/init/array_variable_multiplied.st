// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that a non-constant variable can be used as the multiplier
// in an array initializer with the [(var)(value)] syntax.

FUNCTION main : DINT
VAR
    n : DINT := 5;
    arr : ARRAY [0..4] OF DINT := [(n)(77)];
END_VAR
    printf('%d$N', arr[0]);  // CHECK: 77
    printf('%d$N', arr[2]);  // CHECK: 77
    printf('%d$N', arr[4]);  // CHECK: 77

    main := 0;
END_FUNCTION
