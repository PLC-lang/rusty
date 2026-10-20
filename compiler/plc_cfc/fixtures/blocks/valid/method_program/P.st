PROGRAM P
VAR
    total : DINT;
END_VAR
METHOD bump : DINT
VAR_INPUT
    step : DINT;
END_VAR
    total := total + step;
    bump := total;
END_METHOD
END_PROGRAM
