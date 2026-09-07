FUNCTION myGenAdd<T: ANY_NUM> : T
VAR_INPUT
    a : T;
    b : T;
END_VAR
END_FUNCTION

FUNCTION myGenAdd__DINT : DINT
VAR_INPUT
    a : DINT;
    b : DINT;
END_VAR
    myGenAdd__DINT := a + b;
END_FUNCTION
