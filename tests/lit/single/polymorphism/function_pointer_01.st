// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE VtableFbA:
    STRUCT
        methodeDefinedInA: POINTER TO FbA.methodeDefinedInA := ADR(FbA.methodeDefinedInA);
    END_STRUCT
END_TYPE

VAR_GLOBAL
    instanceVtableFbA: VtableFbA;
END_VAR

FUNCTION_BLOCK FbA
    VAR
        vtable: POINTER TO VtableFbA := ADR(instanceVtableFbA);
    END_VAR

    METHOD methodeDefinedInA: DINT
        printf('Hello from FbA::methodeDefinedInA$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
    END_VAR

    instanceFbA.vtable^.methodeDefinedInA^(instanceFbA); // CHECK: Hello from FbA::methodeDefinedInA
END_FUNCTION