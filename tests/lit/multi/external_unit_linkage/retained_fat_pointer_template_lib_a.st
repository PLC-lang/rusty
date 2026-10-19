FUNCTION_BLOCK FbTemplateA IMPLEMENTS IA
    METHOD foo : DINT
        foo := 1;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    templateConsumerA : Consumer;
    templateInstanceA : FbTemplateA;
END_VAR

FUNCTION bind<T: ANY_INT> : T
VAR_INPUT
    value : T;
END_VAR
    templateConsumerA(in1 := templateInstanceA);
    bind := value;
END_FUNCTION
