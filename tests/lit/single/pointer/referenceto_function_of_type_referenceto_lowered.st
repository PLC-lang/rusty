// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Non-lowered code
/*
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
    END_VAR

    tmpVal := 11;
    refVal REF= tmpVal;
    refVal REF= referenceFunc(refVal);
    conVal := refVal;
END_FUNCTION
*/

FUNCTION referenceFunc
    VAR_INPUT
        __referenceFunc_return_val : REFERENCE TO INT;
        in	: REFERENCE TO INT;
    END_VAR

    in := in + 1;
    __referenceFunc_return_val := in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
        tmpVal : INT;
    END_VAR

    VAR_TEMP
        __referenceFunc_return_val : REFERENCE TO INT;
        __referenceFunc_return_val_store : INT;
    END_VAR

    tmpVal := 11;
    refVal REF= tmpVal;
    __referenceFunc_return_val REF= __referenceFunc_return_val_store;
    referenceFunc(__referenceFunc_return_val, refVal);
    refVal REF= __referenceFunc_return_val;
    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
