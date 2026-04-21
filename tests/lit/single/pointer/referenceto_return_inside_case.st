// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test case #2: verify calls inside CASE branches are lowered correctly.
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
        sel    : INT := 1;
    END_VAR

    tmpVal := 10;
    refVal REF= tmpVal;

    CASE sel OF
        1:
            refVal REF= referenceFunc(refVal);
        2:
            tmpVal := 99;
    END_CASE;

    conVal := refVal;

    // CHECK: 11
    printf('%d$N', conVal);
END_FUNCTION
