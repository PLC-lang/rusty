FUNCTION_BLOCK counter
VAR_INPUT
    step : DINT;
END_VAR
VAR_OUTPUT
    total : DINT;
END_VAR
    total := total + step;
END_FUNCTION_BLOCK
