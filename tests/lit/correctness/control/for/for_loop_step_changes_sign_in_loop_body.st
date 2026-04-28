// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        i, step, temp, iteration : DINT;
    END_VAR

    step := 1;
    iteration := 0;

    FOR i := 5 TO 10 BY step DO
        temp := (step + 1) * -1;
        IF i + temp > 0 THEN
            step := temp;
        ELSE
            step := step + 3;
        END_IF;

        // i:     5, 3,  4, 2,  3, 1, 2,  6, 1, 5, 12
        // step: -2, 1, -2, 1, -2, 1, 4, -5, 4, 7
        iteration := iteration + 1;
    END_FOR

    main := iteration;
    printf('main=%d$N', main); // CHECK: main=10
END_FUNCTION
