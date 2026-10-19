// RUN: %COMPILE %s && %RUN | %CHECK %s

FUNCTION_BLOCK A
    VAR_INPUT
        in: DINT;
    END_VAR

    METHOD alpha: DINT
        alpha := 1;
    END_METHOD

    METHOD compute: DINT
        compute := 2;
    END_METHOD

    printf('A body, in = %d$N', in);
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD compute: DINT
        compute := 3;
    END_METHOD

    printf('B body, in = %d$N', in);
END_FUNCTION_BLOCK

// `compute` of C is in a different vtable slot than `compute` of A
FUNCTION_BLOCK C
    VAR_INPUT
        refToA: REF_TO A;
        pointerToA: POINTER TO A;
        referenceToA: REFERENCE TO A;
    END_VAR

    METHOD compute: DINT
        compute := 99;
    END_METHOD

    METHOD printFromMethod
        printf('method: ref_to = %d, pointer_to = %d, reference_to = %d$N', refToA^.compute(), pointerToA^.compute(), referenceToA.compute());
    END_METHOD

    printf('body: ref_to = %d, pointer_to = %d, reference_to = %d$N', refToA^.compute(), pointerToA^.compute(), referenceToA.compute());
    referenceToA(in := 7);
END_FUNCTION_BLOCK

CLASS Reader
    VAR
        refToA: REF_TO A;
    END_VAR

    METHOD printFromClass
        VAR_INPUT
            target: REF_TO A;
        END_VAR

        refToA := target;
        printf('class: ref_to = %d$N', refToA^.compute());
    END_METHOD
END_CLASS

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;
        instanceReader: Reader;
    END_VAR

    // CHECK: body: ref_to = 2, pointer_to = 2, reference_to = 2
    // CHECK: A body, in = 7
    // CHECK: method: ref_to = 2, pointer_to = 2, reference_to = 2
    instanceC.referenceToA REF= instanceA;
    instanceC(refToA := REF(instanceA), pointerToA := ADR(instanceA));
    instanceC.printFromMethod();

    // CHECK: body: ref_to = 3, pointer_to = 3, reference_to = 3
    // CHECK: B body, in = 7
    // CHECK: method: ref_to = 3, pointer_to = 3, reference_to = 3
    instanceC.referenceToA REF= instanceB;
    instanceC(refToA := REF(instanceB), pointerToA := ADR(instanceB));
    instanceC.printFromMethod();

    // CHECK: class: ref_to = 2
    // CHECK: class: ref_to = 3
    instanceReader.printFromClass(REF(instanceA));
    instanceReader.printFromClass(REF(instanceB));
END_FUNCTION
