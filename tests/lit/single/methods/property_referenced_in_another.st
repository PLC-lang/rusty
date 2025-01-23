// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fbA
    VAR
        localPrivateVariable : DINT;
    END_VAR

    PROPERTY foo : DINT
        SET
            localPrivateVariable := foo + 1;
        END_SET

        GET
            foo := localPrivateVariable;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK fbB
    VAR
        fbAInstance : fbA;
    END_VAR

    METHOD bar : DINT
        GET
            fbAInstance.foo := 5;
            bar := fbAInstance.foo;
        END_GET
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        localVariable : DINT;
        fbBInstance : fbB;
    END_VAR

    localVariable := fbBInstance.foo;
    printf('%d$N',localVariable); // CHECK: 6
END_FUNCTION
