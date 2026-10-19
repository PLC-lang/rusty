FUNCTION_BLOCK FbOverloadB IMPLEMENTS IA
    METHOD foo : DINT
        foo := 2;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    overloadConsumerB : Consumer;
    overloadInstanceB : FbOverloadB;
END_VAR

FUNCTION bind<U: ANY_REAL> : U
VAR_INPUT
    value : U;
END_VAR
    overloadConsumerB(in1 := overloadInstanceB);
    bind := value;
END_FUNCTION
