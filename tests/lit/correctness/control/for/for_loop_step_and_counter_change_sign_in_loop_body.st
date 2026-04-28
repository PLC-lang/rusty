// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i, step, temp, iteration : DINT;
    END_VAR

    step := 1;
    iteration := 0;

    FOR i := 3 TO 10 BY step DO
        step := (step + 1) * -2;

        // i:     3, -1,   5, -9, 17
        // step: -4,  5, -14, 26
        iteration := iteration + 1;
    END_FOR

    main := iteration;
    printf('main=%d$N', main); // CHECK: main=4
END_FUNCTION
