// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
    VAR
        fbPrivateVariable : DINT;
    END_VAR

    PROPERTY foo : DINT
        GET
            printf('Called getter (current value %d)$N', foo);
        END_GET

        SET
            printf('Called setter (with value %d)$N', foo);
        END_SET
    END_PROPERTY

    METHOD bar : DINT
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance : fb;
        mainPrivateVariable : DINT;
    END_VAR

    instance.foo := 5;                      // CHECK: Called setter (with value 5)
    mainPrivateVariable := instance.foo;    // CHECK: Called getter (current value 5)

    instance.foo := 5 + 5;                  // CHECK: Called setter (with value 10)
    mainPrivateVariable := instance.foo;    // CHECK: Called getter (current value 10)
END_FUNCTION
