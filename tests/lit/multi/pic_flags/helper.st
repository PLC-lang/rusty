FUNCTION helper : DINT
helper := 42;
END_FUNCTION

FUNCTION doubler : DINT
VAR_INPUT
    x : DINT;
END_VAR
doubler := x * 2;
END_FUNCTION
