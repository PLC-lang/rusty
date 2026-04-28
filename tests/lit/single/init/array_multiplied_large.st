// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE tarr : ARRAY[0..9999] OF DINT := [10000(7)]; END_TYPE

FUNCTION main : DINT
VAR
    arr : tarr;
END_VAR
    // Spot-check first, middle, and last elements
    printf('%d$N', arr[0]);     // CHECK: 7
    printf('%d$N', arr[4999]);  // CHECK: 7
    printf('%d$N', arr[9999]);  // CHECK: 7
    main := 0;
END_FUNCTION
