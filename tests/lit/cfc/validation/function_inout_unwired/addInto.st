FUNCTION addInto : DINT
    VAR_INPUT
        delta : DINT;
    END_VAR
    VAR_IN_OUT
        acc : DINT;
    END_VAR
    acc := acc + delta;
    addInto := acc;
END_FUNCTION
