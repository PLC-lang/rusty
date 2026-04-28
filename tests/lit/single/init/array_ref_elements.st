// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that arrays initialized with REF() (runtime function calls) are
// correctly lowered into indexed assignments.
FUNCTION main : DINT
VAR
    a : DINT := 10;
    b : DINT := 20;
    c : DINT := 30;
    arr_mult : ARRAY[0..2] OF REF_TO DINT := [3(REF(a))];
    arr_list : ARRAY[0..2] OF REF_TO DINT := [REF(a), REF(b), REF(c)];
END_VAR
    // Multiplied: all point to a
    printf('%d$N', arr_mult[0]^);  // CHECK: 10
    printf('%d$N', arr_mult[1]^);  // CHECK: 10
    printf('%d$N', arr_mult[2]^);  // CHECK: 10

    // Expression list: each points to a different variable
    printf('%d$N', arr_list[0]^);  // CHECK: 10
    printf('%d$N', arr_list[1]^);  // CHECK: 20
    printf('%d$N', arr_list[2]^);  // CHECK: 30

    // Verify they are actual pointers (mutation through ref)
    arr_list[1]^ := 99;
    printf('%d$N', b);  // CHECK: 99
    main := 0;
END_FUNCTION
