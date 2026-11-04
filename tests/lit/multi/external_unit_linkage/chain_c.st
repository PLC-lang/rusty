FUNCTION_BLOCK GrandChild EXTENDS Concrete
    METHOD OVERRIDE area : DINT
    VAR_INPUT
        unit : STRING[10];
    END_VAR
        unit_seen := unit;
        area := 40;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION shape_area : DINT
VAR_INPUT
    shape : IShape;
END_VAR
    shape_area := shape.area('km');
END_FUNCTION

FUNCTION base_of : DINT
VAR_INPUT
    child : IChild;
END_VAR
VAR
    base : IBase;
END_VAR
    base := child;
    base_of := base.base_id() * 10 + child.child_id();
END_FUNCTION

FUNCTION extra_of : DINT
VAR_INPUT
    extra : IExtra;
END_VAR
    extra_of := extra.extra();
END_FUNCTION
