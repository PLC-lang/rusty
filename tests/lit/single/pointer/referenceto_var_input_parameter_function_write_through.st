// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// A write through a `REFERENCE TO` VAR_INPUT must reach the caller's
// variable, also when the argument is a plain variable.

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

    // Control: passing a reference variable works.
    localRef REF= x;
    increment(refIn := localRef);
    // CHECK: x after increment (reference argument): 42
    printf('x after increment (reference argument): %d$N', x);

    increment(refIn := x);
    // CHECK: x after increment (plain argument): 43
    printf('x after increment (plain argument): %d$N', x);
END_FUNCTION
