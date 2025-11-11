// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL CONSTANT
    MYCONST: INT := 0;
END_VAR

// Test enum with no explicit default and a zero variant that is NOT also the first variant - BUT instead of a literal use a constant to initialize
TYPE Status : INT (
    active := 1,
    inactive := MYCONST,
    error := 2
);
END_TYPE

VAR_GLOBAL
    s_global: Status;  // Should initialize to inactive
END_VAR

PROGRAM prog
VAR
    s1 : Status;  // Should initialize to inactive
END_VAR
    // CHECK: 0
    printf('%d$N', s1);  // inactive

    // CHECK: 0
    printf('%d$N', s_global);  // inactive
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
