// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE tarr : ARRAY[0..9] OF DINT := [5(42)]; END_TYPE

FUNCTION main : DINT
VAR
    arr : tarr;
END_VAR
    // First 5 elements should be 42
    printf('%d$N', arr[0]);  // CHECK: 42
    printf('%d$N', arr[4]);  // CHECK: 42
    // Remaining elements should be 0 (default)
    printf('%d$N', arr[5]);  // CHECK: 0
    printf('%d$N', arr[9]);  // CHECK: 0
    main := 0;
END_FUNCTION
