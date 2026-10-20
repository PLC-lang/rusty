FUNCTION_BLOCK counter
VAR
    total : DINT;
END_VAR
METHOD add : DINT
VAR_INPUT
    in : DINT;
END_VAR
VAR_OUTPUT
    doubled : DINT;
END_VAR
    total := total + in;
    doubled := total * 2;
    add := total;
END_METHOD
END_FUNCTION_BLOCK
