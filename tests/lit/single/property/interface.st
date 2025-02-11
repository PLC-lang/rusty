// RUN: (%COMPILE %s && %RUN) | %CHECK %s
INTERFACE myInterface
    PROPERTY foo : DINT
        GET
        END_GET
    END_PROPERTY
    PROPERTY foo : DINT
        SET END_SET
    END_PROPERTY
END_INTERFACE
FUNCTION_BLOCK fb2 implements myInterface
    VAR
        bar : DINT;
    END_VAR
    PROPERTY foo : DINT
        GET
            printf('in fb.fb2.foo getter - bar = %d$N', bar);
            foo := bar * 2;
            printf('in fb.fb2.foo getter - bar = %d$N', bar);
        END_GET
        SET
            printf('in fb.fb2.foo setter - bar = %d$N', bar);
          bar := foo + 5;
            printf('in fb.fb2.foo setter - bar = %d$N', bar);
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK
FUNCTION_BLOCK fb implements myInterface
    VAR
        bar : DINT;
        fubo : fb2;
    END_VAR
    PROPERTY foo : DINT
        GET
            foo := fubo.foo * 10;
        END_GET
        SET
          fubo.foo := foo / 2;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        x : DINT;
        instanceA : fb;
    END_VAR
    instanceA.foo := 20;
    printf('instanceA.foo = %d$n', instanceA.foo); // CHECK: instanceA.foo = 300
    printf('instanceA.fubo.foo = %d$n', instanceA.fubo.foo); // CHECK: instanceA.fubo.foo = 30
END_FUNCTION
