// Issue #921: Array field initialization from another array in struct literal
// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE myStruct : STRUCT
    var1 : DINT;
    var2 : ARRAY[1..2] OF DINT;
END_STRUCT
END_TYPE

FUNCTION main : DINT
VAR
    arr_init : ARRAY[1..2] OF DINT := [1, 2];
    x : myStruct;
END_VAR
    // Initialize array field from another array (regression for #921)
    x := (var1 := 1, var2 := arr_init);

    printf('v1=%d$N', x.var1); // CHECK: v1=1
    printf('v2_1=%d$N', x.var2[1]); // CHECK: v2_1=1
    printf('v2_2=%d$N', x.var2[2]); // CHECK: v2_2=2

    main := 0;
END_FUNCTION
