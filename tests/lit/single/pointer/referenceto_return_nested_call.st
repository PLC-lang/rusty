// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Verify: nested call - result of one ref-return function passed directly to another.
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

    // Nested: doubleIt(addOne(refVal)) should give (5+1)*2 = 12
    refVal REF= doubleIt(addOne(refVal));
    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
