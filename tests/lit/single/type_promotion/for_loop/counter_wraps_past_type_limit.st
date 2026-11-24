// RUN: %COMPILE %s && %RUN | %CHECK %s
// A step past the limit of a counter narrower than 32 bits ends the loop, unless the counter and the step are both bit strings. The counter keeps the wrapped value.
FUNCTION main : DINT
    VAR
        us : USINT;
        ui : UINT;
        s : SINT;
        bt : BYTE;
        step : SINT;
        n : DINT;
        after : DINT;
    END_VAR

    // Negative step below 0
    n := 0;
    FOR us := 10 TO 1 BY -3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := us;
    printf('USINT 10 TO 1 BY -3 = %d, after = %d$N', n, after);         // CHECK: USINT 10 TO 1 BY -3 = 4, after = 254

    n := 0;
    FOR ui := 10 TO 1 BY -3 DO
        n := n + 1;
        IF n > 100000 THEN EXIT; END_IF
    END_FOR
    after := ui;
    printf('UINT 10 TO 1 BY -3 = %d, after = %d$N', n, after);          // CHECK-NEXT: UINT 10 TO 1 BY -3 = 4, after = 65534

    n := 0;
    FOR us := 10 TO 0 BY -1 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := us;
    printf('USINT 10 TO 0 BY -1 = %d, after = %d$N', n, after);         // CHECK-NEXT: USINT 10 TO 0 BY -1 = 11, after = 255

    n := 0;
    FOR s := -120 TO -128 BY -5 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := s;
    printf('SINT -120 TO -128 BY -5 = %d, after = %d$N', n, after);     // CHECK-NEXT: SINT -120 TO -128 BY -5 = 2, after = 126

    // Positive step past the maximum
    n := 0;
    FOR us := 250 TO 255 BY 3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := us;
    printf('USINT 250 TO 255 BY 3 = %d, after = %d$N', n, after);       // CHECK-NEXT: USINT 250 TO 255 BY 3 = 2, after = 0

    n := 0;
    FOR us := 200 TO 250 BY 25 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := us;
    printf('USINT 200 TO 250 BY 25 = %d, after = %d$N', n, after);      // CHECK-NEXT: USINT 200 TO 250 BY 25 = 3, after = 19

    n := 0;
    FOR ui := 65530 TO 65535 BY 4 DO
        n := n + 1;
        IF n > 100000 THEN EXIT; END_IF
    END_FOR
    after := ui;
    printf('UINT 65530 TO 65535 BY 4 = %d, after = %d$N', n, after);    // CHECK-NEXT: UINT 65530 TO 65535 BY 4 = 2, after = 2

    n := 0;
    FOR s := 120 TO 127 BY 5 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := s;
    printf('SINT 120 TO 127 BY 5 = %d, after = %d$N', n, after);        // CHECK-NEXT: SINT 120 TO 127 BY 5 = 2, after = -126

    n := 0;
    FOR bt := 254 TO 255 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := bt;
    printf('BYTE 254 TO 255 = %d, after = %d$N', n, after);             // CHECK-NEXT: BYTE 254 TO 255 = 2, after = 0

    // CONTINUE does not skip the end test
    n := 0;
    FOR us := 250 TO 255 BY 3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
        CONTINUE;
    END_FOR
    after := us;
    printf('USINT 250 TO 255 BY 3 with CONTINUE = %d, after = %d$N', n, after);  // CHECK-NEXT: USINT 250 TO 255 BY 3 with CONTINUE = 2, after = 0

    // A step changed in the body moves the counter past the limit
    n := 0;
    step := 1;
    FOR us := 250 TO 255 BY step DO
        n := n + 1;
        step := 4;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := us;
    printf('USINT 250 TO 255 BY step = %d, after = %d$N', n, after);    // CHECK-NEXT: USINT 250 TO 255 BY step = 2, after = 2

    // A step larger than the counter's range
    n := 0;
    FOR us := 10 TO 200 BY 300 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := us;
    printf('USINT 10 TO 200 BY 300 = %d, after = %d$N', n, after);      // CHECK-NEXT: USINT 10 TO 200 BY 300 = 1, after = 54
END_FUNCTION
