FUNCTION_BLOCK counter
VAR
    count : DINT;
END_VAR
METHOD bump : DINT
VAR_INPUT
    step : DINT;
END_VAR
VAR_OUTPUT
    doubled : DINT;
END_VAR
    count := count + step;
    doubled := count * 2;
    bump := count;
END_METHOD
END_FUNCTION_BLOCK
