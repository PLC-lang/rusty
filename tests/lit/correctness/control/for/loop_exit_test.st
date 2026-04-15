// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    FOR main := 100 TO 1000 BY 7 DO
        EXIT;
        main := 200;
    END_FOR
    WHILE main > 50 DO
        EXIT;
        main := 200;
    END_WHILE

    printf('main=%d$N', main); // CHECK: main=100
END_FUNCTION
