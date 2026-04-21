// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression test for critical issue #2: ensure call-site lowering works
// for reference-to-return function calls inside IF control flow.
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
        flag   : BOOL := TRUE;
    END_VAR

    tmpVal := 11;
    refVal REF= tmpVal;

    IF flag THEN
        refVal REF= referenceFunc(refVal);
    END_IF;

    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
