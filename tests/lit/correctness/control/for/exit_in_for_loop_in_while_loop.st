// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i : INT;
    END_VAR

    main := 0;
    WHILE main < 20 DO
        main := main + 1;
        FOR i := 0 TO 10 BY 1 DO
            EXIT;
        END_FOR
    END_WHILE
    main := i + main;

    printf('main=%d$N', main); // CHECK: main=20
END_FUNCTION
