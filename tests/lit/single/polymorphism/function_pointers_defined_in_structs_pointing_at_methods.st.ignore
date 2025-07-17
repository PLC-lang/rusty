// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE FnCollection:
    STRUCT
        foo : REF_TO FnBlockA.foo := REF(FnBlockA.foo);
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FnBlockA
    METHOD foo
        printf('Hello from foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fnCollectionInstance : FnCollection;
        instance: FnBlockA;
    END_VAR

    fnCollectionInstance.foo^(instance); // CHECK: Hello from foo
END_FUNCTION
