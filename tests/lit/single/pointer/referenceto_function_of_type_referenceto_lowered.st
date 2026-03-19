// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION referenceFunc
    VAR_INPUT
        __referenceFunc_return_val : REFERENCE TO INT;
        in	: REFERENCE TO INT;
    END_VAR

    printf('in 1: %d$N', in);

    in := in + 1;

    printf('in 2: %d$N', in);

    __referenceFunc_return_val REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
        tmpVal : INT;
    END_VAR

    VAR_TEMP
        __referenceFunc_return_val : REFERENCE TO INT;
    END_VAR

    printf('Testing...$N');

    tmpVal := 11;
    refVal REF= tmpVal;
    referenceFunc(__referenceFunc_return_val, refVal);
    // Segfaults here... why??
    refVal REF= __referenceFunc_return_val;
    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
