INTERFACE IGreeter
    METHOD greet
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IGreeter
VAR
    id : DINT;
END_VAR
    METHOD greet
        printf('FbA::greet (id=%d)$N', THIS^.id);
    END_METHOD
END_FUNCTION_BLOCK
