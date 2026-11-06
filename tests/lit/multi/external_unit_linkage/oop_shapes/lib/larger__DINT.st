FUNCTION larger__DINT : DINT
VAR_INPUT
    a : DINT;
    b : DINT;
END_VAR
    IF a > b THEN
        larger__DINT := a;
    ELSE
        larger__DINT := b;
    END_IF
END_FUNCTION
