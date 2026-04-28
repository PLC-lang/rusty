// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE tarr : ARRAY[10..19] OF DINT := [10(77)]; END_TYPE

FUNCTION main : DINT
VAR
    arr : tarr;
END_VAR
    printf('%d$N', arr[10]);  // CHECK: 77
    printf('%d$N', arr[14]);  // CHECK: 77
    printf('%d$N', arr[19]);  // CHECK: 77
    main := 0;
END_FUNCTION
