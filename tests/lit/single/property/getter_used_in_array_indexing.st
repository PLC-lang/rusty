// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    VAR
        localVariable: ARRAY[1..5] OF DINT := [2, 4, 8, 16, 32];
    END_VAR

    PROPERTY foo: ARRAY[1..5] OF DINT
        GET
            foo := localVariable;
        END_GET
    END_PROPERTY

    PROPERTY bar: DINT
        GET
            bar := 3;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: fb;

        arrayValue: ARRAY[1..5] OF DINT;
        arrayIndex: DINT;
    END_VAR

    printf('%d$N', instance.foo[instance.bar]); // CHECK: 8
END_FUNCTION
