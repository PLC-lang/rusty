// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Single interface which multiple function blocks implement and as a result should all be
// assignable to the interface variable in main.

INTERFACE Identifier
    METHOD printId
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FB1 IMPLEMENTS Identifier
    METHOD printId
        printf('FB1$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB2 IMPLEMENTS Identifier
    METHOD printId
        printf('FB2$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB3 IMPLEMENTS Identifier
    METHOD printId
        printf('FB3$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFb1: FB1;
        instanceFb2: FB2;
        instanceFb3: FB3;

        reference: Identifier;
    END_VAR

    // CHECK: FB1
    reference := instanceFb1;
    reference.printId();

    // CHECK-NEXT: FB2
    reference := instanceFb2;
    reference.printId();

    // CHECK-NEXT: FB3
    reference := instanceFb3;
    reference.printId();
END_FUNCTION