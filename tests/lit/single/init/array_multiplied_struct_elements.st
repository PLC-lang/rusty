// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE MyStruct : STRUCT
    a : DINT;
    b : DINT;
END_STRUCT END_TYPE

FUNCTION main : DINT
VAR
    arr : ARRAY[0..2] OF MyStruct := [3((a := 5, b := 10))];
END_VAR
    printf('%d %d$N', arr[0].a, arr[0].b);  // CHECK: 5 10
    printf('%d %d$N', arr[1].a, arr[1].b);  // CHECK: 5 10
    printf('%d %d$N', arr[2].a, arr[2].b);  // CHECK: 5 10
    main := 0;
END_FUNCTION
