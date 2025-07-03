// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    globalOne: DINT := 1;
    globalTwo: DINT := 2;
    globalThree: DINT := 3;
    globalFour: DINT := 4;
    globalFive: DINT := 5;
    globalSix: DINT := 6;
    globalSeven: DINT := 7;
    globalEight: DINT := 8;
    globalNine: DINT := 9;

    globalA: MyStruct2 := (a := (c := REF(globalFive), b := REF(globalSeven)), b := (a := REF(globalThree), b := REF(globalTwo)));
    globalB: MyStruct2 := (b := (a := REF(globalNine)));
END_VAR

TYPE MyStruct2: STRUCT
    a: MyStruct := (a := REF(globalFive), b := REF(globalFour));
    b: MyStruct := (c := REF(globalSeven));
END_STRUCT
END_TYPE

TYPE MyStruct: STRUCT
    a: REF_TO DINT := REF(globalOne);
    b: REF_TO DINT := REF(globalTwo);
    c: REF_TO DINT := REF(globalThree);
END_STRUCT
END_TYPE

FUNCTION main
    printf('%d$N', globalA.a.a^); // CHECK: 5
    printf('%d$N', globalA.a.b^); // CHECK: 7
    printf('%d$N', globalA.a.c^); // CHECK: 5
    printf('%d$N', globalA.b.a^); // CHECK: 3
    printf('%d$N', globalA.b.b^); // CHECK: 2
    printf('%d$N', globalA.b.c^); // CHECK: 7

    printf('%d$N', globalB.b.a^); // CHECK: 9
    printf('%d$N', globalB.b.b^); // CHECK: 2
    printf('%d$N', globalB.b.c^); // CHECK: 7
END_FUNCTION