// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK FbA
    VAR
        localState: DINT := 5;
    END_VAR

    METHOD increaseStateByOne
        localState := localState + 1;
        printf('FbA::increaseStateByOne: localState = %d$N', localState);
    END_METHOD

    METHOD increaseStateByArg
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
        fnPtrIncreaseStateByOne: POINTER TO FbA.increaseStateByOne := ADR(FbA.increaseStateByOne);
        fnPtrIncreaseStateByArg: POINTER TO FbA.increaseStateByArg := ADR(FbA.increaseStateByArg);
    END_VAR

    // CHECK: FbA::increaseStateByOne: localState = 6
    // CHECK: FbA::increaseStateByArg: localState = 10
    fnPtrIncreaseStateByOne^(instanceFbA);
    fnPtrIncreaseStateByArg^(instanceFbA, 4);
END_FUNCTION