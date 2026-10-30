// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// The loop stores counter + step in the counter's own width before the end test, so a step past the type's limit wraps and the loop goes on.
FUNCTION main : DINT
    VAR
        us : USINT;
        ui : UINT;
        ud : UDINT;
        s : SINT;
        n : DINT;
    END_VAR

    // Negative step below 0
    n := 0;
    FOR us := 10 TO 1 BY -3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('USINT 10 TO 1 BY -3 = %d$N', n);                // CHECK: USINT 10 TO 1 BY -3 = 4

    n := 0;
    FOR ui := 10 TO 1 BY -3 DO
        n := n + 1;
        IF n > 100000 THEN EXIT; END_IF
    END_FOR
    printf('UINT 10 TO 1 BY -3 = %d$N', n);                 // CHECK-NEXT: UINT 10 TO 1 BY -3 = 4

    n := 0;
    FOR us := 10 TO 0 BY -1 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('USINT 10 TO 0 BY -1 = %d$N', n);                // CHECK-NEXT: USINT 10 TO 0 BY -1 = 11

    // Positive step past the maximum
    n := 0;
    FOR us := 250 TO 255 BY 3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('USINT 250 TO 255 BY 3 = %d$N', n);              // CHECK-NEXT: USINT 250 TO 255 BY 3 = 2

    n := 0;
    FOR us := 200 TO 250 BY 25 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('USINT 200 TO 250 BY 25 = %d$N', n);             // CHECK-NEXT: USINT 200 TO 250 BY 25 = 3

    n := 0;
    FOR ui := 65530 TO 65535 BY 4 DO
        n := n + 1;
        IF n > 100000 THEN EXIT; END_IF
    END_FOR
    printf('UINT 65530 TO 65535 BY 4 = %d$N', n);           // CHECK-NEXT: UINT 65530 TO 65535 BY 4 = 2

    n := 0;
    FOR ud := 4294967290 TO 4294967295 BY 3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('UDINT 4294967290 TO 4294967295 BY 3 = %d$N', n);    // CHECK-NEXT: UDINT 4294967290 TO 4294967295 BY 3 = 2

    n := 0;
    FOR s := 120 TO 127 BY 5 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('SINT 120 TO 127 BY 5 = %d$N', n);               // CHECK-NEXT: SINT 120 TO 127 BY 5 = 2
END_FUNCTION
