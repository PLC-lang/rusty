// RUN: %COMPILE %s && %RUN | %CHECK %s
// A step or end value in parentheses gives the same number of passes as without parentheses.
VAR_GLOBAL
    end_value : LINT := 4;
END_VAR

FUNCTION end_ref : REFERENCE TO LINT
    end_ref REF= end_value;
END_FUNCTION

FUNCTION main : DINT
    VAR
        ul : ULINT;
        lw : LWORD;
        ud : UDINT;
        step : SINT := 2;
        n : DINT;
    END_VAR

    n := 0;
    FOR ul := 1 TO 7 BY (step) DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('ULINT 1 TO 7 BY (step) = %d$N', n);         // CHECK: ULINT 1 TO 7 BY (step) = 4

    n := 0;
    FOR lw := 1 TO 7 BY ((step)) DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('LWORD 1 TO 7 BY ((step)) = %d$N', n);       // CHECK-NEXT: LWORD 1 TO 7 BY ((step)) = 4

    n := 0;
    FOR ud := 1 TO (end_ref()) DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('UDINT 1 TO (end_ref()) = %d$N', n);         // CHECK-NEXT: UDINT 1 TO (end_ref()) = 4
END_FUNCTION
