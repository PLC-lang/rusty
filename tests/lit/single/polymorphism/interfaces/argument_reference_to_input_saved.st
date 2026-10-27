// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A method saves its REFERENCE TO interface input with REF=. A concrete instance bound in one
// function must still be reachable through the saved reference after that function returned, so the
// fat pointer behind a REFERENCE TO parameter cannot live on the binding function's stack, also for a
// function or method.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Holder
    VAR
        storedRef : REFERENCE TO IA;
    END_VAR

    METHOD setRef
        VAR_INPUT
            refIn : REFERENCE TO IA;
        END_VAR

        storedRef REF= refIn;
    END_METHOD

    METHOD useRef
        storedRef.foo();
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    holder : Holder;
    globalInstance : FbA;
END_VAR

FUNCTION bind
    holder.setRef(globalInstance);
END_FUNCTION

FUNCTION clobber : LINT
    VAR
        buffer : ARRAY[0..511] OF LINT;
        i : DINT;
    END_VAR

    FOR i := 0 TO 511 DO
        buffer[i] := -1;
    END_FOR
    clobber := buffer[0] + buffer[511];
END_FUNCTION

FUNCTION main
    bind();

    // CHECK: main: clobber = -2
    printf('main: clobber = %lld$N', clobber());

    // CHECK-NEXT: FbA::foo
    holder.useRef();
END_FUNCTION
