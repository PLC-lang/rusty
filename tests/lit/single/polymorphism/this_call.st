// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD foo
        printf('A::foo$N');
        THIS^();
    END_METHOD

    printf('A::body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD bar
        THIS^();
    END_METHOD

    printf('B::body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    METHOD foo
        printf('C::foo$N');
        THIS^();
    END_METHOD

    printf('C::body$N');
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;

        refInstanceA: POINTER TO A;
    END_VAR

    // CHECK: A::foo
    // CHECK: A::body
    refInstanceA := ADR(instanceA);
    refInstanceA^.foo();

    // CHECK: A::foo
    // CHECK: A::body
    refInstanceA := ADR(instanceB);
    refInstanceA^.foo();

    // CHECK: C::foo
    // CHECK: C::body
    refInstanceA := ADR(instanceC);
    refInstanceA^.foo();
END_FUNCTION