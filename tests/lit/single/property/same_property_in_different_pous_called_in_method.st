// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK A
    VAR
        x : DINT := 42;
    END_VAR
    PROPERTY foo : DINT
        GET
            foo := x;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK B
    VAR
        x : DINT := 69;
    END_VAR
    PROPERTY foo : DINT
        GET
            foo := x;
        END_GET
    END_PROPERTY

    METHOD bar : DINT
    VAR
        instanceA : A;
    END_VAR
        bar := foo;
    END_METHOD
    
    METHOD baz : DINT
    VAR
        instanceA : A;
    END_VAR
        baz := instanceA.foo;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceB : B;
    END_VAR

    printf('%d$N', instanceB.bar()); // CHECK: 69
    printf('%d$N', instanceB.baz()); // CHECK: 42
END_FUNCTION