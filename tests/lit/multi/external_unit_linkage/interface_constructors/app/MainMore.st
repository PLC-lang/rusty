FUNCTION_BLOCK MainMore IMPLEMENTS IMore
    METHOD id : DINT
        id := 20;
    END_METHOD

    METHOD wide : DINT
    VAR_INPUT
        t : WSTRING[9] := "wide";
    END_VAR
        wide := 200 + LEN(t);
    END_METHOD

    METHOD take_ref : DINT
    VAR_INPUT
        x : REFERENCE TO IOther;
    END_VAR
        take_ref := 2000 + x.id();
    END_METHOD
END_FUNCTION_BLOCK
