// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A property getter result is passed directly as an argument to a function call.
// Verifies that the getter dispatch and argument passing compose correctly.

INTERFACE IA
    PROPERTY value : DINT
        GET END_GET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    PROPERTY value : DINT
        GET
            value := 42;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    PROPERTY value : DINT
        GET
            value := 99;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION consume
    VAR_INPUT
        x : DINT;
    END_VAR

    printf('got %d$N', x);
END_FUNCTION

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        reference : IA;
    END_VAR

    reference := instanceA;
    consume(reference.value);       // CHECK: got 42

    reference := instanceB;
    consume(reference.value);       // CHECK: got 99
END_FUNCTION
