// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A function block keeps a REFERENCE TO interface input between calls. A concrete instance
// bound in one function must still be reachable after that function returned, so the fat pointer
// behind the reference cannot live on the binding function's stack.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Consumer
    VAR_INPUT
        in1 : REFERENCE TO IA;
    END_VAR

    in1.foo();
END_FUNCTION_BLOCK

VAR_GLOBAL
    globalConsumer : Consumer;
    globalInstance : FbA;
END_VAR

FUNCTION bind
    globalConsumer(globalInstance);
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
    // CHECK: FbA::foo
    bind();

    // CHECK-NEXT: main: clobber = -2
    printf('main: clobber = %lld$N', clobber());

    // CHECK-NEXT: FbA::foo
    globalConsumer();
END_FUNCTION
