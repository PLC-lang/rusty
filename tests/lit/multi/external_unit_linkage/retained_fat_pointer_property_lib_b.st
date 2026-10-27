FUNCTION_BLOCK FbPropertyB IMPLEMENTS IA
    METHOD foo : DINT
        foo := 2;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    propertyConsumerB : Consumer;
    propertyInstanceB : FbPropertyB;
END_VAR

FUNCTION_BLOCK OwnerB
    PROPERTY_GET value : DINT
        propertyConsumerB(in1 := propertyInstanceB);
        value := propertyConsumerB.result;
    END_PROPERTY
END_FUNCTION_BLOCK
