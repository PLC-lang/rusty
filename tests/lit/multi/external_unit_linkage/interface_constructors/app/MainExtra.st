FUNCTION_BLOCK MainExtra IMPLEMENTS IExtra
VAR
    text : STRING[20] := 'start';
    base : DINT := 1000;
END_VAR
    METHOD pick : DINT
    VAR_INPUT
        c : Mode;
        r : INT(0..10);
    END_VAR
        IF c = fast THEN
            pick := base + 100 + r;
        ELSE
            pick := base + r;
        END_IF
    END_METHOD

    METHOD grid : DINT
    VAR_INPUT
        others : ARRAY[0..1] OF IOther;
        p : POINTER TO STRING[10];
    END_VAR
        text := 'mgrid';
        p^ := 'mpw';
        grid := base + others[0].id() + others[1].id();
    END_METHOD

    METHOD defaults : DINT
    VAR_INPUT
        d : STRING[10] := 'dflt';
        m : Mode := fast;
    END_VAR
        text := d;
        defaults := base + m;
    END_METHOD

    METHOD triple : ARRAY[0..2] OF DINT
        triple[0] := base + 1;
        triple[1] := base + 2;
        triple[2] := base + 3;
    END_METHOD

    PROPERTY_SET name : STRING[20]
        text := name;
    END_PROPERTY

    PROPERTY_GET name : STRING[20]
        name := text;
    END_PROPERTY

    PROPERTY_GET pair : ARRAY[0..1] OF DINT
        pair[0] := base + 7;
        pair[1] := base + 8;
    END_PROPERTY
END_FUNCTION_BLOCK
