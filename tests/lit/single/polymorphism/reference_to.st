// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD alpha
        printf('A::alpha$N');
    END_METHOD

    METHOD bravo
        printf('A::bravo$N');
    END_METHOD

    printf('A::body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD alpha
        printf('B::alpha$N');
    END_METHOD

    printf('B::body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    METHOD alpha
        printf('C::alpha$N');
    END_METHOD

    METHOD bravo
        printf('C::bravo$N');
    END_METHOD

    printf('C::body$N');
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;

        refInstanceA: REFERENCE TO A;
    END_VAR

    // CHECK: A::body
    // CHECK: A::alpha
    // CHECK: A::bravo
    refInstanceA REF= instanceA;
    refInstanceA();
    refInstanceA.alpha();
    refInstanceA.bravo();

    // CHECK: B::body
    // CHECK: B::alpha
    // CHECK: A::bravo
    refInstanceA REF= instanceB;
    refInstanceA();
    refInstanceA.alpha();
    refInstanceA.bravo();

    // CHECK: C::body
    // CHECK: C::alpha
    // CHECK: C::bravo
    refInstanceA REF= instanceC;
    refInstanceA();
    refInstanceA.alpha();
    refInstanceA.bravo();
END_FUNCTION