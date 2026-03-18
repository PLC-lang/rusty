// RUN: (%COMPILE %s && %RUN) | %CHECK %s
/*
    Non-lowered code:

    FUNCTION referenceFunc : REFERENCE TO INT
        VAR_INPUT
            in	: REFERENCE TO INT;
        END_VAR

        referenceFunc REF= in;
    END_FUNCTION

    FUNCTION main
        VAR
            refVal : REFERENCE TO INT;
            conVal : INT;
        END_VAR

        refVal := 11;
        refVal REF= referenceFunc(refVal);
        conVal := refVal;

        // CHECK: 11
        printf('%d$N', conVal);
    END_FUNCTION
*/

FUNCTION referenceFunc
    VAR_INPUT
        __referenceFunc_return_val : REFERENCE TO INT;
        in	: REFERENCE TO INT;
    END_VAR

    __referenceFunc_return_val REF= in;
END_FUNCTION

FUNCTION main
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
    END_VAR

    VAR_TEMP
        __referenceFunc_return_val : REFERENCE TO INT;
    END_VAR

    refVal := 11;
    referenceFunc(__referenceFunc_return_val, refVal);
    refVal REF= __referenceFunc_return_val;
    conVal := refVal;

    // CHECK: 11
    printf('%d$N', conVal);
END_FUNCTION
