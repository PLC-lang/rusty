// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Verify: Standalone calls (without REF= assignment) to reference-to-return
// functions have their signatures rewritten so the call compiles.
// Top-level standalone calls are already handled by this rewriting.
FUNCTION referenceFunc : REFERENCE TO INT
    VAR_INPUT
        in	: REFERENCE TO INT;
    END_VAR

    in := in + 1;
    referenceFunc REF= in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        conVal : INT;
        tmpVal : INT;
    END_VAR

    tmpVal := 11;
    refVal REF= tmpVal;

    // Call without capturing the return - should still compile & run with the
    // rewritten void signature (the side effect on `in` still matters)
    referenceFunc(refVal);

    conVal := refVal;

    // CHECK: 12
    printf('%d$N', conVal);
END_FUNCTION
