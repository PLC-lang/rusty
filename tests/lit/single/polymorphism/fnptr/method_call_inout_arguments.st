// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD inoutArguments
        VAR_IN_OUT
            inout1: DINT;
            inout2: DINT;
        END_VAR

        inout1 := 100;
        inout2 := 200;
        printf('A::inoutArguments: inout1 = %d, inout2 = %d$N', inout1, inout2);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        localInOut1: DINT;
        localInOut2: DINT;

        fnPtrInOutArguments: FNPTR A.inoutArguments := ADR(A.inoutArguments);
    END_VAR

    // Explicit
    // CHECK: A::inoutArguments: inout1 = 100, inout2 = 200
    // CHECK: A::inoutArguments: inout1 = 100, inout2 = 200
    fnPtrInOutArguments^(instanceA, localInOut1, localInOut2);
    fnPtrInOutArguments^(instanceA, inout1 := localInOut1, inout2 := localInOut2);
END_FUNCTION

