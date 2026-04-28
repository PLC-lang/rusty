// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Multiple sequential calls pass different concrete types to the same interface-typed parameter.
// The single __tmp variable is reused across calls — each call should dispatch to the correct implementation.

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

FUNCTION_BLOCK FbC IMPLEMENTS IA
    METHOD foo
        printf('FbC::foo$N');
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
        instanceFbC: FbC;
        consumer: FbConsumer;
    END_VAR

    // CHECK: FbA::foo
    consumer.bar(refIA := instanceFbA);

    // CHECK-NEXT: FbB::foo
    consumer.bar(refIA := instanceFbB);

    // CHECK-NEXT: FbC::foo
    consumer.bar(refIA := instanceFbC);

    // Back to FbA to confirm __tmp is correctly overwritten
    // CHECK-NEXT: FbA::foo
    consumer.bar(refIA := instanceFbA);
END_FUNCTION
