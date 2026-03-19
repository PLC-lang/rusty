// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION referenceFunc : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    in := in + 1;
    referenceFunc REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
        tmpVal : INT;
    END_VAR

    tmpVal := 11;
    refVal REF= tmpVal;
    refVal REF= referenceFunc(refVal);
    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
