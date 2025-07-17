// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE UserDefinedVT_FbA: STRUCT
        increaseStateByOne: POINTER TO FbA.increaseStateByOne := ADR(FbA.increaseStateByOne);
        increaseStateByArg: POINTER TO FbA.increaseStateByArg := ADR(FbA.increaseStateByArg);
    END_STRUCT
END_TYPE

VAR_GLOBAL
    UserDefinedVT_FbA_instance: UserDefinedVT_FbA;
END_VAR

FUNCTION_BLOCK FbA
    VAR
        vt: POINTER TO __VOID := ADR(UserDefinedVT_FbA_instance);
        localState: DINT := 5;
    END_VAR

    METHOD increaseStateByOne: DINT
        localState := localState + 1;
        printf('FbA::increaseStateByOne: localState = %d$N', localState);
    END_METHOD

    METHOD increaseStateByArg: DINT
        VAR_INPUT
            in: DINT;
        END_VAR

        localState := localState + in;
        printf('FbA::increaseStateByArg: localState = %d$N', localState);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
        refInstanceFbA: POINTER TO FbA := ADR(instanceFbA);
    END_VAR

    UserDefinedVT_FbA#(refInstanceFbA^.vt^).increaseStateByOne^(refInstanceFbA^); // CHECK: FbA::increaseStateByOne: localState = 6
    UserDefinedVT_FbA#(refInstanceFbA^.vt^).increaseStateByArg^(refInstanceFbA^, 4); // CHECK: FbA::increaseStateByArg: localState = 10
END_FUNCTION