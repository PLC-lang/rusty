// RUN: (%COMPILE %s && %RUN) | %CHECK %s

PROGRAM testProg
VAR
    val : LREAL := 3.14;
END_VAR
VAR_IN_OUT
    ioVal : LREAL := 1.0;
    ioVal1 : LREAL;
END_VAR
    // CHECK: Value: 3.140000
    printf('Value: %f$N', val);
    // CHECK: IO Value: 1.000000
    printf('IO Value: %f$N', ioVal);
    // CHECK: IO Value: 5.000000
    printf('IO Value: %f$N', ioVal1);
END_PROGRAM

FUNCTION main : DINT
    VAR
        v : LREAL := 5.0;
        u : LREAL := 5.0;
    END_VAR
    testProg(v,u);
END_FUNCTION
