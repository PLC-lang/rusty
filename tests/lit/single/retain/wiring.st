// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL RETAIN
    globalVar : DINT;
END_VAR

PROGRAM mainProg
    VAR RETAIN
        prgVar : DINT;
    END_VAR

    prgVar := 20;
    globalVar := 50;
END_PROGRAM

FUNCTION main : DINT
    mainProg();
    printf('%d$N', mainProg.prgVar); //CHECK: 20
    // Implicitly created global variable
    printf('%d$N', __mainProg_prgVar__retain); //CHECK-NEXT: 20
    printf('%d$N', globalVar); //CHECK-NEXT: 50
END_FUNCTION