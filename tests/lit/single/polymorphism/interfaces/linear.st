// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: (Big) linear / single inheritance hierachy between interfaces, A <- B <- C <- DD <- E <- F

INTERFACE A
    METHOD foo
    END_METHOD
END_INTERFACE

INTERFACE B EXTENDS A
    METHOD bar
    END_METHOD
END_INTERFACE

INTERFACE C EXTENDS B
    METHOD baz
    END_METHOD
END_INTERFACE

INTERFACE DD EXTENDS C
    METHOD qux
    END_METHOD
END_INTERFACE

INTERFACE E EXTENDS DD
    METHOD quux
    END_METHOD
END_INTERFACE

INTERFACE F EXTENDS E
    METHOD corge
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS B
    METHOD foo
        printf('FbB::foo$N');
    END_METHOD

    METHOD bar
        printf('FbB::bar$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC IMPLEMENTS C
    METHOD foo
        printf('FbC::foo$N');
    END_METHOD

    METHOD bar
        printf('FbC::bar$N');
    END_METHOD

    METHOD baz
        printf('FbC::baz$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbD IMPLEMENTS DD
    METHOD foo
        printf('FbD::foo$N');
    END_METHOD

    METHOD bar
        printf('FbD::bar$N');
    END_METHOD

    METHOD baz
        printf('FbD::baz$N');
    END_METHOD

    METHOD qux
        printf('FbD::qux$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbE IMPLEMENTS E
    METHOD foo
        printf('FbE::foo$N');
    END_METHOD

    METHOD bar
        printf('FbE::bar$N');
    END_METHOD

    METHOD baz
        printf('FbE::baz$N');
    END_METHOD

    METHOD qux
        printf('FbE::qux$N');
    END_METHOD

    METHOD quux
        printf('FbE::quux$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbF IMPLEMENTS F
    METHOD foo
        printf('FbF::foo$N');
    END_METHOD

    METHOD bar
        printf('FbF::bar$N');
    END_METHOD

    METHOD baz
        printf('FbF::baz$N');
    END_METHOD

    METHOD qux
        printf('FbF::qux$N');
    END_METHOD

    METHOD quux
        printf('FbF::quux$N');
    END_METHOD

    METHOD corge
        printf('FbF::corge$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        refA: A;
        refB: B;
        refC: C;
        refD: DD;
        refE: E;
        refF: F;

        instanceA: FbA;
        instanceB: FbB;
        instanceC: FbC;
        instanceD: FbD;
        instanceE: FbE;
        instanceF: FbF;
    END_VAR

    // ~~ refA := instance{A,B,C,DD,E,F} ~~
    // CHECK: FbA::foo
    // CHECK-NEXT: FbB::foo
    // CHECK-NEXT: FbC::foo
    // CHECK-NEXT: FbD::foo
    // CHECK-NEXT: FbE::foo
    // CHECK-NEXT: FbF::foo
    refA := instanceA;
    refA.foo();

    refA := instanceB;
    refA.foo();

    refA := instanceC;
    refA.foo();

    refA := instanceD;
    refA.foo();

    refA := instanceE;
    refA.foo();

    refA := instanceF;
    refA.foo();

    // ~~ refB := instance{B,C,DD,E,F} ~~
    // CHECK-NEXT: FbB::bar
    // CHECK-NEXT: FbC::bar
    // CHECK-NEXT: FbD::bar
    // CHECK-NEXT: FbE::bar
    // CHECK-NEXT: FbF::bar
    refB := instanceB;
    refB.bar();

    refB := instanceC;
    refB.bar();

    refB := instanceD;
    refB.bar();

    refB := instanceE;
    refB.bar();

    refB := instanceF;
    refB.bar();

    // ~~ refC := instance{C,DD,E,F} ~~
    // CHECK-NEXT: FbC::baz
    // CHECK-NEXT: FbD::baz
    // CHECK-NEXT: FbE::baz
    // CHECK-NEXT: FbF::baz
    refC := instanceC;
    refC.baz();

    refC := instanceD;
    refC.baz();

    refC := instanceE;
    refC.baz();

    refC := instanceF;
    refC.baz();

    // ~~ refD := instance{DD,E,F} ~~
    // CHECK-NEXT: FbD::qux
    // CHECK-NEXT: FbE::qux
    // CHECK-NEXT: FbF::qux
    refD := instanceD;
    refD.qux();

    refD := instanceE;
    refD.qux();

    refD := instanceF;
    refD.qux();

    // ~~ refE := instance{E,F} ~~
    // CHECK-NEXT: FbE::quux
    // CHECK-NEXT: FbF::quux
    refE := instanceE;
    refE.quux();

    refE := instanceF;
    refE.quux();

    // ~~ refF := instance{F} ~~
    // CHECK-NEXT: FbF::corge
    refF := instanceF;
    refF.corge();
END_FUNCTION