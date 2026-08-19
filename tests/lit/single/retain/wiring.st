// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL RETAIN
    globalVar : DINT;
END_VAR

PROGRAM mainProg
    VAR RETAIN
        prgVar : DINT;
        prgArr : ARRAY[0..100] OF DINT;
    END_VAR

    prgVar := 20;
    globalVar := 50;
    prgArr[100] := 30;
END_PROGRAM

FUNCTION main : DINT
    mainProg();
    printf('%d$N', mainProg.prgVar); //CHECK: 20
    // Implicitly created global variable
    printf('%d$N', __mainProg_prgVar__retain); //CHECK-NEXT: 20
    printf('%d$N', globalVar); //CHECK-NEXT: 50
    // Variables with an inline type definition get the same treatment
    printf('%d$N', mainProg.prgArr[100]); //CHECK-NEXT: 30
    printf('%d$N', __mainProg_prgArr__retain[100]); //CHECK-NEXT: 30
END_FUNCTION