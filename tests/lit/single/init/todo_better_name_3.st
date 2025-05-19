// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    globalFoo: DINT := 15;
    globalBar: DINT := 30;
    globalBar2: DINT := 45;

    globalStructA: StructA := (instanceB := (bar := REF(globalBar2)));
    globalStructB: StructA;
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
    printf('%d$N', globalStructA.instanceB.foo^); // CHECK: 15
    printf('%d$N', globalStructA.instanceB.bar^); // CHECK: 45

    printf('%d$N', globalStructB.instanceB.foo^); // CHECK: 15
    printf('%d$N', globalStructC.instanceB.bar^); // CHECK: 30
END_FUNCTION