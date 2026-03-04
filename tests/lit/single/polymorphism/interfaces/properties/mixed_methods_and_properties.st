// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface declares both regular methods and properties. Verifies that methods and
// property accessors coexist and dispatch correctly through the same interface variable.

INTERFACE IA
    METHOD greet END_METHOD

    PROPERTY name : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    VAR
        _id : DINT := 1;
    END_VAR

    METHOD greet
        printf('Hello from A$N');
    END_METHOD

    PROPERTY name : DINT
        GET
            name := _id;
        END_GET

        SET
            _id := name;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    VAR
        _id : DINT := 2;
    END_VAR

    METHOD greet
        printf('Hello from B$N');
    END_METHOD

    PROPERTY name : DINT
        GET
            name := _id;
        END_GET

        SET
            _id := name;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        reference : IA;
    END_VAR

    reference := instanceA;
    reference.greet();                  // CHECK: Hello from A
    printf('%d$N', reference.name);     // CHECK: 1

    reference := instanceB;
    reference.greet();                  // CHECK: Hello from B
    printf('%d$N', reference.name);     // CHECK: 2

    reference.name := 42;
    printf('%d$N', reference.name);     // CHECK: 42
END_FUNCTION
