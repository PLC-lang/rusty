// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Diamond inheritance pattern:
//   A
//  / \
// B   C
//  \ /
//   DD
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
    refA.foo(); // CHECK: FbB::foo

    refA := instanceC;
    refA.foo(); // CHECK: FbC::foo

    refA := instanceD;
    refA.foo(); // CHECK: FbD::foo

    // ~~ refB := instance{B,DD} ~~
    refB := instanceB;
    refB.foo(); // CHECK: FbB::foo
    refB.bar(); // CHECK: FbB::bar

    refB := instanceD;
    refB.foo(); // CHECK: FbD::foo
    refB.bar(); // CHECK: FbD::bar

    // ~~ refC := instance{C,DD} ~~
    refC := instanceC;
    refC.foo(); // CHECK: FbC::foo
    refC.baz(); // CHECK: FbC::baz

    refC := instanceD;
    refC.foo(); // CHECK: FbD::foo
    refC.baz(); // CHECK: FbD::baz

    // ~~ refD := instance{DD} ~~
    refD := instanceD;
    refD.foo(); // CHECK: FbD::foo
    refD.bar(); // CHECK: FbD::bar
    refD.baz(); // CHECK: FbD::baz
    refD.qux(); // CHECK: FbD::qux
END_FUNCTION