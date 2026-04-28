// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Classic diamond pattern inheritance hierachy between interfaces. Should assert that even with
// multi inheritance dynamic dispatch works correctly.
// A
// / \
// B C
// \ /
// DD

INTERFACE A
    METHOD foo
    END_METHOD
END_INTERFACE

INTERFACE B EXTENDS A
    METHOD bar
    END_METHOD
END_INTERFACE

INTERFACE C EXTENDS A
    METHOD baz
    END_METHOD
END_INTERFACE

INTERFACE DD EXTENDS B, C
    METHOD qux
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

FUNCTION main
    VAR
        refA: A;
        refB: B;
        refC: C;
        refD: DD;

        instanceA: FbA;
        instanceB: FbB;
        instanceC: FbC;
        instanceD: FbD;
    END_VAR

    // ~~ refA := instance{A,B,C,DD} ~~
    refA := instanceA;
    refA.foo(); // CHECK: FbA::foo

    refA := instanceB;
    refA.foo(); // CHECK-NEXT: FbB::foo

    refA := instanceC;
    refA.foo(); // CHECK-NEXT: FbC::foo

    refA := instanceD;
    refA.foo(); // CHECK-NEXT: FbD::foo

    // ~~ refB := instance{B,DD} ~~
    refB := instanceB;
    refB.foo(); // CHECK-NEXT: FbB::foo
    refB.bar(); // CHECK-NEXT: FbB::bar

    refB := instanceD;
    refB.foo(); // CHECK-NEXT: FbD::foo
    refB.bar(); // CHECK-NEXT: FbD::bar

    // ~~ refC := instance{C,DD} ~~
    refC := instanceC;
    refC.foo(); // CHECK-NEXT: FbC::foo
    refC.baz(); // CHECK-NEXT: FbC::baz

    refC := instanceD;
    refC.foo(); // CHECK-NEXT: FbD::foo
    refC.baz(); // CHECK-NEXT: FbD::baz

    // ~~ refD := instance{DD} ~~
    refD := instanceD;
    refD.foo(); // CHECK-NEXT: FbD::foo
    refD.bar(); // CHECK-NEXT: FbD::bar
    refD.baz(); // CHECK-NEXT: FbD::baz
    refD.qux(); // CHECK-NEXT: FbD::qux
END_FUNCTION