FUNCTION_BLOCK LibText IMPLEMENTS INamed
VAR
    text : STRING[20] := 'lib';
END_VAR
    METHOD put : DINT
    VAR_INPUT
        s : STRING[20];
        a : ARRAY[0..3] OF DINT;
        w : WSTRING[20];
    END_VAR
        text := s;
        put := a[0] + a[3];
    END_METHOD

    METHOD get : STRING[20]
        get := text;
    END_METHOD

    METHOD split : DINT
    VAR_OUTPUT
        head : STRING[20];
    END_VAR
    VAR_IN_OUT
        tail : STRING[20];
    END_VAR
        head := text;
        tail := 'cut';
        split := 3;
    END_METHOD

    METHOD by_ref : DINT
    VAR_INPUT
        r : REFERENCE TO STRING[20];
        p : REF_TO DINT;
    END_VAR
        r := text;
        by_ref := p^ + 1;
    END_METHOD

    PROPERTY_GET label : STRING[20]
        label := text;
    END_PROPERTY

    METHOD rename : DINT
    VAR_INPUT
        n : STRING[20];
    END_VAR
        text := n;
        rename := 7;
    END_METHOD
END_FUNCTION_BLOCK
