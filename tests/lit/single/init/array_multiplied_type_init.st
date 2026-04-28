// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE tarr : ARRAY[0..99] OF DINT := [100(42)]; END_TYPE

FUNCTION main : DINT
VAR
    arr : tarr;
END_VAR
    // Check first, middle, and last elements
    printf('%d$N', arr[0]);   // CHECK: 42
    printf('%d$N', arr[49]);  // CHECK: 42
    printf('%d$N', arr[99]);  // CHECK: 42
    main := 0;
END_FUNCTION
