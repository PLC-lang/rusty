// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE UserDefinedVT_A: STRUCT
        inputArguments: POINTER TO A.inputArguments := ADR(A.inputArguments);
        outputArguments: POINTER TO A.outputArguments := ADR(A.outputArguments);
    END_STRUCT
END_TYPE

VAR_GLOBAL
    UserDefinedVT_A_instance: UserDefinedVT_A;
END_VAR

FUNCTION_BLOCK A
    VAR
        vt: POINTER TO __VOID := ADR(UserDefinedVT_A_instance);
    END_VAR

    METHOD inputArguments
        VAR_INPUT
            in1: DINT;
            in2: DINT;
        END_VAR

        printf('A::inputArguments: in1 = %d, in2 = %d$N', in1, in2);
    END_METHOD

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
        refInstanceA: POINTER TO A := ADR(instanceA);
        localOut1: DINT;
        localOut2: DINT;
    END_VAR

    // Implicit
    // CHECK: A::inputArguments: in1 = 5, in2 = 10
    // CHECK: A::inputArguments: in1 = 5, in2 = 10
    UserDefinedVT_A#(refInstanceA^.vt^).inputArguments^(refInstanceA^, 5, 10);
    UserDefinedVT_A#(refInstanceA^.vt^).inputArguments^(refInstanceA^, in1 := 5, in2 := 10);

    // Explicit
    // CHECK: A::outputArguments: out1 = 100, out2 = 200
    // CHECK: A::outputArguments: out1 = 100, out2 = 200
    UserDefinedVT_A#(refInstanceA^.vt^).outputArguments^(refInstanceA^, localOut1, localOut2);
    UserDefinedVT_A#(refInstanceA^.vt^).outputArguments^(refInstanceA^, out1 => localOut1, out2 => localOut2);
END_FUNCTION

