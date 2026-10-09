FUNCTION_BLOCK AppLogger IMPLEMENTS ILogger
VAR
    last : DINT;
END_VAR
    METHOD log : DINT
    VAR_INPUT value : DINT; END_VAR
        last := value;
        log := value + 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK AppCircle IMPLEMENTS IShape
    METHOD area : DINT area := 7; END_METHOD
    PROPERTY_GET name : DINT name := 300; END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK AppBigCube EXTENDS LibCube
    METHOD area : DINT area := 50; END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
VAR
    square : LibSquare;
    cube : LibCube;
    bigCube : AppBigCube;
    circle : AppCircle;
    logger : AppLogger;
    service : LibService;
    shape : IShape;
    solid : ISolid;
END_VAR
    // Lib type, called through an interface in the app
    shape := square;
    printf('%d %d$N', shape.area(), shape.name);
    shape := cube;
    printf('%d %d$N', shape.area(), shape.name);
    solid := cube;
    printf('%d$N', solid.volume());

    // App type passed to a lib function that calls through the interface
    printf('%d$N', lib_area_of(circle));
    printf('%d$N', lib_area_of(square));

    // App type that extends a lib type, overriding a method
    shape := bigCube;
    printf('%d %d$N', shape.area(), shape.name);
    printf('%d$N', lib_volume_as_shape(bigCube));

    // Dependency inversion: the lib stores app objects and calls them later
    service.inject(logger);
    service.add_shape(0, circle);
    service.add_shape(1, bigCube);
    printf('%d %d$N', service.run(), logger.last);
END_FUNCTION
