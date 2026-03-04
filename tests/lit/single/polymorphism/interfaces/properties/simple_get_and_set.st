// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A single interface with a property (GET and SET). Two function blocks implement it.
// Assigning each to an interface variable and accessing the property should dispatch to the
// correct implementation.

INTERFACE IA
    PROPERTY foo : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY foo : DINT
        GET
            foo := _val;
        END_GET

        SET
            _val := foo;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    VAR
        _val : DINT := 20;
    END_VAR

    PROPERTY foo : DINT
        GET
            foo := _val + 100;
        END_GET

        SET
            _val := foo + 100;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        reference : IA;
    END_VAR

    // Dispatch through FbA
    reference := instanceA;
    printf('%d$N', reference.foo);      // CHECK: 10

    reference.foo := 42;
    printf('%d$N', reference.foo);      // CHECK: 42

    // Dispatch through FbB
    reference := instanceB;
    printf('%d$N', reference.foo);      // CHECK: 120

    reference.foo := 5;
    printf('%d$N', reference.foo);      // CHECK: 205
END_FUNCTION
