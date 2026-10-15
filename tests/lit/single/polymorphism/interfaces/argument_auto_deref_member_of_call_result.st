// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A REFERENCE TO member read from a call result names the storage the reference points to,
// not the result's temporary, so it can be passed to a by-reference interface parameter.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

TYPE Holder : STRUCT
    r : REFERENCE TO IA;
END_STRUCT
END_TYPE

VAR_GLOBAL
    globalInstance : FbA;
    globalRef : IA;
END_VAR

FUNCTION makeHolder : Holder
    globalRef := globalInstance;
    makeHolder.r REF= globalRef;
END_FUNCTION

FUNCTION consume
    VAR_IN_OUT
        inout1 : IA;
    END_VAR

    inout1.foo();
END_FUNCTION

FUNCTION main
    // CHECK: FbA::foo
    consume((makeHolder()).r);
END_FUNCTION
