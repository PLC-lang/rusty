// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD inputArguments
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR

        printf('A::inputArguments: in1 = %d, in2 = %d$N', in1, in2);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        localOut1: DINT;
        localOut2: DINT;

        fnPtrInputArguments: __FPOINTER A.inputArguments := ADR(A.inputArguments);
    END_VAR

    // Implicit
    // CHECK: A::inputArguments: in1 = 5, in2 = 10
    // CHECK: A::inputArguments: in1 = 5, in2 = 10
    fnPtrInputArguments^(instanceA, 5, 10);
    fnPtrInputArguments^(instanceA, in1 := 5, in2 := 10);
END_FUNCTION