// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    globalValueA: DINT := 5;
    globalValueB: DINT := 10;
    globalValueC: DINT := 15;

    globalStructOne: StructA;
    globalStructTwo: StructA := (value := REF(globalValueA), instanceB := (value := REF(globalValueB), instanceC := (value := REF(globalValueC))));
END_VAR

TYPE StructA:
    STRUCT
        value: REF_TO DINT := REF(globalValueC);
        instanceB: StructB;
    END_STRUCT
END_TYPE

TYPE StructB:
    STRUCT
        value: REF_TO DINT := REF(globalValueA);
        instanceC: StructC;
    END_STRUCT
END_TYPE

TYPE StructC:
    STRUCT
        value: REF_TO DINT := REF(globalValueB);
    END_STRUCT
END_TYPE

FUNCTION main
    printf('%d$N', globalStructOne.value^); // CHECK: 15
    printf('%d$N', globalStructOne.instanceB.value^); // CHECK: 5
    printf('%d$N', globalStructOne.instanceB.instanceC.value^); // CHECK: 10

    printf('%d$N', globalStructTwo.value^); // CHECK: 5
    printf('%d$N', globalStructTwo.instanceB.value^); // CHECK: 10
    printf('%d$N', globalStructTwo.instanceB.instanceC.value^); // CHECK: 15
END_FUNCTION