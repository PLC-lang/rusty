// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// A plain variable passed to a `REFERENCE TO` VAR_INPUT must arrive as the
// address of that variable, not as its value. This test only inspects the
// stored pointer identity and never dereferences it, so it fails with a
// value mismatch instead of a crash.

VAR_GLOBAL
    savedRef : REFERENCE TO DINT;
END_VAR

FUNCTION saveReference
VAR_INPUT
    refIn : REFERENCE TO DINT;
END_VAR
    savedRef REF= refIn;
END_FUNCTION

FUNCTION main : DINT
    VAR
        x : DINT := 41;
        localRef : REFERENCE TO DINT;
    END_VAR

    // Control: passing a reference variable works.
    localRef REF= x;
    saveReference(refIn := localRef);
    // CHECK: savedRef targets x (reference argument): 1
    printf('savedRef targets x (reference argument): %d$N', REF(savedRef) = REF(x));

    savedRef REF= 0;

    // Passing a plain variable must implicitly pass its address.
    saveReference(refIn := x);
    // CHECK: savedRef targets x (plain argument): 1
    printf('savedRef targets x (plain argument): %d$N', REF(savedRef) = REF(x));
END_FUNCTION
