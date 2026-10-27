// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A call that binds a concrete instance to a saved REFERENCE TO interface parameter sits
// behind AND_THEN. The binding must only happen when the call runs, so a skipped call leaves the
// previously saved reference untouched.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    VAR_INPUT
        id : DINT;
    END_VAR

    METHOD foo
        printf('FbA::foo %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Holder
    VAR
        storedRef : REFERENCE TO IA;
    END_VAR

    METHOD bind : BOOL
        VAR_INPUT
            refIn : REFERENCE TO IA;
        END_VAR

        storedRef REF= refIn;
        bind := TRUE;
    END_METHOD

    METHOD useRef
        storedRef.foo();
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    holder : Holder;
    instanceA : FbA := (id := 1);
    instanceB : FbA := (id := 2);
END_VAR

FUNCTION bindIf : BOOL
    VAR_INPUT
        enabled : BOOL;
    END_VAR
    VAR_IN_OUT
        instance : FbA;
    END_VAR

    bindIf := enabled AND_THEN holder.bind(instance);
END_FUNCTION

FUNCTION main
    // CHECK: FbA::foo 1
    bindIf(TRUE, instanceA);
    holder.useRef();

    // CHECK-NEXT: FbA::foo 1
    bindIf(FALSE, instanceB);
    holder.useRef();

    // CHECK-NEXT: FbA::foo 2
    bindIf(TRUE, instanceB);
    holder.useRef();
END_FUNCTION
