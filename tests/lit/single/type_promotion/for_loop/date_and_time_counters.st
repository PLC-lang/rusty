// RUN: %COMPILE %s && %RUN | %CHECK %s
// A TIME counter computes its next value as a TIME, so the loop ends at the end value.
FUNCTION main : DINT
    VAR
        t : TIME;
        lt : LTIME;
        n : DINT;
    END_VAR

    n := 0;
    FOR t := T#1s TO T#3s BY T#1s DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('TIME T#1s TO T#3s BY T#1s = %d$N', n);    // CHECK: TIME T#1s TO T#3s BY T#1s = 3

    n := 0;
    FOR t := T#3s TO T#1s BY -T#1s DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('TIME T#3s TO T#1s BY -T#1s = %d$N', n);   // CHECK-NEXT: TIME T#3s TO T#1s BY -T#1s = 3

    n := 0;
    FOR t := T#1s TO T#1s500ms DO
        n := n + 1;
        IF n > 10000 THEN EXIT; END_IF
    END_FOR
    printf('TIME T#1s TO T#1s500ms = %d$N', n);       // CHECK-NEXT: TIME T#1s TO T#1s500ms = 501

    n := 0;
    FOR lt := LTIME#1s TO LTIME#3s BY LTIME#1s DO
        n := n + 1;
        IF n > 100 THEN EXIT; END_IF
    END_FOR
    printf('LTIME 1s TO 3s BY 1s = %d$N', n);         // CHECK-NEXT: LTIME 1s TO 3s BY 1s = 3
END_FUNCTION
