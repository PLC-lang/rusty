// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression test for critical issue #3: return lowering must still occur even
// when the REF= assignment to the function name is not the last statement in
// the function body.
FUNCTION referenceFunc : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR
    VAR
        dummy : INT;
    END_VAR

    in := in + 1;
    referenceFunc REF= in;
    dummy := 0; // keep a trailing statement to verify the REF= above is still lowered
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
