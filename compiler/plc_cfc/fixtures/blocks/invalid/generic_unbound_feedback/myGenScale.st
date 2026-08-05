FUNCTION myGenScale<T: ANY_NUM> : T
VAR_INPUT
    a : DINT;
    b : T;
END_VAR
END_FUNCTION

FUNCTION myGenScale__DINT : DINT
VAR_INPUT
    a : DINT;
    b : DINT;
END_VAR
    myGenScale__DINT := b + a;
END_FUNCTION
