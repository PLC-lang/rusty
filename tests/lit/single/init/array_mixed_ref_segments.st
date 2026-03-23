// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests mixed multiplied segments with REF() calls.
FUNCTION main : DINT
VAR
    a : DINT := 1;
    b : DINT := 2;
    arr : ARRAY[0..4] OF REF_TO DINT := [2(REF(a)), REF(b), 2(REF(a))];
END_VAR
    printf('%d$N', arr[0]^);  // CHECK: 1
    printf('%d$N', arr[1]^);  // CHECK: 1
    printf('%d$N', arr[2]^);  // CHECK: 2
    printf('%d$N', arr[3]^);  // CHECK: 1
    printf('%d$N', arr[4]^);  // CHECK: 1
    main := 0;
END_FUNCTION
