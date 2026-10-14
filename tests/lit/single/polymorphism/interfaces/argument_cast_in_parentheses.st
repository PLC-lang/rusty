// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A concrete instance behind a cast written with parentheses is still a variable, so it can be
// passed to a by-reference interface parameter like the instance itself.

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
    consumer : Consumer;
    instance : FbA;
END_VAR

FUNCTION main
    // CHECK: FbA::foo
    consumer(FbA#(instance));

    // CHECK-NEXT: FbA::foo
    consumer(FbA#instance);
END_FUNCTION
