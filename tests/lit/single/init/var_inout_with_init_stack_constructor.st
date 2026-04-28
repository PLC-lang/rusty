// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that VAR_IN_OUT with initializers keeps passed values

FUNCTION test_inout_no_init
VAR_IN_OUT
    x : INT;
END_VAR
    // CHECK: 42
    printf('%d$N', x);
END_FUNCTION

PROGRAM prog
VAR
    v : INT;
END_VAR
    v := 42;
    test_inout_no_init(v);
END_PROGRAM

FUNCTION main: DINT
    prog();
END_FUNCTION
