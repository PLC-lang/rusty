// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    globalFoo: DINT := 15;
    globalBar: DINT := 30;
    globalBar2: DINT := 45;

    globalStructOne: StructA;
    globalStructTwo: StructA := (instanceB := (bar := REF(globalBar2)));
END_VAR

TYPE StructA:
    STRUCT
        instanceB: StructB := (foo := REF(globalFoo));
    END_STRUCT
END_TYPE

TYPE StructB:
    STRUCT
        foo: REF_TO DINT;
        bar: REF_TO DINT := REF(globalBar);
    END_STRUCT
END_TYPE

FUNCTION main
    printf('%d$N', globalStructOne.instanceB.foo^); // CHECK: 15
    printf('%d$N', globalStructOne.instanceB.bar^); // CHECK: 30

    printf('%d$N', globalStructTwo.instanceB.foo^); // CHECK: 15
    printf('%d$N', globalStructTwo.instanceB.bar^); // CHECK: 45
END_FUNCTION