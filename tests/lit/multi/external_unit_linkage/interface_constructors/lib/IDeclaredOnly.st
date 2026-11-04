INTERFACE IDeclaredOnly
    METHOD pick : DINT
    VAR_INPUT
        c : (cyan, magenta);
    END_VAR
    END_METHOD

    METHOD grid : DINT
    VAR_INPUT
        n : ARRAY[0..1] OF ARRAY[0..1] OF STRING[5];
        row : ARRAY[0..1] OF STRING[5];
    END_VAR
    END_METHOD

    METHOD slot : REFERENCE TO DINT END_METHOD

    METHOD texts : DINT
    VAR_IN_OUT
        arr : ARRAY[*] OF STRING[4];
    END_VAR
    END_METHOD

    METHOD nested_pointers : DINT
    VAR_INPUT
        q : POINTER TO ARRAY[0..1] OF STRING[5];
        r : REF_TO ARRAY[0..1] OF DINT;
    END_VAR
    END_METHOD
END_INTERFACE
