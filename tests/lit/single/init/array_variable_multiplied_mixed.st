// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Tests that a non-constant variable multiplier works in a mixed expression
// list where subsequent elements use expression-based offsets.

FUNCTION main : DINT
VAR
    n : DINT := 3;
    arr : ARRAY [0..5] OF DINT := [(n)(10), 20, 30, 40];
END_VAR
    // First n elements filled by the FOR loop
    printf('%d$N', arr[0]);  // CHECK: 10
    printf('%d$N', arr[1]);  // CHECK: 10
    printf('%d$N', arr[2]);  // CHECK: 10
    // Subsequent elements at offsets n, n+1, n+2
    printf('%d$N', arr[3]);  // CHECK: 20
    printf('%d$N', arr[4]);  // CHECK: 30
    printf('%d$N', arr[5]);  // CHECK: 40

    main := 0;
END_FUNCTION
