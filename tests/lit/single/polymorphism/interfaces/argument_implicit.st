// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: FbA implements IA, FbB extends FbA and implicitly implements IA. Both should be passable as
// interface-typed arguments, with FbB's overridden method dispatched correctly.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB EXTENDS FbA
    METHOD foo
        printf('FbB::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC EXTENDS FbB
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbConsumer
    METHOD bar
        VAR_INPUT
            refIA: IA;
        END_VAR

        refIA.foo();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
        instanceFbB: FbB;
        instanceFbC: FbC;
        consumer: FbConsumer;
    END_VAR

    // CHECK: FbA::foo
    consumer.bar(refIA := instanceFbA);

    // CHECK-NEXT: FbB::foo
    consumer.bar(refIA := instanceFbB);

    // FbC inherits FbB's override
    // CHECK-NEXT: FbB::foo
    consumer.bar(refIA := instanceFbC);
END_FUNCTION
