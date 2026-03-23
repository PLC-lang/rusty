// RUN: (%COMPILE %s && %RUN) | %CHECK %s

PROGRAM testProg
VAR_IN_OUT
    ioValue : LREAL := 3.14;
END_VAR
    // CHECK: Value: 3.140000
    printf('Value: %f$N', ioValue);
END_PROGRAM

FUNCTION main : DINT
    VAR
        value : LREAL := 1.0;
    END_VAR
    testProg(value);
END_FUNCTION
