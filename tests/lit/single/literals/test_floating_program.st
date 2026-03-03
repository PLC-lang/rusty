// RUN: (%COMPILE %s && %RUN) | %CHECK %s

PROGRAM testProg
VAR
    val : LREAL := 3.14;
END_VAR
    // CHECK: Value: 3.140000
    printf('Value: %f$N', val);
END_PROGRAM

FUNCTION main : DINT
    testProg();
END_FUNCTION
