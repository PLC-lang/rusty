// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Verify calls inside WHILE loops are lowered.
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

    tmpVal := 0;
    refVal REF= tmpVal;

    WHILE refVal < 3 DO
        refVal REF= referenceFunc(refVal);
    END_WHILE;

    conVal := refVal;

    // CHECK: 3
    printf('%d$N', conVal);
END_FUNCTION
