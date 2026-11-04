FUNCTION shape_area : DINT
VAR_INPUT
    shape : IShape;
END_VAR
    shape_area := shape.area('km');
END_FUNCTION
