// RUN: %COMPILE %s && %RUN | %CHECK %s
// A compare of a UDINT or ULINT operand with an untyped literal is unsigned, wherever the operand is declared.
VAR_GLOBAL
    g : UDINT := 4294967295;
END_VAR

VAR_GLOBAL CONSTANT
    gc : ULINT := 18446744073709551615;
END_VAR

FUNCTION_BLOCK probe
    VAR_INPUT
        i : UDINT;
    END_VAR
    VAR_IN_OUT
        io : ULINT;
    END_VAR
    VAR_OUTPUT
        o : UDINT;
    END_VAR
    VAR
        m : ULINT := 18446744073709551615;
    END_VAR
    VAR CONSTANT
        c : UDINT := 4294967295;
    END_VAR
    VAR_TEMP
        t : UDINT;
    END_VAR

    o := 4294967295;
    t := 4294967295;
    printf('VAR_INPUT > 16#7FFFFFFF = %d$N', i > 16#7FFFFFFF);
    printf('VAR_IN_OUT > 16#7FFFFFFF = %d$N', io > 16#7FFFFFFF);
    printf('VAR_OUTPUT < 0 = %d$N', o < 0);
    printf('VAR > 3000000000 = %d$N', m > 3000000000);
    printf('VAR CONSTANT > 16#7FFFFFFF = %d$N', c > 16#7FFFFFFF);
    printf('VAR_TEMP < 0 = %d$N', t < 0);
    printf('VAR_GLOBAL > 16#7FFFFFFF = %d$N', g > 16#7FFFFFFF);
    printf('VAR_GLOBAL CONSTANT < 0 = %d$N', gc < 0);
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        inst : probe;
        v : ULINT := 18446744073709551615;
    END_VAR

    inst(i := 4294967295, io := v);
    // CHECK: VAR_INPUT > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_IN_OUT > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_OUTPUT < 0 = 0
    // CHECK-NEXT: VAR > 3000000000 = 1
    // CHECK-NEXT: VAR CONSTANT > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_TEMP < 0 = 0
    // CHECK-NEXT: VAR_GLOBAL > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_GLOBAL CONSTANT < 0 = 0
END_FUNCTION
