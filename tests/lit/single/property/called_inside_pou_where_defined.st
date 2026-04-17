// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    PROPERTY_GET sayCheese: DINT
        printf('Cheese');
    END_GET

    sayCheese;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : A;
    END_VAR

    // CHECK: Cheese
    instanceA();
END_FUNCTION
