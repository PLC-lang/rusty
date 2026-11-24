// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// For a 64-bit counter, the next value has the counter's own type, so a next value past the type's limit wraps before the end test and the loop goes on.
FUNCTION main : DINT
    VAR
        ul : ULINT;
        l : LINT;
        n : DINT;
    END_VAR

    n := 0;
    FOR ul := 10 TO 0 BY -1 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('ULINT 10 TO 0 BY -1 = %d$N', n);                                    // CHECK: ULINT 10 TO 0 BY -1 = 11

    n := 0;
    FOR l := 9223372036854775800 TO 9223372036854775807 BY 5 DO
        n := n + 1;
        IF n > 1000 THEN EXIT; END_IF
    END_FOR
    printf('LINT 9223372036854775800 TO 9223372036854775807 BY 5 = %d$N', n);  // CHECK-NEXT: LINT 9223372036854775800 TO 9223372036854775807 BY 5 = 2
END_FUNCTION
