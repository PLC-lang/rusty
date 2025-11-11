// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test IEC 61131-3 syntax: TYPE NAME : TYPE (variants) := default;
TYPE Status : DWORD (
    inactive := 16#00000000,
    active := 16#00000001,
    error := 16#FFFFFFFF
) := inactive;
END_TYPE

VAR_GLOBAL
    s_global: Status;  // Should initialize to inactive (explicit default)
END_VAR

PROGRAM prog
VAR
    s1 : Status;  // Should initialize to inactive (explicit default)
    s2 : Status := active;
    s3 : Status := error;
END_VAR
    // CHECK: 0
    printf('%d$N', s1);  // inactive (default)

    // CHECK: 1
    printf('%d$N', s2);  // active

    // CHECK: -1
    printf('%d$N', s3);  // error (as signed)

    // CHECK: 0
    printf('%d$N', s_global);  // inactive (default)
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
