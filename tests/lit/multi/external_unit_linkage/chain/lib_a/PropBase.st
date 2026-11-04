FUNCTION_BLOCK PropBase
VAR
    text : STRING[20] := 'base';
END_VAR
    PROPERTY_GET name : STRING[20]
        name := text;
    END_PROPERTY
END_FUNCTION_BLOCK
