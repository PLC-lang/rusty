// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that arrays initialized with variable references (non-constant
// expressions) are correctly lowered into indexed assignments.
FUNCTION main : DINT
VAR
    x : DINT := 42;
    y : DINT := 7;
    arr_mult : ARRAY[0..2] OF DINT := [3(x)];
    arr_list : ARRAY[0..3] OF DINT := [x, y, x, y];
END_VAR
    // Multiplied: all get value of x at init time
    printf('%d$N', arr_mult[0]);  // CHECK: 42
    printf('%d$N', arr_mult[1]);  // CHECK: 42
    printf('%d$N', arr_mult[2]);  // CHECK: 42

    // Expression list: alternating x and y
    printf('%d$N', arr_list[0]);  // CHECK: 42
    printf('%d$N', arr_list[1]);  // CHECK: 7
    printf('%d$N', arr_list[2]);  // CHECK: 42
    printf('%d$N', arr_list[3]);  // CHECK: 7
    main := 0;
END_FUNCTION
