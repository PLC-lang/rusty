// RUN: %COMPILE %s && %RUN | %CHECK %s
// FOR loops whose counter stays inside the range of its type.
FUNCTION main : DINT
    VAR
        us : USINT;
        ud : UDINT;
        s : SINT;
        n : DINT;
    END_VAR

    n := 0;
    FOR us := 1 TO 10 BY 3 DO
        n := n + 1;
    END_FOR
    printf('USINT 1 TO 10 BY 3 = %d$N', n);                 // CHECK: USINT 1 TO 10 BY 3 = 4

    n := 0;
    FOR us := 10 TO 1 BY -1 DO
        n := n + 1;
    END_FOR
    printf('USINT 10 TO 1 BY -1 = %d$N', n);                // CHECK-NEXT: USINT 10 TO 1 BY -1 = 10

    n := 0;
    FOR us := 200 TO 220 BY 10 DO
        n := n + 1;
    END_FOR
    printf('USINT 200 TO 220 BY 10 = %d$N', n);             // CHECK-NEXT: USINT 200 TO 220 BY 10 = 3

    n := 0;
    FOR ud := 10 TO 1 BY -3 DO
        n := n + 1;
    END_FOR
    printf('UDINT 10 TO 1 BY -3 = %d$N', n);                // CHECK-NEXT: UDINT 10 TO 1 BY -3 = 4

    n := 0;
    FOR s := 10 TO -10 BY -5 DO
        n := n + 1;
    END_FOR
    printf('SINT 10 TO -10 BY -5 = %d$N', n);               // CHECK-NEXT: SINT 10 TO -10 BY -5 = 5
END_FUNCTION
