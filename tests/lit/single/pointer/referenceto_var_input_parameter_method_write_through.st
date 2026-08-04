// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// A reference member assigned from a `REFERENCE TO` method input must target
// the caller's variable: a later write through the member must reach it.

FUNCTION_BLOCK TestFB
VAR
    storedRef : REFERENCE TO LINT;
END_VAR

METHOD setRef
VAR_INPUT
    refIn : REFERENCE TO LINT;
END_VAR
    storedRef REF= refIn;
END_METHOD

METHOD addThroughRef
VAR_INPUT
    value : LINT;
END_VAR
    storedRef := storedRef + value;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb : TestFB;
        target : LINT := 5;
    END_VAR

    fb.setRef(refIn := target);
    fb.addThroughRef(value := 10);
    // CHECK: target after addThroughRef: 15
    printf('target after addThroughRef: %d$N', target);
END_FUNCTION
