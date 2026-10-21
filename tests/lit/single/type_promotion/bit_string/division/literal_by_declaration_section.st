// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A DWORD operand divided by an untyped literal is converted to DINT, wherever the operand is declared.
VAR_GLOBAL
    g : DWORD := 16#FFFF_FFFF;
END_VAR

VAR_GLOBAL CONSTANT
    gc : DWORD := 16#FFFF_FFFF;
END_VAR

FUNCTION_BLOCK probe
    VAR_INPUT
        i : DWORD;
    END_VAR
    VAR_INPUT CONSTANT
        ic : DWORD := 16#FFFF_FFFF;
    END_VAR
    VAR_IN_OUT
        io : DWORD;
    END_VAR
    VAR_OUTPUT
        o : DWORD;
    END_VAR
    VAR CONSTANT
        c : DWORD := 16#FFFF_FFFF;
    END_VAR
    VAR_TEMP
        t : DWORD;
    END_VAR
    VAR
        q : ULINT;
    END_VAR

    o := 16#FFFF_FFFF;
    t := 16#FFFF_FFFF;
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
        v : DWORD := 16#FFFF_FFFF;
    END_VAR

    inst(i := 16#FFFF_FFFF, io := v);
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
