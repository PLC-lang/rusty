// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    VAR
        localStateA: DINT := 1;
    END_VAR

    printf('A::body: localStateA = %d$N', localStateA);
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    VAR
        localStateB: DINT := 2;
    END_VAR

    SUPER^();
    printf('B::body: localStateB = %d$N', localStateB);
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    VAR
        localStateC: DINT := 3;
    END_VAR

    SUPER^();
    printf('C::body: localStateC = %d$N', localStateC);
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;

        refInstanceA: POINTER TO A;
        refInstanceB: POINTER TO B;
        refInstanceC: POINTER TO C;
    END_VAR

    instanceA.localStateA := 5;
    instanceB.localStateB := 10;
    instanceC.localStateC := 15;

    // CHECK: A::body: localStateA = 5
    refInstanceA := ADR(instanceA);
    refInstanceA^();

    // CHECK: A::body: localStateA = 1
    // CHECK: B::body: localStateB = 10
    refInstanceA := ADR(instanceB);
    refInstanceA^();

    // CHECK: A::body: localStateA = 1
    // CHECK: B::body: localStateB = 2
    // CHECK: C::body: localStateC = 15
    refInstanceA := ADR(instanceC);
    refInstanceA^();


    // CHECK: A::body: localStateA = 1
    // CHECK: B::body: localStateB = 10
    refInstanceB := ADR(instanceB);
    refInstanceB^();

    // CHECK: A::body: localStateA = 1
    // CHECK: B::body: localStateB = 2
    // CHECK: C::body: localStateC = 15
    refInstanceB := ADR(instanceC);
    refInstanceB^();

    // CHECK: A::body: localStateA = 1
    // CHECK: B::body: localStateB = 2
    // CHECK: C::body: localStateC = 15
    refInstanceC := ADR(instanceC);
    refInstanceC^();
END_FUNCTION