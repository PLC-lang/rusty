FUNCTION_BLOCK Impl IMPLEMENTS IShape, IChild
VAR
    unit_seen : STRING[10];
END_VAR
    METHOD area : DINT
    VAR_INPUT
        unit : STRING[10];
    END_VAR
        unit_seen := unit;
        area := 20;
    END_METHOD

    METHOD base_id : DINT
        base_id := 2;
    END_METHOD

    METHOD child_id : DINT
        child_id := 3;
    END_METHOD
END_FUNCTION_BLOCK
