// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    // globalStructA: StructA := (value := - 5, instanceB := (value := - 10, instanceC := (value := - 15))); // FIXME: Currently not working
    globalStructA: StructA := (value := - 5, instanceB := (value := - 10));
END_VAR

TYPE StructA:
    STRUCT
        value: DINT := 5;
        instanceB: StructB := (value := 5);
    END_STRUCT
END_TYPE

TYPE StructB:
    STRUCT
        value: DINT := 10;
        instanceC: StructC := (value := 10);
    END_STRUCT
END_TYPE

TYPE StructC:
    STRUCT
        value: DINT := 15;
    END_STRUCT
END_TYPE

FUNCTION main
    printf('%d$N', globalStructA.value); // CHECK: -5
    printf('%d$N', globalStructA.instanceB.value); // CHECK: -10
    printf('%d$N', globalStructA.instanceB.instanceC.value); // CHECK: 10
END_FUNCTION