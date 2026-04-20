// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Critical issue #2: Calls inside FOR loops are not lowered.
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
        i      : INT;
    END_VAR

    tmpVal := 0;
    refVal REF= tmpVal;

    FOR i := 1 TO 5 DO
        refVal REF= referenceFunc(refVal);
    END_FOR;

    conVal := refVal;

    // CHECK: 5
    printf('%d$N', conVal);
END_FUNCTION
