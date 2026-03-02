// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that VAR_OUTPUT variables can be initialized

PROGRAM TestVarOutput
VAR_OUTPUT
    result : INT := 99;
    msg : STRING := 'output';
END_VAR

    // CHECK: 99
    printf('%d$N', result);

    // CHECK: output
    printf('%s$N', msg);
END_PROGRAM

FUNCTION main: DINT
    TestVarOutput();
END_FUNCTION
