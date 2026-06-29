PROGRAM mainProg
    VAR
        counter : DINT;

        FB_MA_dint : TEM_MovingAverage_DINT;
        bytes : ARRAY[0..20] OF BYTE;

        AvgCount : UDINT;
        start : BOOL;
        clear : BOOL;
    END_VAR

    // CHECK: Counter: 0
    printf('Counter: %d', bytes[0]);

    IF start THEN
        counter := counter +1;
    END_IF
    IF clear THEN
        clear := FALSE;
        counter := 0;
    END_IF

    FB_MA_dint(
        diInputValue := counter,
        udiCycles := 1000,
        diAverageValue => AvgCount
    );
END_PROGRAM

FUNCTION main : DINT
    mainProg();
END_FUNCTION
