// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
    VAR
        fb1 : fb1;
        fb1_value_store : INT;

        fb2 : fb2;
        fb2_value_store : INT;
    END_VAR

    fb1_value_store := 51;
    fb1.value := fb1_value_store;

    fb2_value_store := 52;
    fb2.value := fb2_value_store;

    // CHECK: FB1-Value: 51
    printf('FB1-Value: %d$N', fb1.value);

    // CHECK: FB2-Value: 52
    printf('FB2-Value: %d$N', fb2.value);
END_PROGRAM

// INT Container
FUNCTION_BLOCK fb1
    VAR
        _value	: INT;
    END_VAR

    PROPERTY_GET value : REFERENCE TO INT
        value REF= _value;
    END_PROPERTY

    PROPERTY_SET value : REFERENCE TO INT
        _value := value;
    END_PROPERTY
END_FUNCTION_BLOCK

// LINT Container
FUNCTION_BLOCK fb2
    VAR
        _value	: INT;
    END_VAR

    PROPERTY_GET value : REFERENCE TO INT
        value REF= _value;
    END_PROPERTY

    PROPERTY_SET value : REFERENCE TO INT
        _value := value;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    test();
END_FUNCTION
