INTERFACE IChild EXTENDS IBase
    METHOD child_id : DINT END_METHOD
END_INTERFACE

INTERFACE IExtra
    METHOD extra : DINT END_METHOD
END_INTERFACE

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

FUNCTION_BLOCK Concrete EXTENDS AbstractShape
    METHOD OVERRIDE area : DINT
    VAR_INPUT
        unit : STRING[10];
    END_VAR
        unit_seen := unit;
        area := 30;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Child EXTENDS Base IMPLEMENTS IExtra
    METHOD OVERRIDE kind : DINT
        kind := 11;
    END_METHOD

    METHOD extra : DINT
        extra := 12;
    END_METHOD
END_FUNCTION_BLOCK

CLASS CChild EXTENDS CBase
    METHOD OVERRIDE value : DINT
        value := 200;
    END_METHOD
END_CLASS

FUNCTION_BLOCK PropChild EXTENDS PropBase
    PROPERTY_GET name : STRING[20]
        name := 'child';
    END_PROPERTY
END_FUNCTION_BLOCK

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
