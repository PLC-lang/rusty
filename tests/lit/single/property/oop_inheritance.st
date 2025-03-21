// RUN: (%COMPILE %s && %RUN) | %CHECK %s

INTERFACE intf
    PROPERTY prop: DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK fb IMPLEMENTS intf
    VAR
        localPrivateVariable: DINT := 69;
    END_VAR

    PROPERTY prop: DINT
        GET
            prop := localPrivateVariable;
        END_GET

        SET
            // This should never be called when using fb2 because it will be overridden
            localPrivateVariable := 0;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
    PROPERTY prop: DINT
        SET
            localPrivateVariable := prop;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main: DINT
    VAR
        instance: fb2;
    END_VAR

    // This will call the getter from fb
    printf('%d$N', instance.prop); // CHECK: 69

    // This will call the setter from fb2
    instance.prop := 420;
    printf('%d$N', instance.prop); // CHECK: 420
END_FUNCTION