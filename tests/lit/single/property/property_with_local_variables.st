// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
    VAR
        localPrivateVariable: DINT := 5;
    END_VAR

    PROPERTY foo: DINT
        GET
            VAR
                one: DINT := 1;
            END_VAR

            foo := localPrivateVariable + one;
        END_GET

        SET
            VAR
                two: DINT := 2;
                three: DINT := 3;
            END_VAR
            localPrivateVariable := two + foo + three;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        localVariable: DINT;
        instance: fb;
    END_VAR

    // Retrieving the getter return value using another variable
    localVariable := instance.foo;
    printf('%d$N', localVariable); // CHECK: 6

    // Retrieving the getter return value using the property directly
    printf('%d$N', instance.foo); // CHECK: 6

    instance.foo := 10;
    printf('%d$N', instance.foo); // CHECK: 16
END_FUNCTION