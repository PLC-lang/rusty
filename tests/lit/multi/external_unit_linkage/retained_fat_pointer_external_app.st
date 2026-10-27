INTERFACE IA
    METHOD foo : DINT END_METHOD
END_INTERFACE

{external}
FUNCTION_BLOCK Consumer
VAR_INPUT
    in1 : REFERENCE TO IA;
END_VAR
VAR_OUTPUT
    result : DINT;
END_VAR
END_FUNCTION_BLOCK

{external}
FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo : DINT END_METHOD
END_FUNCTION_BLOCK

// The declaration keeps the library function's body. Codegen omits it, so the retained fat pointer
// global that the body binds must only be declared here and defined by the library.
{external}
FUNCTION bindExt
VAR_IN_OUT
    consumer : Consumer;
    instance : FbA;
END_VAR
    consumer(instance);
END_FUNCTION

FUNCTION main
VAR
    consumer : Consumer;
    instance : FbA;
END_VAR
    bindExt(consumer, instance);
    consumer();
    printf('%d$N', consumer.result);
END_FUNCTION
