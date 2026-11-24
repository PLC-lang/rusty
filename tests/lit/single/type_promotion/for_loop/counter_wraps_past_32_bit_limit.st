// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// For a 32-bit or 64-bit counter, and for a bit-string counter with a bit-string step, the sum of counter and step is usually only as wide as the counter, so a step past the type's limit wraps before the end test and the loop goes on.
FUNCTION main : DINT
    VAR
        ud : UDINT;
        d : DINT;
        ul : ULINT;
        bt : BYTE;
        n : DINT;
    END_VAR

    n := 0;
    FOR ud := 4294967290 TO 4294967295 BY 3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('UDINT 4294967290 TO 4294967295 BY 3 = %d$N', n);    // CHECK: UDINT 4294967290 TO 4294967295 BY 3 = 2

    n := 0;
    FOR d := 2147483640 TO 2147483647 BY 5 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('DINT 2147483640 TO 2147483647 BY 5 = %d$N', n);     // CHECK-NEXT: DINT 2147483640 TO 2147483647 BY 5 = 2

    n := 0;
    FOR ul := 10 TO 0 BY -1 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('ULINT 10 TO 0 BY -1 = %d$N', n);                    // CHECK-NEXT: ULINT 10 TO 0 BY -1 = 11

    n := 0;
    FOR bt := 250 TO 255 BY BYTE#3 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('BYTE 250 TO 255 BY BYTE#3 = %d$N', n);              // CHECK-NEXT: BYTE 250 TO 255 BY BYTE#3 = 2
END_FUNCTION
