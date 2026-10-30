// RUN: %COMPILE %s && %RUN | %CHECK %s
// A UDINT operand divided by an untyped literal stays UDINT, wherever the operand is declared.
VAR_GLOBAL
    g : UDINT := 4294967295;
END_VAR

VAR_GLOBAL CONSTANT
    gc : UDINT := 4294967295;
END_VAR

FUNCTION_BLOCK probe
    VAR_INPUT
        i : UDINT;
    END_VAR
    VAR_INPUT CONSTANT
        ic : UDINT := 4294967295;
    END_VAR
    VAR_IN_OUT
        io : UDINT;
    END_VAR
    VAR_OUTPUT
        o : UDINT;
    END_VAR
    VAR CONSTANT
        c : UDINT := 4294967295;
    END_VAR
    VAR_TEMP
        t : UDINT;
    END_VAR
    VAR
        q : ULINT;
    END_VAR

    o := 4294967295;
    t := 4294967295;
    q := i / 2;
    printf('VAR_INPUT / 2 = %llu$N', q);
    q := ic MOD 7;
    printf('VAR_INPUT CONSTANT MOD 7 = %llu$N', q);
    q := io / 2;
    printf('VAR_IN_OUT / 2 = %llu$N', q);
    q := io MOD 7;
    printf('VAR_IN_OUT MOD 7 = %llu$N', q);
    q := o / 2;
    printf('VAR_OUTPUT / 2 = %llu$N', q);
    q := c MOD 7;
    printf('VAR CONSTANT MOD 7 = %llu$N', q);
    q := t / 2;
    printf('VAR_TEMP / 2 = %llu$N', q);
    q := g MOD 7;
    printf('VAR_GLOBAL MOD 7 = %llu$N', q);
    q := gc / 2;
    printf('VAR_GLOBAL CONSTANT / 2 = %llu$N', q);
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        inst : probe;
        v : UDINT := 4294967295;
    END_VAR

    inst(i := 4294967295, io := v);
    // CHECK: VAR_INPUT / 2 = 2147483647
    // CHECK-NEXT: VAR_INPUT CONSTANT MOD 7 = 3
    // CHECK-NEXT: VAR_IN_OUT / 2 = 2147483647
    // CHECK-NEXT: VAR_IN_OUT MOD 7 = 3
    // CHECK-NEXT: VAR_OUTPUT / 2 = 2147483647
    // CHECK-NEXT: VAR CONSTANT MOD 7 = 3
    // CHECK-NEXT: VAR_TEMP / 2 = 2147483647
    // CHECK-NEXT: VAR_GLOBAL MOD 7 = 3
    // CHECK-NEXT: VAR_GLOBAL CONSTANT / 2 = 2147483647
END_FUNCTION
