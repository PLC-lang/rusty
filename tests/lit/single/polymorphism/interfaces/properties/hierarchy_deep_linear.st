// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Deep linear interface chain A <- B <- C <- D. Each interface declares one property.
// Two function blocks implement D: FbD uses its own getters, FbDOverride overrides some of them.
// Verifies that accessing all inherited properties (overridden and non-overridden) through the
// leaf interface dispatches correctly.

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

INTERFACE C EXTENDS B
    PROPERTY valC : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

INTERFACE DD EXTENDS C
    PROPERTY valD : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

// Implements all four properties directly
FUNCTION_BLOCK FbD IMPLEMENTS DD
    VAR_INPUT
        _a : DINT := 10;
        _b : DINT := 20;
        _c : DINT := 30;
        _d : DINT := 40;
    END_VAR

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

// Extends FbD and overrides valA (root) and valC (middle) getters,
// leaves valB and valD non-overridden.
FUNCTION_BLOCK FbDOverride EXTENDS FbD
    PROPERTY valA : DINT
        GET valA := _a + 1000; END_GET
    END_PROPERTY

    PROPERTY valC : DINT
        GET valC := _c + 3000; END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        inst : FbD;
        instOvr : FbDOverride;

        refA : A;
        refB : B;
        refC : C;
        refD : DD;
    END_VAR

    // === FbD (no overrides) through every interface level ===
    refD := inst;
    printf('%d$N', refD.valA);          // CHECK: 10
    printf('%d$N', refD.valB);          // CHECK-NEXT: 20
    printf('%d$N', refD.valC);          // CHECK-NEXT: 30
    printf('%d$N', refD.valD);          // CHECK-NEXT: 40

    refC := inst;
    printf('%d$N', refC.valA);          // CHECK-NEXT: 10
    printf('%d$N', refC.valB);          // CHECK-NEXT: 20
    printf('%d$N', refC.valC);          // CHECK-NEXT: 30

    refB := inst;
    printf('%d$N', refB.valA);          // CHECK-NEXT: 10
    printf('%d$N', refB.valB);          // CHECK-NEXT: 20

    refA := inst;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 10

    // === FbDOverride (valA and valC overridden) through leaf interface ===
    refD := instOvr;
    printf('%d$N', refD.valA);          // CHECK-NEXT: 1010
    printf('%d$N', refD.valB);          // CHECK-NEXT: 20
    printf('%d$N', refD.valC);          // CHECK-NEXT: 3030
    printf('%d$N', refD.valD);          // CHECK-NEXT: 40

    // === Overrides visible through intermediate interfaces too ===
    refA := instOvr;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 1010

    refB := instOvr;
    printf('%d$N', refB.valA);          // CHECK-NEXT: 1010
    printf('%d$N', refB.valB);          // CHECK-NEXT: 20

    refC := instOvr;
    printf('%d$N', refC.valA);          // CHECK-NEXT: 1010
    printf('%d$N', refC.valC);          // CHECK-NEXT: 3030

    // === Setters through leaf, verify through root ===
    refD := instOvr;
    refD.valA := 99;
    refA := instOvr;
    printf('%d$N', refA.valA);          // CHECK-NEXT: 1099

    refD.valB := 77;
    refB := instOvr;
    printf('%d$N', refB.valB);          // CHECK-NEXT: 77
END_FUNCTION
