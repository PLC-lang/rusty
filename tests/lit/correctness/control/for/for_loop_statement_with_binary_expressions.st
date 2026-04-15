// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION main : DINT
    VAR
        step : DINT := 1;
        x, y, iteration : DINT;
        z : DINT := 25;
    END_VAR

    // 1 TO 23 BY 3
    FOR x := y + 1 TO z - 2 BY step * 3 DO
        // x: 1, 4, 7, 10, 13, 16, 19, 22
        iteration := iteration + 1;
    END_FOR

    main := iteration;
    printf('main=%d$N', main); // CHECK: main=8
END_FUNCTION
