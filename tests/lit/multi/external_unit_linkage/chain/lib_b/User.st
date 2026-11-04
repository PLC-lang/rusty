FUNCTION_BLOCK User
VAR_INPUT
    plain : Plain;
    plains : ARRAY[0..1] OF Plain;
    shape : IShape;
    shapes : ARRAY[0..1] OF IShape;
END_VAR
    METHOD total : DINT
        total := plain.x + shape.area('m') + shapes[0].area('cm') + shapes[1].area('mm');
    END_METHOD
END_FUNCTION_BLOCK
