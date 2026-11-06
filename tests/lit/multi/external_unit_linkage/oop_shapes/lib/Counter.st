CLASS ABSTRACT Counter
VAR
    count : DINT;
END_VAR
    METHOD ABSTRACT step : DINT END_METHOD

    METHOD tick : DINT
        count := count + 2;
        tick := count;
    END_METHOD

    METHOD current : DINT
        current := count;
    END_METHOD
END_CLASS
