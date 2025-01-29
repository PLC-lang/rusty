// Ignored for now
// NUR: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    PROPERTY foo : DINT
        GET
            printf('Inside foo$N');
            foo := bar;
        END_GET
    END_PROPERTY

    PROPERTY bar : DINT
        GET
            printf('Inside bar$N');
            bar := foo;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        x : DINT;
        instanceA : A;
    END_VAR

    printf('Inside main$N');
    x := instanceA.foo;
END_FUNCTION