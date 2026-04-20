// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Critical issue #2: Call-site lowering only handles top-level statements.
// Calls to reference-to-return functions inside control flow are not lowered.
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
