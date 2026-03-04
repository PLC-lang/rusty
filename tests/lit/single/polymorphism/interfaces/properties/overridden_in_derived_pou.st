// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: FbB extends FbA and overrides the getter. Both are assigned to the same interface
// variable. Verifies the overridden getter dispatches correctly while the setter from FbA is
// inherited.

INTERFACE IA
    PROPERTY foo : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    VAR
        _val : DINT := 0;
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

FUNCTION_BLOCK FbB EXTENDS FbA
    PROPERTY foo : DINT
        GET
            // Override: adds 1000 to distinguish from FbA's getter
            foo := _val + 1000;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        reference : IA;
    END_VAR

    // FbA getter
    reference := instanceA;
    reference.foo := 5;
    printf('%d$N', reference.foo);      // CHECK: 5

    // FbB overrides getter, inherits setter from FbA
    reference := instanceB;
    reference.foo := 7;
    printf('%d$N', reference.foo);      // CHECK: 1007
END_FUNCTION
