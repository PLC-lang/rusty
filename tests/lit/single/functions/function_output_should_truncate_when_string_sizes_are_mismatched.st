// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION Config_GetString : DINT
    VAR_INPUT
        Key : STRING[1023];
    END_VAR
    VAR_OUTPUT
        Value : STRING[28];
    END_VAR

    Value := 'I am a non-truncated string';

END_FUNCTION

FUNCTION Config_GetString_Auto : DINT
    VAR_INPUT
        Key : STRING[1023];
    END_VAR
    VAR_OUTPUT
        Value : STRING;
    END_VAR

    Value := 'I am an automatically sized non-truncated string';

END_FUNCTION

PROGRAM mainProg
    VAR
        MyString : STRING[10];
        MyInt: INT := 1234;
        retValue : DINT;
    END_VAR

    retValue := Config_GetString('/some/config/value', MyString);

    // CHECK: I am a non
    PRINTF('%s$N', MyString);

    retValue := Config_GetString_Auto('/some/config/value', MyString);

    // CHECK: I am an au
    PRINTF('%s$N', MyString);
END_PROGRAM

FUNCTION main
    mainProg();
END_FUNCTION
