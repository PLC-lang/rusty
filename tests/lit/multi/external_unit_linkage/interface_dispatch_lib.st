INTERFACE IShape
    METHOD area : DINT END_METHOD
    PROPERTY_GET name : DINT END_PROPERTY
END_INTERFACE

INTERFACE ISolid EXTENDS IShape
    METHOD volume : DINT END_METHOD
END_INTERFACE

INTERFACE ILogger
    METHOD log : DINT
    VAR_INPUT value : DINT; END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK LibSquare IMPLEMENTS IShape
VAR
    side : DINT := 3;
END_VAR
    METHOD area : DINT area := side * side; END_METHOD
    PROPERTY_GET name : DINT name := 100; END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK LibCube EXTENDS LibSquare IMPLEMENTS ISolid
    METHOD volume : DINT volume := area() * side; END_METHOD
    PROPERTY_GET name : DINT name := 200; END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION lib_area_of : DINT
VAR_INPUT shape : IShape; END_VAR
    lib_area_of := shape.area() + shape.name;
END_FUNCTION

FUNCTION_BLOCK LibService
VAR
    logger : ILogger;
    shapes : ARRAY[0..1] OF IShape;
END_VAR
    METHOD inject
    VAR_INPUT l : ILogger; END_VAR
        logger := l;
    END_METHOD

    METHOD add_shape
    VAR_INPUT i : DINT; s : IShape; END_VAR
        shapes[i] := s;
    END_METHOD

    METHOD run : DINT
        run := logger.log(shapes[0].area() + shapes[1].area());
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION lib_volume_as_shape : DINT
VAR_INPUT solid : ISolid; END_VAR
VAR shape : IShape; END_VAR
    shape := solid;
    lib_volume_as_shape := solid.volume() + shape.area();
END_FUNCTION
