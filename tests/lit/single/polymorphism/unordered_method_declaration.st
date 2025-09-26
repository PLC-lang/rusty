// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// This test ensures that even with an unordered method declaration, the methods internally are still defined
// in a fixed order within the virtual table. Specifically, if function block A defines methods in order
// [alpha, bravo, charlie] and function block B extends A overridding bravo and introducing delta in order
// [delta, bravo] then internally the virtual table must be defined as [alpha, bravo, charlie, delta] rather
// than [delta, bravo, alpha, charlie]. Not doing so would break the ability to upcast from B to A because
// B's virtual table would not match the expected layout of its super class A = [alpha, bravo, charlie].

FUNCTION_BLOCK A
    METHOD alpha
        printf('A::alpha$N');
    END_METHOD

    METHOD bravo
        printf('A::bravo$N');
    END_METHOD

    METHOD charlie
        printf('A::charlie$N');
    END_METHOD

    METHOD delta
        printf('A::delta$N');
    END_METHOD

    printf('A::body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD echo
        printf('B::echo$N');
    END_METHOD

    // Overridden
    METHOD charlie
        printf('B::charlie$N');
    END_METHOD

    // Overridden
    METHOD alpha
        printf('B::alpha$N');
    END_METHOD

    METHOD foxtrot
        printf('B::foxtrot$N');
    END_METHOD

    printf('B::body$N');
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    METHOD golf
        printf('C::golf$N');
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

    // CHECK: A::body
    // CHECK: A::alpha
    // CHECK: A::bravo
    // CHECK: A::charlie
    // CHECK: A::delta
    refInstanceA := ADR(instanceA);
    refInstanceA^();
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();

    // CHECK: B::body
    // CHECK: B::alpha
    // CHECK: A::bravo
    // CHECK: B::charlie
    // CHECK: A::delta
    refInstanceA := ADR(instanceB);
    refInstanceA^();
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();

    // CHECK: C::body
    // CHECK: B::alpha
    // CHECK: A::bravo
    // CHECK: B::charlie
    // CHECK: A::delta
    refInstanceA := ADR(instanceC);
    refInstanceA^();
    refInstanceA^.alpha();
    refInstanceA^.bravo();
    refInstanceA^.charlie();
    refInstanceA^.delta();
END_FUNCTION