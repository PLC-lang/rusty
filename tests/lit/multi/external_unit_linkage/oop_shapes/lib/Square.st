FUNCTION_BLOCK Square EXTENDS AbstractShape
VAR
    side : DINT := 3;
END_VAR
    METHOD OVERRIDE area : DINT
        area := side * side;
    END_METHOD

    METHOD OVERRIDE describe : DINT
    VAR_INPUT
        prefix : STRING[10];
    END_VAR
        label := prefix;
        describe := side;
    END_METHOD

    METHOD FINAL sides : DINT
        sides := 4;
    END_METHOD
END_FUNCTION_BLOCK
