// RUN: %COMPILE %s && %RUN | %CHECK %s
// A BOOL counter computes its next value as a LINT, like the other counters of up to 32 bits, so the loop ends after FALSE and TRUE.
FUNCTION main : DINT
    VAR
        b : BOOL;
        end_b : BOOL := TRUE;
        n : DINT;
    END_VAR

    n := 0;
    FOR b := FALSE TO TRUE DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('BOOL FALSE TO TRUE = %d$N', n);   // CHECK: BOOL FALSE TO TRUE = 2

    n := 0;
    FOR b := FALSE TO end_b DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('BOOL FALSE TO end_b = %d$N', n);  // CHECK-NEXT: BOOL FALSE TO end_b = 2
END_FUNCTION
