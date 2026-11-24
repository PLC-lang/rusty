// RUN: %COMPILE %s && %RUN | %CHECK %s
// A counter narrower than 64 bits with a LINT or ULINT end value runs the expected number of passes.
FUNCTION main : DINT
    VAR
        us : USINT;
        ud : UDINT;
        d : DINT;
        end_l : LINT := 5;
        end_ul : ULINT := 5;
        step : SINT := 2;
        n : DINT;
    END_VAR

    n := 0;
    FOR us := 1 TO end_l DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('USINT 1 TO LINT 5 = %d$N', n);                      // CHECK: USINT 1 TO LINT 5 = 5

    n := 0;
    FOR ud := 1 TO end_ul DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('UDINT 1 TO ULINT 5 = %d$N', n);                     // CHECK-NEXT: UDINT 1 TO ULINT 5 = 5

    n := 0;
    FOR d := 1 TO end_l BY step DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('DINT 1 TO LINT 5 BY SINT 2 = %d$N', n);             // CHECK-NEXT: DINT 1 TO LINT 5 BY SINT 2 = 3
END_FUNCTION
