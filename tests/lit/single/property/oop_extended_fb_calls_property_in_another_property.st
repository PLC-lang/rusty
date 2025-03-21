// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK fb
    PROPERTY foo: DINT
        GET
            foo := 69;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
    PROPERTY bar: DINT
        GET
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