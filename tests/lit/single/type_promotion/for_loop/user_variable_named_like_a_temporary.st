// RUN: %COMPILE %s && %RUN | %CHECK %s
// A user variable with the name of a loop temporary does not change the loop.
FUNCTION other : DINT
    VAR
        __next_0 : LREAL;
        r : REAL := 1.5;
    END_VAR
    __next_0 := r;
    other := 0;
END_FUNCTION

FUNCTION main : DINT
    VAR
        i : DINT;
        n : DINT;
    END_VAR

    n := 0;
    FOR i := 16777216 TO 16777220 DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('DINT 16777216 TO 16777220 = %d, after = %d$N', n, i);  // CHECK: DINT 16777216 TO 16777220 = 5, after = 16777221
END_FUNCTION
