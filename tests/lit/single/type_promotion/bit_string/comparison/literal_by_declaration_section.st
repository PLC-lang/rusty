// RUN: %COMPILE %s && %RUN | %CHECK %s
// A compare of a DWORD or LWORD operand with an untyped literal is unsigned, wherever the operand is declared.
VAR_GLOBAL
    g : DWORD := 16#FFFF_FFFF;
END_VAR

VAR_GLOBAL CONSTANT
    gc : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
END_VAR

FUNCTION_BLOCK probe
    VAR_INPUT
        i : DWORD;
    END_VAR
    VAR_IN_OUT
        io : LWORD;
    END_VAR
    VAR_OUTPUT
        o : DWORD;
    END_VAR
    VAR
        m : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR
    VAR CONSTANT
        c : DWORD := 16#FFFF_FFFF;
    END_VAR
    VAR_TEMP
        t : DWORD;
    END_VAR

    o := 16#FFFF_FFFF;
    t := 16#FFFF_FFFF;
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
        v : LWORD := 16#FFFF_FFFF_FFFF_FFFF;
    END_VAR

    inst(i := 16#FFFF_FFFF, io := v);
    // CHECK: VAR_INPUT > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_IN_OUT > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_OUTPUT < 0 = 0
    // CHECK-NEXT: VAR > 3000000000 = 1
    // CHECK-NEXT: VAR CONSTANT > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_TEMP < 0 = 0
    // CHECK-NEXT: VAR_GLOBAL > 16#7FFFFFFF = 1
    // CHECK-NEXT: VAR_GLOBAL CONSTANT < 0 = 0
END_FUNCTION
