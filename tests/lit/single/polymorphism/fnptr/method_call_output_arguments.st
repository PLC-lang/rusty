// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD outputArguments
        VAR_OUTPUT
            out1: DINT;
            out2: DINT;
        END_VAR

        out1 := 100;
        out2 := 200;
        printf('A::outputArguments: out1 = %d, out2 = %d$N', out1, out2);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        localOut1: DINT;
        localOut2: DINT;

        fnPtrOutputArguments: __FPOINTER A.outputArguments := ADR(A.outputArguments);
    END_VAR

    // Explicit
    // CHECK: A::outputArguments: out1 = 100, out2 = 200
    // CHECK: A::outputArguments: out1 = 100, out2 = 200
    fnPtrOutputArguments^(instanceA, localOut1, localOut2);
    fnPtrOutputArguments^(instanceA, out1 => localOut1, out2 => localOut2);
END_FUNCTION

