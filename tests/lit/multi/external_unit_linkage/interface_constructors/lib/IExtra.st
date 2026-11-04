INTERFACE IExtra
    METHOD pick : DINT
    VAR_INPUT
        c : Mode;
        r : INT(0..10);
    END_VAR
    END_METHOD

    METHOD grid : DINT
    VAR_INPUT
        others : ARRAY[0..1] OF IOther;
        p : POINTER TO STRING[10];
    END_VAR
    END_METHOD

    METHOD defaults : DINT
    VAR_INPUT
        d : STRING[10] := 'dflt';
        m : Mode := fast;
    END_VAR
    END_METHOD

    METHOD triple : ARRAY[0..2] OF DINT END_METHOD

    PROPERTY_SET name : STRING[20] END_PROPERTY
    PROPERTY_GET name : STRING[20] END_PROPERTY
    PROPERTY_GET pair : ARRAY[0..1] OF DINT END_PROPERTY
END_INTERFACE
