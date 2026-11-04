FUNCTION_BLOCK ABSTRACT AbstractShape IMPLEMENTS IArea
VAR_OUTPUT
    label : STRING[10] := 'shape';
    scale : DINT := 2;
END_VAR
    METHOD ABSTRACT area : DINT END_METHOD

    METHOD ABSTRACT describe : DINT
    VAR_INPUT
        prefix : STRING[10];
    END_VAR
    END_METHOD

    METHOD scaled : DINT
        scaled := THIS^.area() * scale;
    END_METHOD
END_FUNCTION_BLOCK
