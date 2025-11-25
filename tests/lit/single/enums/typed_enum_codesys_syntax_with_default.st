// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test Codesys syntax: TYPE NAME : (variants) TYPE := default;
TYPE Priority : (
    low := 10,
    medium := 20,
    high := 30
) INT := medium;
END_TYPE

VAR_GLOBAL
    p_global: Priority;  // Should initialize to medium (explicit default)
END_VAR

PROGRAM prog
VAR
    p1 : Priority;  // Should initialize to medium (explicit default)
    p2 : Priority := high;
END_VAR
    // CHECK: 20
    printf('%d$N', p1);  // medium (default)

    // CHECK: 30
    printf('%d$N', p2);  // high

    // CHECK: 20
    printf('%d$N', p_global);  // medium (default)
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
