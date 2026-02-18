// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A method takes a mix of interface-typed and non-interface-typed parameters. Only the
// interface-typed arguments should be wrapped in a fat pointer; the rest remain untouched.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbConsumer
    METHOD bar
        VAR_INPUT
            x: DINT;
            refIA: IA;
            y: DINT;
        END_VAR

        printf('x=%d, y=%d$N', x, y);
        refIA.foo();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
        consumer: FbConsumer;
    END_VAR

    // CHECK: x=1, y=2
    // CHECK-NEXT: FbA::foo
    consumer.bar(x := 1, refIA := instanceFbA, y := 2);
END_FUNCTION
