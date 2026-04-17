// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK A
    VAR
        x : DINT := 42;
    END_VAR
    PROPERTY_GET foo: DINT
        foo := x;
    END_GET

    METHOD bar : DINT
        bar := foo;
    END_METHOD
END_FUNCTION_BLOCK

ACTIONS
    ACTION baz
        // CHECK: 42
        printf('%d$N', bar());
        // CHECK: 42
        printf('%d$N', foo);
    END_ACTION
END_ACTIONS

FUNCTION main
    VAR
        instanceA : A;
    END_VAR
        instanceA.baz();
END_FUNCTION
