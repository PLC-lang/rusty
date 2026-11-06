INTERFACE IText
    METHOD put : DINT
    VAR_INPUT
        s : STRING[20];
        a : ARRAY[0..3] OF DINT;
        w : WSTRING[20];
    END_VAR
    END_METHOD

    METHOD get : STRING[20] END_METHOD

    METHOD split : DINT
    VAR_OUTPUT
        head : STRING[20];
    END_VAR
    VAR_IN_OUT
        tail : STRING[20];
    END_VAR
    END_METHOD

    METHOD by_ref : DINT
    VAR_INPUT
        r : REFERENCE TO STRING[20];
        p : REF_TO DINT;
    END_VAR
    END_METHOD

    PROPERTY_GET label : STRING[20] END_PROPERTY
END_INTERFACE
