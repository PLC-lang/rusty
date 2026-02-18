// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Interface references passed as positional (unnamed) VAR_INPUT arguments are correctly wrapped
// in a fat pointer and dispatched to the implementing function block's method.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    METHOD foo
        printf('FbB::foo$N');
    END_METHOD
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
        consumer: FbConsumer;
    END_VAR

    // CHECK: FbA::foo
    consumer.bar(instanceFbA);

    // CHECK-NEXT: FbB::foo
    consumer.bar(instanceFbB);
END_FUNCTION
