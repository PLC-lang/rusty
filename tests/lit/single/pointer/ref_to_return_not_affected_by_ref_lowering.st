// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression test: functions returning REF_TO must not be rewritten by the
// reference-to-return lowering.
FUNCTION getRef : REF_TO INT
    VAR_INPUT
        in	: REF_TO INT;
    END_VAR

    getRef := in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        ptr    : REF_TO INT;
        tmpVal : INT := 77;
    END_VAR

    ptr := getRef(REF(tmpVal));

    // CHECK: 77
    printf('%d$N', ptr^);
END_FUNCTION
