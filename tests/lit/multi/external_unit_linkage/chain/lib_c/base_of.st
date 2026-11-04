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
