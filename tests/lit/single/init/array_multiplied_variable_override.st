// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE tarr : ARRAY[0..9] OF DINT := [10(42)]; END_TYPE

PROGRAM prog
VAR
    default_arr : tarr;
    override_arr : tarr := [10(99)];
END_VAR
    printf('%d$N', default_arr[0]);   // CHECK: 42
    printf('%d$N', default_arr[9]);   // CHECK: 42
    printf('%d$N', override_arr[0]);  // CHECK: 99
    printf('%d$N', override_arr[9]);  // CHECK: 99
END_PROGRAM

FUNCTION main : DINT
    prog();
    main := 0;
END_FUNCTION
