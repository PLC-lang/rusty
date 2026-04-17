// RUN: (%COMPILE %s && %RUN) | %CHECK %s

INTERFACE intf
    PROPERTY_GET prop: DINT END_GET
    PROPERTY_SET prop: DINT END_SET
END_INTERFACE

FUNCTION_BLOCK fb IMPLEMENTS intf
    VAR
        localPrivateVariable: DINT := 69;
    END_VAR

    PROPERTY_GET prop: DINT
        prop := localPrivateVariable;
    END_GET

    PROPERTY_SET prop: DINT
        // This should never be called when using fb2 because it will be overridden
        localPrivateVariable := 0;
    END_SET
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
    PROPERTY_SET prop: DINT
        localPrivateVariable := prop;
    END_SET
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
