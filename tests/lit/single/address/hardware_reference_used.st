// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    a AT %IX1.2.1 : BOOL;
    b AT %QX1.2.2 : BOOL;
    c AT %ID1.2.3 : DWORD;
    d AT %ID1.2.4 : DWORD;
END_VAR

FUNCTION main : DINT
    __init___hardware_reference_used_st();

    a := TRUE;
    printf('%u$N', a); //CHECK: 1
    b := TRUE;
    printf('%u$N', b); //CHECK: 1
    c := 1337;
    printf('%d$N', c); //CHECK: 1337
    d := 98765;
    printf('%d$N', d); //CHECK: 98765
END_FUNCTION
