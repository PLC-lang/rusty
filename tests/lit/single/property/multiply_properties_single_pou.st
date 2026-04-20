// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    VAR
        foo : DINT;
    END_VAR

    PROPERTY_GET increaseFooBy10: DINT
        printf('Inside increaseFooBy10$N');
        foo := foo + 10;
    END_PROPERTY

    PROPERTY_GET increaseFooBy20: DINT
        printf('Inside increaseFooBy20$N');
        foo := foo + 20;
    END_PROPERTY

    PROPERTY_GET increaseFooBy30: DINT
        printf('Inside increaseFooBy30$N');
        increaseFooBy20;
        increaseFooBy10;
    END_PROPERTY

    PROPERTY_GET readFoo: DINT
        printf('Inside readFoo$N');
        readFoo := foo;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : A;
        result : DINT;
    END_VAR

    // CHECK: Inside increaseFooBy10
    instanceA.increaseFooBy10;

    // CHECK: Inside increaseFooBy20
    instanceA.increaseFooBy20;

    // CHECK: Inside increaseFooBy30
    // CHECK: Inside increaseFooBy20
    // CHECK: Inside increaseFooBy10
    instanceA.increaseFooBy30;

    // CHECK: Inside readFoo
    // CHECK: 60
    printf('%d$N', instanceA.readFoo);
END_FUNCTION
