FUNCTION_BLOCK GrandChild EXTENDS Concrete
    METHOD OVERRIDE area : DINT
    VAR_INPUT
        unit : STRING[10];
    END_VAR
        unit_seen := unit;
        area := 40;
    END_METHOD
END_FUNCTION_BLOCK
