// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that VAR_IN_OUT parameters without initializers keep their passed values
// Note: VAR_IN_OUT initializers are NOT applied in the current implementation

FUNCTION test_no_init
VAR_IN_OUT
    value : INT;
END_VAR
    // CHECK: 42
    printf('%d$N', value);
END_FUNCTION

FUNCTION main: DINT
    VAR
        x : INT;
    END_VAR
    x := 42;
    test_no_init(x);
END_FUNCTION
