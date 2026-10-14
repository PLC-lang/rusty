FUNCTION_BLOCK FbTemplateB IMPLEMENTS IA
    METHOD foo : DINT
        foo := 1;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    templateConsumerB : Consumer;
    templateInstanceB : FbTemplateB;
END_VAR

FUNCTION bind<U: ANY_INT> : U
VAR_INPUT
    value : U;
END_VAR
    templateConsumerB(in1 := templateInstanceB);
    bind := value;
END_FUNCTION
