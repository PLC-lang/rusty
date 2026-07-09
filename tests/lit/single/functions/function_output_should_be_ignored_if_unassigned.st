// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION GetVal : DINT
    VAR_OUTPUT
        Value : STRING[80];
    END_VAR

    Value := 'Hello World';

    GetVal := 2;
END_FUNCTION

PROGRAM mainProg
    VAR
        s : STRING;
    END_VAR

    // CHECK: 2
    IF GetVal(Value => ) = 0 THEN
        PRINTF('0$N');
    ELSE
        PRINTF('2$N');
    END_IF
END_PROGRAM

FUNCTION main
    mainProg();
END_FUNCTION
