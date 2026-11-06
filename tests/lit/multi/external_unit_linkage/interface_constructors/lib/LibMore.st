FUNCTION_BLOCK LibMore IMPLEMENTS IMore
    METHOD id : DINT
        id := 10;
    END_METHOD

    METHOD wide : DINT
    VAR_INPUT
        t : WSTRING[9] := "wide";
    END_VAR
        wide := 100;
    END_METHOD

    METHOD take_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IOther;
    END_VAR
        take_ref := 1000 + x.id();
    END_METHOD
END_FUNCTION_BLOCK
