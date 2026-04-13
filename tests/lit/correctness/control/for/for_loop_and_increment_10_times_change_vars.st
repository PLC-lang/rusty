// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i : INT := 0;
    END_VAR
    VAR_TEMP
        start, end : INT;
    END_VAR

    main := 100;
    start := 1;
    end := 2;
    FOR i := start TO end BY 1 DO
        main := main + 1;
        end := 10;
    END_FOR

    printf('main=%d$N', main); // CHECK: main=110
END_FUNCTION
