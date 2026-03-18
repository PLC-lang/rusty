// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION referenceFunc : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    referenceFunc REF= in;
END_FUNCTION

FUNCTION main
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
    END_VAR

    refVal := 11;
    refVal REF= referenceFunc(refVal);
    conVal := refVal;

    // CHECK: 11
    printf('%d$N', conVal);
END_FUNCTION
