FUNCTION_BLOCK counter
VAR_INPUT
    step : DINT;
END_VAR
VAR_OUTPUT
    count : DINT;
END_VAR
END_FUNCTION_BLOCK

ACTIONS
    ACTION bump
        count := count + step;
    END_ACTION
END_ACTIONS
