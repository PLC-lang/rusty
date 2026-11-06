FUNCTION_BLOCK Triangle EXTENDS AbstractShape
    METHOD OVERRIDE area : DINT
        area := 6;
    END_METHOD

    METHOD OVERRIDE describe : DINT
    VAR_INPUT
        prefix : STRING[10];
    END_VAR
        label := prefix;
        describe := 3;
    END_METHOD
END_FUNCTION_BLOCK
