// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION referenceFunc : REFERENCE TO ARRAY[1..3] OF INT
    VAR_INPUT
        in	: REFERENCE TO ARRAY[1..3] OF INT;
    END_VAR

    referenceFunc REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO ARRAY[1..3] OF INT;
        conVal : ARRAY[1..3] OF INT;
        tmpVal : ARRAY[1..3] OF INT;
    END_VAR

    refVal REF= tmpVal;
    tmpVal[1] := 3;
    tmpVal[2] := 2;
    tmpVal[3] := 1;

    refVal REF= referenceFunc(refVal);
    conVal := refVal;

    // CHECK: 1, 2, 3
    printf('%d, %d, %d$N', conVal[3], conVal[2], conVal[1]);
END_FUNCTION
