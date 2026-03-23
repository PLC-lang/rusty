// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Verify that arrays with type-level initializers work correctly
// when declared in external blocks.
TYPE tarr : ARRAY[0..4] OF DINT := [5(77)]; END_TYPE

VAR_GLOBAL
    g_arr : tarr;
END_VAR

FUNCTION main : DINT
VAR
    local_arr : tarr;
END_VAR
    // Global instance should be initialized by the global ctor
    printf('%d$N', g_arr[0]);   // CHECK: 77
    printf('%d$N', g_arr[4]);   // CHECK: 77

    // Local instance should also be initialized via the type ctor
    printf('%d$N', local_arr[0]);  // CHECK: 77
    printf('%d$N', local_arr[4]);  // CHECK: 77
    main := 0;
END_FUNCTION
