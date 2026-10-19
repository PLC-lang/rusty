FUNCTION_BLOCK FbPropertyA IMPLEMENTS IA
    METHOD foo : DINT
        foo := 1;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    propertyConsumerA : Consumer;
    propertyInstanceA : FbPropertyA;
END_VAR

FUNCTION_BLOCK OwnerA
    PROPERTY_GET value : DINT
        propertyConsumerA(in1 := propertyInstanceA);
        value := propertyConsumerA.result;
    END_PROPERTY
END_FUNCTION_BLOCK
