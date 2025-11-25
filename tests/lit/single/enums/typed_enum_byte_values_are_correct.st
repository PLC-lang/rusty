// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Test BYTE enum values work correctly
TYPE Status : BYTE (inactive := 0, active := 1, error := 255);
END_TYPE

VAR_GLOBAL
    status_global: Status := active;
END_VAR

PROGRAM prog
VAR
    s1 : Status := inactive;
    s2 : Status := active;
    s3 : Status := error;
END_VAR
    // CHECK: 0
    printf('%d$N', s1);  // inactive

    // CHECK: 1
    printf('%d$N', s2);  // active

    // CHECK: 255
    printf('%d$N', s3);  // error

    // CHECK: 1
    printf('%d$N', status_global);  // active
END_PROGRAM

FUNCTION main
    prog();
END_FUNCTION
