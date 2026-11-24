// RUN: %COMPILE %s && %RUN | %CHECK %s
// A 64-bit or floating-point counter computes its next value in its own type, as an assignment to the counter does.
FUNCTION main : DINT
    VAR
        r : REAL;
        lr : LREAL;
        l : LINT;
        ul : ULINT;
        n : DINT;
    END_VAR

    n := 0;
    FOR r := 0.0 TO 1.0 BY 0.25 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('REAL 0.0 TO 1.0 BY 0.25 = %d, after = %.2f$N', n, r);
    // CHECK: REAL 0.0 TO 1.0 BY 0.25 = 5, after = 1.25

    n := 0;
    FOR lr := 1.0 TO 0.0 BY -0.5 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('LREAL 1.0 TO 0.0 BY -0.5 = %d, after = %.2f$N', n, lr);
    // CHECK-NEXT: LREAL 1.0 TO 0.0 BY -0.5 = 3, after = -0.50

    n := 0;
    FOR ul := 9223372036854775806 TO 9223372036854775810 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('ULINT across 2^63 = %d, after = %llu$N', n, ul);
    // CHECK-NEXT: ULINT across 2^63 = 5, after = 9223372036854775811

    n := 0;
    FOR l := 9223372036854775800 TO 9223372036854775805 BY 2 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('LINT near the maximum = %d, after = %lld$N', n, l);
    // CHECK-NEXT: LINT near the maximum = 3, after = 9223372036854775806
END_FUNCTION
