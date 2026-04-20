// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Critical issue #2: Calls inside WHILE loops are not lowered.
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

    WHILE tmpVal < 3 DO
        refVal REF= referenceFunc(refVal);
    END_WHILE;

    conVal := refVal;

    // CHECK: 3
    printf('%d$N', conVal);
END_FUNCTION
