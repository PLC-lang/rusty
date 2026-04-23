// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Verify: calling the same reference-to-return function multiple times at top level.
// The temp variable is shared across calls — verify values don't leak between calls.
FUNCTION referenceFunc : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    in := in + 1;
    referenceFunc REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refA   : REFERENCE TO INT;
        refB   : REFERENCE TO INT;
        valA   : INT;
        valB   : INT;
        tmpA   : INT;
        tmpB   : INT;
    END_VAR

    tmpA := 10;
    tmpB := 20;
    refA REF= tmpA;
    refB REF= tmpB;

    refA REF= referenceFunc(refA);  // 10 + 1 = 11
    refB REF= referenceFunc(refB);  // 20 + 1 = 21

    valA := refA;
    valB := refB;

    // CHECK: 11
    printf('%d$N', valA);
    // CHECK: 21
    printf('%d$N', valB);
END_FUNCTION
