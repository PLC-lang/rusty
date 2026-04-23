// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Verify: calling multiple different reference-to-return functions from the same POU.
FUNCTION addOne : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    in := in + 1;
    addOne REF= in;
END_FUNCTION

FUNCTION doubleIt : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    in := in * 2;
    doubleIt REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
        tmpVal : INT;
    END_VAR

    tmpVal := 5;
    refVal REF= tmpVal;

    refVal REF= addOne(refVal);    // 5 + 1 = 6
    refVal REF= doubleIt(refVal);  // 6 * 2 = 12
    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
