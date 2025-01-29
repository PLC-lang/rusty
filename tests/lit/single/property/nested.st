// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    VAR
        instanceB : B;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK B
    VAR
        instanceC : C;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK C
    VAR
        localPrivateVariable : DINT := 69;
    END_VAR

    PROPERTY foo : DINT
        GET
            printf('Inside getter$N');
            foo := localPrivateVariable;
        END_GET

        SET
            printf('Inside setter$N');
            localPrivateVariable := localPrivateVariable + foo;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : A;
    END_VAR

    // CHECK: Inside getter
    // CHECK: 69
    printf('%d$N', instanceA.instanceB.instanceC.foo); 

    // CHECK: Inside setter
    instanceA.instanceB.instanceC.foo := 351;

    // CHECK: Inside getter
    // CHECK: 420
    printf('%d$N', instanceA.instanceB.instanceC.foo); 
END_FUNCTION