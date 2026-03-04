// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface with a read-only property (GET only). Two function blocks implement it
// with different backing logic. Verifies getter-only dispatch works.

INTERFACE IA
    PROPERTY value : DINT
        GET END_GET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK Counter IMPLEMENTS IA
    VAR
        _count : DINT := 0;
    END_VAR

    PROPERTY value : DINT
        GET
            _count := _count + 1;
            value := _count;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FixedValue IMPLEMENTS IA
    PROPERTY value : DINT
        GET
            value := 999;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        counter : Counter;
        fixed : FixedValue;
        reference : IA;
    END_VAR

    reference := counter;
    printf('%d$N', reference.value);    // CHECK: 1
    printf('%d$N', reference.value);    // CHECK: 2
    printf('%d$N', reference.value);    // CHECK: 3

    reference := fixed;
    printf('%d$N', reference.value);    // CHECK: 999
    printf('%d$N', reference.value);    // CHECK: 999
END_FUNCTION
