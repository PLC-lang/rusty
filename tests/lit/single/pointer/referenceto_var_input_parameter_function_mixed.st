// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Mixes reference and plain arguments to `REFERENCE TO` inputs of several
// functions in one unit. This exact combination once produced mistyped call
// IR that made the LLVM backend abort with "Cannot emit physreg copy
// instruction", so it also pins compile robustness.

VAR_GLOBAL
    savedRef : REFERENCE TO DINT;
END_VAR

FUNCTION saveReference
VAR_INPUT
    refIn : REFERENCE TO DINT;
END_VAR
    savedRef REF= refIn;
END_FUNCTION

FUNCTION increment
VAR_INPUT
    refIn : REFERENCE TO DINT;
END_VAR
    refIn := refIn + 1;
END_FUNCTION

FUNCTION main : DINT
    VAR
        x : DINT := 41;
        localRef : REFERENCE TO DINT;
    END_VAR

    localRef REF= x;
    saveReference(refIn := localRef);
    // CHECK: savedRef targets x (reference argument): 1
    printf('savedRef targets x (reference argument): %d$N', REF(savedRef) = REF(x));

    savedRef REF= 0;

    saveReference(refIn := x);
    // CHECK: savedRef targets x (plain argument): 1
    printf('savedRef targets x (plain argument): %d$N', REF(savedRef) = REF(x));

    increment(refIn := x);
    // CHECK: x after increment: 42
    printf('x after increment: %d$N', x);
END_FUNCTION
