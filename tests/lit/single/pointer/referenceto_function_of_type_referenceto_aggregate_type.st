// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION referenceFunc : REFERENCE TO STRING
    VAR_INPUT
        in	: REFERENCE TO STRING;
    END_VAR

    referenceFunc REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO STRING;
        conVal : STRING;
        tmpVal : STRING := 'Hello world!';
    END_VAR

    refVal REF= tmpVal;
    refVal REF= referenceFunc(refVal);
    conVal := refVal;

    // CHECK: Hello world!
    printf('%s$N', conVal);
END_FUNCTION
