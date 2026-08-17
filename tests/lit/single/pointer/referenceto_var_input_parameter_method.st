// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// A `REFERENCE TO` VAR_INPUT of a method must receive the address of a plain
// variable argument. Field report: a reference member assigned from such an
// input inside a method stays null (or holds the argument's value as an
// address). This test only inspects the stored pointer identity and never
// dereferences it, so it fails with a value mismatch instead of a crash.

FUNCTION_BLOCK TestFB
VAR
    storedRef : REFERENCE TO LINT;
    memberRef : REFERENCE TO LINT;
    memberVal : LINT;
END_VAR

METHOD setRef
VAR_INPUT
    refIn : REFERENCE TO LINT;
END_VAR
    storedRef REF= refIn;
END_METHOD

METHOD cyclic
    memberRef REF= memberVal;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        fb : TestFB;
        target : LINT := 5;
    END_VAR

    // Control: REF= between members inside a method works.
    fb.cyclic();
    // CHECK: memberRef targets memberVal: 1
    printf('memberRef targets memberVal: %d$N', REF(fb.memberRef) = REF(fb.memberVal));

    fb.setRef(refIn := target);
    // CHECK: storedRef targets target: 1
    printf('storedRef targets target: %d$N', REF(fb.storedRef) = REF(target));
END_FUNCTION
