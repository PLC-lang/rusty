FUNCTION_BLOCK Named
VAR
    text : STRING[20] := 'init';
END_VAR
    PROPERTY_GET name : STRING[20]
        name := text;
    END_PROPERTY

    PROPERTY_SET name : STRING[20]
        text := name;
    END_PROPERTY

    PROPERTY_GET pair : ARRAY[0..1] OF DINT
        pair[0] := 5;
        pair[1] := 6;
    END_PROPERTY
END_FUNCTION_BLOCK
