// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that an array with constant-expression bounds and a constant-multiplied
// REAL initializer compiles and runs correctly.

VAR_GLOBAL CONSTANT
    NB_BOOL   : DINT := 12;
    NB_ANALOG : DINT := 2;
END_VAR

FUNCTION main : DINT
VAR
    MAX_TIME_BOOL   : ARRAY [1..NB_BOOL] OF REAL := [(NB_BOOL)(0.0033)];
    MAX_TIME_ANALOG : ARRAY [1..NB_ANALOG] OF REAL := [(NB_ANALOG)(1.5)];
END_VAR
    // Check first, middle, and last elements of MAX_TIME_BOOL
    printf('%.4f$N', MAX_TIME_BOOL[1]);   // CHECK: 0.0033
    printf('%.4f$N', MAX_TIME_BOOL[6]);   // CHECK: 0.0033
    printf('%.4f$N', MAX_TIME_BOOL[12]);  // CHECK: 0.0033

    // Check both elements of MAX_TIME_ANALOG
    printf('%.1f$N', MAX_TIME_ANALOG[1]);  // CHECK: 1.5
    printf('%.1f$N', MAX_TIME_ANALOG[2]);  // CHECK: 1.5

    main := 0;
END_FUNCTION
