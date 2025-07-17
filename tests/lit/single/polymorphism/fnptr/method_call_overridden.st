// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE UserDefinedVT_A: STRUCT
        increaseStateByOne: POINTER TO A.increaseStateByOne := ADR(A.increaseStateByOne);
        increaseStateByArg: POINTER TO A.increaseStateByArg := ADR(A.increaseStateByArg);
    END_STRUCT
END_TYPE

TYPE UserDefinedVT_B: STRUCT
        increaseStateByOne: POINTER TO A.increaseStateByOne := ADR(B.increaseStateByOne); // Overridden method
        increaseStateByArg: POINTER TO A.increaseStateByArg := ADR(A.increaseStateByArg);
        decreaseStateByOne: POINTER TO B.decreaseStateByOne := ADR(B.decreaseStateByOne);
    END_STRUCT
END_TYPE

VAR_GLOBAL
    UserDefinedVT_A_instance: UserDefinedVT_A;
    UserDefinedVT_B_instance: UserDefinedVT_B;
END_VAR

FUNCTION_BLOCK A
    VAR
        vt: POINTER TO __VOID := ADR(UserDefinedVT_A_instance);
        localState: DINT := 5;
    END_VAR

    METHOD increaseStateByOne: DINT
        localState := localState + 1;
        printf('A::increaseStateByOne: localState = %d$N', localState);
    END_METHOD

    METHOD increaseStateByArg: DINT
        VAR_INPUT
            in: DINT;
        END_VAR

        localState := localState + in;
        printf('A::increaseStateByArg: localState = %d$N', localState);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD increaseStateByOne: DINT
        localState := localState + 1;
        printf('B::increaseStateByOne: localState = %d (overridden)$N', localState);
    END_METHOD

    METHOD decreaseStateByOne: DINT
        localState := localState - 1;
        printf('B::decreaseStateByOne: localState = %d$N', localState);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        refInstanceA: POINTER TO A;
        refInstanceB: POINTER TO B;
    END_VAR

    // We need to initialize the virtual table of B here, because there's no other direct way to do before invoking methods from it (no constructors, no __init implementation)
    // Essentially we are overriding the variable defined in the super (A) which should now point to the correct virtual table
    instanceB.vt := ADR(UserDefinedVT_B_instance);

    // CHECK: A::increaseStateByOne: localState = 6
    // CHECK: A::increaseStateByArg: localState = 10
    refInstanceA := ADR(instanceA);
    UserDefinedVT_A#(refInstanceA^.vt^).increaseStateByOne^(refInstanceA^);
    UserDefinedVT_A#(refInstanceA^.vt^).increaseStateByArg^(refInstanceA^, 4);


    // CHECK: B::increaseStateByOne: localState = 6 (overridden)
    // CHECK: A::increaseStateByArg: localState = 10
    refInstanceA := ADR(instanceB);
    UserDefinedVT_A#(refInstanceA^.vt^).increaseStateByOne^(refInstanceA^);
    UserDefinedVT_A#(refInstanceA^.vt^).increaseStateByArg^(refInstanceA^, 4);

    // CHECK: B::decreaseStateByOne: localState = 9
    refInstanceB := ADR(instanceB);
    UserDefinedVT_B#(refInstanceB^.vt^).decreaseStateByOne^(refInstanceB^);
END_FUNCTION

