// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE tarr : ARRAY[0..9] OF DINT := [5(10), 3(20), 99, 100]; END_TYPE

FUNCTION main : DINT
VAR
    arr : tarr;
END_VAR
    // First segment: indices 0..4 should be 10
    printf('%d$N', arr[0]);  // CHECK: 10
    printf('%d$N', arr[4]);  // CHECK: 10
    // Second segment: indices 5..7 should be 20
    printf('%d$N', arr[5]);  // CHECK: 20
    printf('%d$N', arr[7]);  // CHECK: 20
    // Individual values at indices 8 and 9
    printf('%d$N', arr[8]);  // CHECK: 99
    printf('%d$N', arr[9]);  // CHECK: 100
    main := 0;
END_FUNCTION
