// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test enum without zero variant but WITH explicit default - should init to explicit default
TYPE PRIORITY : INT (
    low := 10,
    medium := 20,
    high := 30
) := medium;
END_TYPE

VAR_GLOBAL
    p_global: PRIORITY;
END_VAR

PROGRAM prog
VAR
    p1 : PRIORITY;  // Should initialize to medium (explicit default), not low
    p2 : PRIORITY := high;
END_VAR
    // CHECK: 20
    printf('%d$N', p1);  // Should be medium (explicit default)

    // CHECK: 30
    printf('%d$N', p2);  // high

    p1 := PRIORITY.low;
    // CHECK: 10
    printf('%d$N', p1);  // low

    // CHECK: 20
    printf('%d$N', p_global); // medium
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
