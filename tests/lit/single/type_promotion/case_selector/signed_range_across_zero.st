// RUN: %COMPILE %s && %RUN | %CHECK %s
// Range labels that cross zero on a signed selector.
FUNCTION branch_sint : DINT
    VAR_INPUT
        x : SINT;
    END_VAR

    CASE x OF
        -128..-100: branch_sint := 1;
        -10..10: branch_sint := 2;
        100..127: branch_sint := 3;
    ELSE
        branch_sint := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_int : DINT
    VAR_INPUT
        x : INT;
    END_VAR

    CASE x OF
        -32768..-30000: branch_int := 1;
        -1000..1000: branch_int := 2;
    ELSE
        branch_int := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_dint : DINT
    VAR_INPUT
        x : DINT;
    END_VAR

    CASE x OF
        -2147483648..-2000000000: branch_dint := 1;
        -1000000000..1000000000: branch_dint := 2;
    ELSE
        branch_dint := 9;
    END_CASE
END_FUNCTION

FUNCTION branch_lint : DINT
    VAR_INPUT
        x : LINT;
    END_VAR

    CASE x OF
        -9223372036854775807..-9000000000000000000: branch_lint := 1;
        -5000000000..5000000000: branch_lint := 2;
    ELSE
        branch_lint := 9;
    END_CASE
END_FUNCTION

FUNCTION main : DINT
    // SINT
    printf('SINT -128 -> %d$N', branch_sint(-128));         // CHECK: SINT -128 -> 1
    printf('SINT -5 -> %d$N', branch_sint(-5));             // CHECK-NEXT: SINT -5 -> 2
    printf('SINT 5 -> %d$N', branch_sint(5));               // CHECK-NEXT: SINT 5 -> 2
    printf('SINT 127 -> %d$N', branch_sint(127));           // CHECK-NEXT: SINT 127 -> 3
    printf('SINT -50 -> %d$N', branch_sint(-50));           // CHECK-NEXT: SINT -50 -> 9

    // INT
    printf('INT -32768 -> %d$N', branch_int(-32768));       // CHECK-NEXT: INT -32768 -> 1
    printf('INT -500 -> %d$N', branch_int(-500));           // CHECK-NEXT: INT -500 -> 2
    printf('INT 2000 -> %d$N', branch_int(2000));           // CHECK-NEXT: INT 2000 -> 9

    // DINT
    printf('DINT -2147483648 -> %d$N', branch_dint(-2147483648));   // CHECK-NEXT: DINT -2147483648 -> 1
    printf('DINT -7 -> %d$N', branch_dint(-7));                     // CHECK-NEXT: DINT -7 -> 2

    // LINT
    printf('LINT -9100000000000000000 -> %d$N', branch_lint(-9100000000000000000));    // CHECK-NEXT: LINT -9100000000000000000 -> 1
    printf('LINT -4000000000 -> %d$N', branch_lint(-4000000000));                      // CHECK-NEXT: LINT -4000000000 -> 2
END_FUNCTION
