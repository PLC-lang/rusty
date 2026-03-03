// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    alias AT %IX1.1 : BOOL;
END_VAR

PROGRAM prog
VAR
    tmp : BOOL;
END_VAR
    // CHECK: 1
    alias := TRUE;
    tmp := %IX1.1;
    printf('%u$N', tmp);

    // CHECK: 0
    %IX1.1 := FALSE;
    tmp := alias;
    printf('%u$N', tmp);
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
