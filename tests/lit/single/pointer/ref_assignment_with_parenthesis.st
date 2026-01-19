// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION main : DINT
    VAR
        refe: REFERENCE TO DINT;
        myDint : DINT;
    END_VAR

    myDint := 10;
    refe REF= (myDint);

    // CHECK: 10
    printf('%d$N', refe);
END_FUNCTION
