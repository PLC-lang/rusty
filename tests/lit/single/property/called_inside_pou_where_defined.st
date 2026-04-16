// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    PROPERTY sayCheese : DINT
        PROPERTY_GET sayCheese: DINT
            printf('Cheese');
        END_GET
    END_PROPERTY

    sayCheese;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : A;
    END_VAR

    // CHECK: Cheese
    instanceA();
END_FUNCTION
