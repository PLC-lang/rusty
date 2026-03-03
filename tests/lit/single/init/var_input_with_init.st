// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that VAR_INPUT variables can be initialized

PROGRAM TestVarInput
VAR_INPUT
    x : INT := 42;
    s : STRING := 'hello';
END_VAR
VAR
    result : INT;
END_VAR
    result := x;

    // CHECK: 42
    printf('%d$N', result);

    // CHECK: hello
    printf('%s$N', s);
END_PROGRAM

FUNCTION main: DINT
    TestVarInput();
END_FUNCTION
