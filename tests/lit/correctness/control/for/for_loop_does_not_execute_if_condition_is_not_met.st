// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i, end, step, res : DINT;
    END_VAR

    end := -1;
    step := -1;
    res := 100;

    FOR i := end TO 0 BY step DO
        res := i;
    END_FOR

    main := res;
    printf('main=%d$N', main); // CHECK: main=100
END_FUNCTION
