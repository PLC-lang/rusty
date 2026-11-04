FUNCTION_BLOCK Concrete EXTENDS AbstractShape
    METHOD OVERRIDE area : DINT
    VAR_INPUT
        unit : STRING[10];
    END_VAR
        unit_seen := unit;
        area := 30;
    END_METHOD
END_FUNCTION_BLOCK
