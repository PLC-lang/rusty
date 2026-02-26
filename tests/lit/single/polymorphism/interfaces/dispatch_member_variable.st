// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A function block stores an interface reference as a member variable. A concrete instance is
// assigned to it, and dispatch happens through the stored reference. The fat pointer must survive being
// written to and read back from the FB's struct fields.

INTERFACE IA
    METHOD greet
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbHello IMPLEMENTS IA
    METHOD greet
        printf('Hello$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbWorld IMPLEMENTS IA
    METHOD greet
        printf('World$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbConsumer
    VAR
        stored: IA;
    END_VAR

    METHOD store
        VAR_INPUT
            ref: IA;
        END_VAR

        stored := ref;
    END_METHOD

    METHOD invoke
        stored.greet();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        hello: FbHello;
        world: FbWorld;
        consumer: FbConsumer;
    END_VAR

    // Store FbHello, dispatch through stored member
    // CHECK: Hello
    consumer.store(hello);
    consumer.invoke();

    // Reassign to FbWorld, dispatch again
    // CHECK-NEXT: World
    consumer.store(world);
    consumer.invoke();

    // Back to FbHello to confirm no stale pointer
    // CHECK-NEXT: Hello
    consumer.store(hello);
    consumer.invoke();
END_FUNCTION
