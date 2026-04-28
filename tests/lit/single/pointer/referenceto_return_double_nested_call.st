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

FUNCTION tripleIt : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    in := in * 3;
    tripleIt REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
        tmpVal : INT;
    END_VAR

    tmpVal := 5;
    refVal REF= tmpVal;

    // Nested: tripleIt(doubleIt(addOne(refVal))) should give ((5+1)*2)*3 = 36
    refVal REF= tripleIt(doubleIt(addOne(refVal)));
    conVal := refVal;

    // CHECK: 36
    printf('%d$N', conVal);
END_FUNCTION
