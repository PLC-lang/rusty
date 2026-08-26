// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that DATE_AND_TIME_TO_DATE saturates at the DATE range minimum for
// values on the first representable day (its midnight lies below the range)
// instead of aborting the process

FUNCTION main : DINT
VAR
    a : DATE;
    b : DATE;
END_VAR
    a := DATE_AND_TIME_TO_DATE(DT#1677-09-21-12:00:00);
    // CHECK: -9223372036854775808
    printf('%lld$N', a);

    b := DATE_AND_TIME_TO_DATE(DT#2024-05-05-13:30:15);
    IF b = DATE#2024-05-05 THEN
        // CHECK: midnight_ok
        printf('midnight_ok$N');
    END_IF

    main := 0;
END_FUNCTION
