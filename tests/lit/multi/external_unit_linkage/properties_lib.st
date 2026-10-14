INTERFACE IValue
    PROPERTY_GET value : DINT END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbBase IMPLEMENTS IValue
VAR
    _value : DINT;
END_VAR
    PROPERTY_GET value : DINT
        value := _value;
    END_PROPERTY

    PROPERTY_SET value : DINT
        _value := value;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbChild EXTENDS FbBase
    PROPERTY_GET value : DINT
        value := _value * 2;
    END_PROPERTY
END_FUNCTION_BLOCK

CLASS Counter
VAR
    _count : DINT := 7;
END_VAR
    PROPERTY_GET count : DINT
        count := _count;
    END_PROPERTY
END_CLASS
