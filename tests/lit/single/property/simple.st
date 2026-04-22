// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
    VAR
        localPrivateVariable : DINT := 5;
    END_VAR

    PROPERTY_GET foo: DINT
        foo := localPrivateVariable;
    END_PROPERTY

    PROPERTY_SET foo: DINT
        localPrivateVariable := foo + 5;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        localVariable : DINT;
        instance : fb;
    END_VAR

    // Retrieving the getter return value using another variable
    localVariable := instance.foo;
    printf('%d$N', localVariable);  // CHECK: 5

    // Retrieving the getter return value using the property directly
    printf('%d$N', instance.foo);   // CHECK: 5

    instance.foo := 10;
    printf('%d$N', instance.foo);   // CHECK: 15
END_FUNCTION
