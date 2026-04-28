// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: FbA is the only POU which directly implements interface A. FbB and FbC indirectly implement the
// interface by extending from FbA. Implicitly, this means FbB and FbC can be assigned to variables of type
// interface A.

INTERFACE A
    METHOD id
    END_METHOD
END_INTERFACE

// Only FbA implements A directly
FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD uniqueA1
    END_METHOD

    METHOD id
        printf('FbA$N');
    END_METHOD
END_FUNCTION_BLOCK

// FbB implicitly implements A via FbA (overridden)
FUNCTION_BLOCK FbB EXTENDS FbA
    METHOD id
        printf('FbB$N');
    END_METHOD

    METHOD uniqueB2
    END_METHOD
END_FUNCTION_BLOCK

// FbC implicitly implements A via FbB (overridden)
FUNCTION_BLOCK FbC EXTENDS FbB
    METHOD unqiueC1
    END_METHOD

    METHOD id
        printf('FbC$N');
    END_METHOD

    METHOD unqiueC2
    END_METHOD
END_FUNCTION_BLOCK

// FbD implicitly implements A via FbC (inherited)
FUNCTION_BLOCK FbD EXTENDS FbC
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        reference: A;

        instanceA: FbA;
        instanceB: FbB;
        instanceC: FbC;
        instanceD: FbD;
    END_VAR

    // CHECK: FbA
    reference := instanceA;
    reference.id();

    // CHECK-NEXT: FbB
    reference := instanceB;
    reference.id();

    // CHECK-NEXT: FbC
    reference := instanceC;
    reference.id();

    // CHECK-NEXT: FbC
    reference := instanceD;
    reference.id();
END_FUNCTION