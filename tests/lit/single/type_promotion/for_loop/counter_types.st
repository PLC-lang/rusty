// RUN: %COMPILE %s && %RUN | %CHECK %s
// The next counter value is a LINT for an integer, bit-string or BOOL counter of up to 32 bits, and has the counter's
// own type otherwise, so every counter type ends at its end value.
FUNCTION other : DINT
    VAR
        __next_0 : LREAL;
        r : REAL := 1.5;
    END_VAR
    // A user variable with the name of a loop temporary does not change the loop
    __next_0 := r;
    other := 0;
END_FUNCTION

FUNCTION main : DINT
    VAR
        d : DINT;
        ud : UDINT;
        b : BOOL;
        t : TIME;
        r : REAL;
        ul : ULINT;
        n : DINT;
        after : LINT;
    END_VAR

    n := 0;
    FOR d := 16777216 TO 16777220 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('DINT 16777216 TO 16777220 = %d$N', n);                                   // CHECK: DINT 16777216 TO 16777220 = 5

    n := 0;
    FOR ud := 2400000000 TO 2000000000 BY -100000000 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    after := ud;
    printf('UDINT 2400000000 TO 2000000000 BY -100000000 = %d, after = %lld$N', n, after);
    // CHECK-NEXT: UDINT 2400000000 TO 2000000000 BY -100000000 = 5, after = 1900000000

    n := 0;
    FOR b := FALSE TO TRUE DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('BOOL FALSE TO TRUE = %d$N', n);                                          // CHECK-NEXT: BOOL FALSE TO TRUE = 2

    n := 0;
    FOR t := T#1s TO T#3s BY T#1s DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('TIME T#1s TO T#3s BY T#1s = %d$N', n);                                   // CHECK-NEXT: TIME T#1s TO T#3s BY T#1s = 3

    n := 0;
    FOR r := 0.0 TO 1.0 BY 0.25 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('REAL 0.0 TO 1.0 BY 0.25 = %d, after = %.2f$N', n, r);                    // CHECK-NEXT: REAL 0.0 TO 1.0 BY 0.25 = 5, after = 1.25

    n := 0;
    FOR ul := 9223372036854775806 TO 9223372036854775810 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('ULINT across 2^63 = %d$N', n);                                           // CHECK-NEXT: ULINT across 2^63 = 5
END_FUNCTION
