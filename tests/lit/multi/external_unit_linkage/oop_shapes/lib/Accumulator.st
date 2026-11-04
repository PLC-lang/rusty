FUNCTION_BLOCK Accumulator
VAR_OUTPUT
    total : DINT;
    items : ARRAY[0..2] OF STRING[5];
    mode : (adding, idle) := adding;
    inner : STRUCT
        hits : DINT := 1;
    END_STRUCT;
END_VAR
    METHOD push : DINT
    VAR_INPUT
        v : DINT;
        tag : STRING[5];
    END_VAR
    VAR
        local_tag : STRING[5];
    END_VAR
    VAR_TEMP
        tmp : ARRAY[0..1] OF DINT;
    END_VAR
        tmp[0] := v;
        local_tag := tag;
        items[0] := local_tag;
        IF mode = adding THEN
            total := total + tmp[0];
        END_IF
        inner.hits := inner.hits + 1;
        push := total;
    END_METHOD
END_FUNCTION_BLOCK

ACTIONS Accumulator
    ACTION clear
        total := 0;
    END_ACTION
END_ACTIONS
