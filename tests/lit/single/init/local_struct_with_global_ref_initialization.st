// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    globalVar: DINT := 10;
END_VAR

TYPE Parent:
    STRUCT
        foo: REF_TO DINT;
    END_STRUCT
END_TYPE

TYPE Child:
    STRUCT
        instance: Parent := (foo := REF(globalVar));
    END_STRUCT
END_TYPE

FUNCTION main
    VAR
        localChild: Child;
    END_VAR

    // CHECK: Value: 10
    printf('Value: %d$N', localChild.instance.foo^);
END_FUNCTION