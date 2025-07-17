// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    vtable_FbA_instance: vtable_FbA;
END_VAR

TYPE vtable_FbA: STRUCT
    foo: POINTER TO FbA.foo := ADR(FbA.foo);
END_STRUCT END_TYPE

FUNCTION_BLOCK FbA
    VAR
        __vtable: POINTER TO __VOID := ADR(vtable_FbA_instance);
    END_VAR

    METHOD foo
        printf('Hello from FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK


FUNCTION main
    VAR
        instanceFbA: FbA;
    END_VAR

    vtable_FbA#(instanceFbA.__vtable).foo^(instanceFbA); // CHECK: Hello from FbA::foo
END_FUNCTION
