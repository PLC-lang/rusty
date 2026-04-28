// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR

    main := 100;
    FOR i := 10 TO 1 BY -1 DO
        main := main + 1;
    END_FOR

    printf('main=%d$N', main); // CHECK: main=110
END_FUNCTION
