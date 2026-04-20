// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Critical issue #3: Only the last statement in the function body is checked for
// return lowering. If the REF= assignment to the function name isn't the last
// statement, the lowering is skipped.
FUNCTION referenceFunc : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR
    VAR
        dummy : INT;
    END_VAR

    in := in + 1;
    referenceFunc REF= in;
    dummy := 0; // this is the last statement, so the REF= above won't be lowered
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
