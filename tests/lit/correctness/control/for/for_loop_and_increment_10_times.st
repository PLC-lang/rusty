// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR

    main := 100;
    FOR i := 1 TO 10 DO
        main := main + 1;
    END_FOR

    printf('main=%d$N', main); // CHECK: main=110
END_FUNCTION
