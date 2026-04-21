// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb1
    VAR
        localPrivateVariable: DINT := 68;
    END_VAR

    PROPERTY prop: DINT
        GET
            prop := localPrivateVariable;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb1
    PROPERTY prop: DINT
        SET
            VAR
                x: DINT;
            END_VAR

            x := prop + 1;
            localPrivateVariable := x;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: fb2;
    END_VAR

    instance.prop := 68;
    printf('%d$N', instance.prop); // CHECK: 69
END_FUNCTION