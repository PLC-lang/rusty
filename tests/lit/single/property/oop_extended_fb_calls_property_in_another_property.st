// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    PROPERTY foo: DINT
        PROPERTY_GET foo: DINT
            foo := 69;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
    PROPERTY bar: DINT
        PROPERTY_GET bar: DINT
            bar := foo;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fbInstance: fb2;
    END_VAR

    printf('%d$N', fbInstance.bar); // CHECK: 69
END_FUNCTION
