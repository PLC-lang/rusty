// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression for #1443: pointer arithmetic on a paren-REF result followed by
// a deref must read the adjacent slot, not segfault.

FUNCTION main : DINT
VAR
    arr : ARRAY[0..1] OF DINT := [10, 20];
    out_val : DINT;
END_VAR
    out_val := (REF(arr[0]) + 1)^;
    // CHECK: 20
    printf('%d$N', out_val);
    main := 0;
END_FUNCTION
