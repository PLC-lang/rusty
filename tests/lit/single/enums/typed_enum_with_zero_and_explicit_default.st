// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test enum with zero variant BUT explicit default - should use explicit default
TYPE Status : INT (
    inactive := 0,
    active := 1,
    error := 2
) := active;
END_TYPE

VAR_GLOBAL
    s_global: Status;  // Should initialize to active (explicit default)
END_VAR

PROGRAM prog
VAR
    s1 : Status;  // Should initialize to active (explicit default), not inactive
    s2 : Status := error;
END_VAR
    // CHECK: 1
    printf('%d$N', s1);  // active (explicit default overrides zero)

    // CHECK: 2
    printf('%d$N', s2);  // error

    // CHECK: 1
    printf('%d$N', s_global);  // active (explicit default)
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
