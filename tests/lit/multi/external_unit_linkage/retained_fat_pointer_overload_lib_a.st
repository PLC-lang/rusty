FUNCTION_BLOCK FbOverloadA IMPLEMENTS IA
    METHOD foo : DINT
        foo := 1;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    overloadConsumerA : Consumer;
    overloadInstanceA : FbOverloadA;
END_VAR

FUNCTION bind<T: ANY_INT> : T
VAR_INPUT
    value : T;
END_VAR
    overloadConsumerA(in1 := overloadInstanceA);
    bind := value;
END_FUNCTION
