// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// -- FbA
TYPE VtableFbA:
    STRUCT
        methodDefinedInA_One: POINTER TO FbA.methodDefinedInA_One := ADR(FbA.methodDefinedInA_One);
        methodDefinedInA_Two: POINTER TO FbA.methodDefinedInA_Two := ADR(FbA.methodDefinedInA_Two);
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FbA
    VAR
        vtable: POINTER TO VtableFbA;
        oneA: DINT := 1;
        twoA: DINT := 2;
    END_VAR

    METHOD methodDefinedInA_One: DINT
        printf('Hello from FbA::methodDefinedInA_One (%d, %d)$N', oneA, twoA);
    END_METHOD

    METHOD methodDefinedInA_Two: DINT
        printf('Hello from FbA::methodDefinedInA_Two (%d, %d)$N', oneA, twoA);
    END_METHOD
END_FUNCTION_BLOCK

// -- FbB (overrides methodDefinedInA_Two)
TYPE VtableFbB:
    STRUCT
        vtableA: VtableFbA := (
            methodDefinedInA_One := ADR(FbA.methodDefinedInA_One),
            methodDefinedInA_Two := ADR(FbB.methodDefinedInA_Two) // Overridden
        );
        methodDefinedInB_One: POINTER TO FbB.methodDefinedInB_One := ADR(FbB.methodDefinedInB_One);
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FbB EXTENDS FbA
    VAR
        threeB: DINT := 3;
        fourB: DINT := 4;
    END_VAR

    // Overridden
    METHOD methodDefinedInA_Two: DINT
        printf('Hello from FbB::methodDefinedInA_Two (%d, %d, %d, %d) (OVERRIDDEN)$N', oneA, twoA, threeB, fourB);
    END_METHOD

    METHOD methodDefinedInB_One: DINT
        printf('Hello from FbB::methodDefinedInB_One (%d, %d)$N', threeB, fourB);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    instanceVTableFbA: VtableFbA;
    instanceVTableFbB: VtableFbB;
END_VAR

FUNCTION main
    VAR
        instanceFbA: FbA;
        instanceFbB: FbB;

        refInstance: POINTER TO FbA;
    END_VAR

    instanceFbA.vtable := ADR(instanceVTableFbA);
    instanceFbB.vtable := ADR(instanceVTableFbB);

    refInstance := ADR(instanceFbA);
    refInstance^.vtable^.methodDefinedInA_One^(refInstance^); // CHECK: Hello from FbA::methodDefinedInA_One (1, 2)
    refInstance^.vtable^.methodDefinedInA_Two^(refInstance^); // CHECK: Hello from FbA::methodDefinedInA_Two (1, 2)

    refInstance := ADR(instanceFbB);
    refInstance^.vtable^.methodDefinedInA_One^(refInstance^); // CHECK: Hello from FbA::methodDefinedInA_One (1, 2)
    refInstance^.vtable^.methodDefinedInA_Two^(refInstance^); // CHECK: Hello from FbB::methodDefinedInA_Two (1, 2, 3, 4) (OVERRIDDEN)
END_FUNCTION