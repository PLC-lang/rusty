// RUN: %COMPILE %s && %RUN | %CHECK %s
// An unsigned counter with a signed step compares its next value without a change of sign.
FUNCTION main : DINT
    VAR
        ud : UDINT;
        end_ud : UDINT := 2000000000;
        step_d : DINT := -100000000;
        n : DINT;
        after : LINT;
    END_VAR

    n := 0;
    FOR ud := 2400000000 TO 2000000000 BY -100000000 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := ud;
    printf('UDINT 2400000000 TO 2000000000 BY -100000000 = %d, after = %lld$N', n, after);
    // CHECK: UDINT 2400000000 TO 2000000000 BY -100000000 = 5, after = 1900000000

    n := 0;
    FOR ud := 2400000000 TO end_ud BY step_d DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := ud;
    printf('UDINT 2400000000 TO UDINT BY DINT = %d, after = %lld$N', n, after);
    // CHECK-NEXT: UDINT 2400000000 TO UDINT BY DINT = 5, after = 1900000000

    n := 0;
    FOR ud := 2000000000 TO 2400000000 BY 100000000 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    after := ud;
    printf('UDINT 2000000000 TO 2400000000 BY 100000000 = %d, after = %lld$N', n, after);
    // CHECK-NEXT: UDINT 2000000000 TO 2400000000 BY 100000000 = 5, after = 2500000000
END_FUNCTION
