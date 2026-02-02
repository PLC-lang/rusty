// RUN: (%COMPILE %s && %RUN) | %CHECK %s

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

    // CHECK: FB2
    reference := instanceFb2;
    reference.printId();

    // CHECK: FB3
    reference := instanceFb3;
    reference.printId();
END_FUNCTION