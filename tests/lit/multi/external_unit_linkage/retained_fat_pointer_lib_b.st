FUNCTION_BLOCK FbB IMPLEMENTS IA
    METHOD foo : DINT
        foo := 2;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    consumerB : Consumer;
    instanceB : FbB;
END_VAR

FUNCTION bindB
    consumerB(instanceB);
END_FUNCTION
