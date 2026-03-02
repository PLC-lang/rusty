// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test mixed VAR_INPUT and VAR_IN_OUT parameters

FUNCTION test_mixed_params
VAR_INPUT
    input_val : INT;
    input_str : STRING;
END_VAR
VAR_IN_OUT
    inout_val : INT;
END_VAR
    // CHECK: 5
    printf('%d$N', input_val);

    // CHECK: hello
    printf('%s$N', input_str);

    // CHECK: 42
    printf('%d$N', inout_val);
END_FUNCTION

PROGRAM prog
VAR
    x : INT;
END_VAR
    x := 42;
    test_mixed_params(5, 'hello', x);
END_PROGRAM

FUNCTION main: DINT
    prog();
END_FUNCTION
