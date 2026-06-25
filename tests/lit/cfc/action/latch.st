// A function block with an action that mutates its internal state. The action takes no
// inputs and returns nothing — it only acts on `state` (this is what actions are for).
FUNCTION_BLOCK Latch
VAR
    state : DINT;
END_VAR
END_FUNCTION_BLOCK

ACTIONS
    ACTION fire
        state := 5;
    END_ACTION
END_ACTIONS
