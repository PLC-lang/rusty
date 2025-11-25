// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test enum without zero variant and NO explicit default - should init to first element
TYPE Priority : INT (
    low := 10,
    medium := 20,
    high := 30
);
END_TYPE

VAR_GLOBAL
    p_global: Priority;  // Should initialize to low (first element)
END_VAR

PROGRAM prog
VAR
    p1 : Priority;  // Should initialize to low (first element)
    p2 : Priority := high;
END_VAR
    // CHECK: 10
    printf('%d$N', p1);  // low (first element default)

    // CHECK: 30
    printf('%d$N', p2);  // high

    // CHECK: 10
    printf('%d$N', p_global);  // low (first element default)
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
