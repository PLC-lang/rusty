// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR

    main := 1000;
    FOR i := 1 TO 10 BY 2 DO
        main := main + 1;
    END_FOR

    printf('main=%d$N', main); // CHECK: main=1005
END_FUNCTION
