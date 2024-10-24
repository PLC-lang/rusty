// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    a AT %IX1.2.1 : BOOL;
    b AT %QX1.2.2 : BOOL;
    c AT %ID1.2.3 : DWORD;
    d AT %ID1.2.4 : DWORD;
END_VAR

FUNCTION main : DINT
    %IX1.2.1 := TRUE;
    printf('%u$N', a); //CHECK: 1
    b := TRUE;
    printf('%u$N', %QX1.2.2); //CHECK: 1
    %IX1.2.3 := 1337;
    printf('%d$N', c); //CHECK: 1337
    d := 98765;
    printf('%d$N', %QX1.2.4); //CHECK: 98765
END_FUNCTION