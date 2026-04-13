// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    FOR main := 1 TO 10 BY 1 DO
        main := 10;
        CONTINUE;
        main := 200;
    END_FOR

    printf('main=%d$N', main); // CHECK: main=11
END_FUNCTION
