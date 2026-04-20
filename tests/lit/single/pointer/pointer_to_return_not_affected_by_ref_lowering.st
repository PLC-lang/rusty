// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Critical issue #1: The lowering matches all PointerType returns, not just REFERENCE TO.
// A function returning POINTER TO INT should NOT be rewritten by the reference-to-return
// lowering — it should keep its return type and work normally.
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
