CLASS ByTwo EXTENDS Counter IMPLEMENTS IArea
    METHOD OVERRIDE step : DINT
        step := 2;
    END_METHOD

    METHOD area : DINT
        area := 4;
    END_METHOD
END_CLASS
