// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Virtual Table Definitions
TYPE UserVT_FbA:
    STRUCT
        printNumber: POINTER TO FbA.printNumber := ADR(FbA.printNumber);
    END_STRUCT
END_TYPE

TYPE UserVT_FbB:
    STRUCT
        // No override, hence the ADR(FbA.<...>)
        printNumber: POINTER TO FbA.printNumber := ADR(FbA.printNumber);
    END_STRUCT
END_TYPE

TYPE UserVT_FbC:
    STRUCT
        // override, hence the ADR(FbC.<...>)
        printNumber: POINTER TO FbA.printNumber := ADR(FbC.printNumber);
    END_STRUCT
END_TYPE

// Virtual Table Instances
VAR_GLOBAL
    UserVT_FbA_instance: UserVT_FbA;
    UserVT_FbB_instance: UserVT_FbB;
    UserVT_FbC_instance: UserVT_FbC;
END_VAR

FUNCTION_BLOCK FbA
    VAR
        vt: POINTER TO __VOID;
        localStateA: DINT := 5;
    END_VAR

    METHOD printNumber
        VAR_INPUT
            in: DINT;
        END_VAR

        printf('FbA::printNumber: localStateA = %d, in = %d$N', localStateA, in);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB EXTENDS FbA
    VAR
        localStateB: DINT := 10;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC EXTENDS FbA
    VAR
        localStateC: DINT := 15;
    END_VAR

    METHOD printNumber
        VAR_INPUT
            in: DINT;
        END_VAR

        printf('FbC::printNumber: localStateA = %d, localStateC = %d, in = %d$N', localStateA, localStateC, in);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
        instanceFbB: FbB;
        instanceFbC: FbC;
        refInstanceFbA: POINTER TO FbA;
    END_VAR

    // Virtual Table Initialization
    instanceFbA.vt := ADR(UserVT_FbA_instance);
    instanceFbB.vt := ADR(UserVT_FbB_instance);
    instanceFbC.vt := ADR(UserVT_FbC_instance);

    // CHECK: FbA::printNumber: localStateA = 5, in = 5
    refInstanceFbA := ADR(instanceFbA);
    UserVT_FbA#(refInstanceFbA^.vt^).printNumber^(refInstanceFbA^, 5);
    
    // CHECK: FbA::printNumber: localStateA = 5, in = 10
    refInstanceFbA := ADR(instanceFbB);
    UserVT_FbA#(refInstanceFbA^.vt^).printNumber^(refInstanceFbA^, 10);

    // CHECK: FbC::printNumber: localStateA = 5, localStateC = 15, in = 15
    refInstanceFbA := ADR(instanceFbC);
    UserVT_FbA#(refInstanceFbA^.vt^).printNumber^(refInstanceFbA^, 15);
END_FUNCTION