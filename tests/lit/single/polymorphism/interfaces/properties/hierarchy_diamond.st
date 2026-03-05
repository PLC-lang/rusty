// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Classic diamond pattern inheritance hierarchy between interfaces, using properties
// instead of methods. Should assert that even with multi-inheritance, property dispatch through
// inherited interfaces works correctly.
//    A
//   / \
//  B   C
//   \ /
//    DD

INTERFACE A
    PROPERTY valA : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE B EXTENDS A
    PROPERTY valB : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE C EXTENDS A
    PROPERTY valC : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE DD EXTENDS B, C
    PROPERTY valD : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    VAR _a : DINT := 1; END_VAR
    PROPERTY valA : DINT
        GET valA := _a; END_GET
        SET _a := valA; END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS B
    VAR _a : DINT := 2; _b : DINT := 20; END_VAR
    PROPERTY valA : DINT
        GET valA := _a; END_GET
        SET _a := valA; END_SET
    END_PROPERTY
    PROPERTY valB : DINT
        GET valB := _b; END_GET
        SET _b := valB; END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC IMPLEMENTS C
    VAR _a : DINT := 3; _c : DINT := 30; END_VAR
    PROPERTY valA : DINT
        GET valA := _a; END_GET
        SET _a := valA; END_SET
    END_PROPERTY
    PROPERTY valC : DINT
        GET valC := _c; END_GET
        SET _c := valC; END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbD IMPLEMENTS DD
    VAR _a : DINT := 4; _b : DINT := 40; _c : DINT := 400; _d : DINT := 4000; END_VAR
    PROPERTY valA : DINT
        GET valA := _a; END_GET
        SET _a := valA; END_SET
    END_PROPERTY
    PROPERTY valB : DINT
        GET valB := _b; END_GET
        SET _b := valB; END_SET
    END_PROPERTY
    PROPERTY valC : DINT
        GET valC := _c; END_GET
        SET _c := valC; END_SET
    END_PROPERTY
    PROPERTY valD : DINT
        GET valD := _d; END_GET
        SET _d := valD; END_SET
    END_PROPERTY
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

    // ~~ refA := instance{A,B,C,DD} — valA accessible through root interface ~~
    refA := instanceA;
    printf('%d$N', refA.valA);          // CHECK: 1

    refA := instanceB;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 2

    refA := instanceC;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 3

    refA := instanceD;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 4

    // ~~ refB := instance{B,DD} — valA inherited from A, valB declared on B ~~
    refB := instanceB;
    printf('%d$N', refB.valA);          // CHECK-NEXT: 2
    printf('%d$N', refB.valB);          // CHECK-NEXT: 20

    refB := instanceD;
    printf('%d$N', refB.valA);          // CHECK-NEXT: 4
    printf('%d$N', refB.valB);          // CHECK-NEXT: 40

    // ~~ refC := instance{C,DD} — valA inherited from A, valC declared on C ~~
    refC := instanceC;
    printf('%d$N', refC.valA);          // CHECK-NEXT: 3
    printf('%d$N', refC.valC);          // CHECK-NEXT: 30

    refC := instanceD;
    printf('%d$N', refC.valA);          // CHECK-NEXT: 4
    printf('%d$N', refC.valC);          // CHECK-NEXT: 400

    // ~~ refD := instanceDD — all properties accessible through the leaf interface ~~
    refD := instanceD;
    printf('%d$N', refD.valA);          // CHECK-NEXT: 4
    printf('%d$N', refD.valB);          // CHECK-NEXT: 40
    printf('%d$N', refD.valC);          // CHECK-NEXT: 400
    printf('%d$N', refD.valD);          // CHECK-NEXT: 4000

    // ~~ setter through leaf interface, verify via root ~~
    refD.valA := 999;
    refA := instanceD;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 999
END_FUNCTION
