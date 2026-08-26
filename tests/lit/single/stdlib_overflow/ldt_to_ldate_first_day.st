// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that LDATE_AND_TIME_TO_LDATE saturates at the LDATE range minimum for
// values on the first representable day (its midnight lies below the range)
// instead of aborting the process

FUNCTION main : DINT
VAR
    a : LDATE;
    b : LDATE;
END_VAR
    a := LDATE_AND_TIME_TO_LDATE(LDT#1677-09-21-12:00:00);
    IF a = LDT#1677-09-21-00:12:43.145224192 THEN
        // CHECK: saturated_ok
        printf('saturated_ok$N');
    END_IF

    b := LDATE_AND_TIME_TO_LDATE(LDT#2024-05-05-13:30:15);
    IF b = LDATE#2024-05-05 THEN
        // CHECK: midnight_ok
        printf('midnight_ok$N');
    END_IF

    main := 0;
END_FUNCTION
