PROGRAM mainProg
    VAR
        counter : DINT;

        FB_MA_dint : TEM_MovingAverage_UDINT;
        bytes : ARRAY[0..20] OF BYTE;
        
        AvgCount : UDINT;
        start : BOOL;
        clear : BOOL;
    END_VAR

    // CHECK: 0
    PRINTF('Counter: %d', bytes[0]);

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

FUNCTION main
    mainProg();
END_FUNCTION