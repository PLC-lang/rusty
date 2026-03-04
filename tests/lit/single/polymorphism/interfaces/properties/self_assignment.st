// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Self-assignment through an interface: reference.foo := reference.foo.
// Verifies that both getter and setter dispatch correctly in the same statement.

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
            _val := foo + 1;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : FbA;
        reference : IA;
    END_VAR

    reference := instance;

    // Self-assignment: get returns 10, set stores 10+1=11
    reference.foo := reference.foo;
    printf('%d$N', reference.foo);      // CHECK: 11

    // Again: get returns 11, set stores 11+1=12
    reference.foo := reference.foo;
    printf('%d$N', reference.foo);      // CHECK: 12
END_FUNCTION
