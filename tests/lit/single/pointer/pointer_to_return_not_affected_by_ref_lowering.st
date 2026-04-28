// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression test: ensure reference-to-return lowering does not affect POINTER TO returns.
// A function returning POINTER TO INT should keep its return type and continue to work
// normally rather than being lowered as though it returned REFERENCE TO.
FUNCTION getPointer : POINTER TO INT
    VAR_INPUT
        in	: REF_TO INT;
    END_VAR

    getPointer := in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        ptr    : POINTER TO INT;
        tmpVal : INT := 42;
    END_VAR

    ptr := getPointer(REF(tmpVal));

    // CHECK: 42
    printf('%d$N', ptr^);
END_FUNCTION
