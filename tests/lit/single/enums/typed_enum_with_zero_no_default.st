// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test enum with zero variant and NO explicit default - should init to zero
TYPE Status : INT (
    inactive := 0,
    active := 1,
    error := 2
);
END_TYPE

VAR_GLOBAL
    s_global: Status;  // Should initialize to inactive (zero value)
END_VAR

PROGRAM prog
VAR
    s1 : Status;  // Should initialize to inactive (zero value)
    s2 : Status := active;
END_VAR
    // CHECK: 0
    printf('%d$N', s1);  // inactive (zero default)

    // CHECK: 1
    printf('%d$N', s2);  // active

    // CHECK: 0
    printf('%d$N', s_global);  // inactive (zero default)
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
