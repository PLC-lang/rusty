// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure interface references passed as VAR_INPUT arguments are correctly dispatched to the
// implementing function block's method.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

INTERFACE IB
    METHOD bar
        VAR_INPUT
            refIA: IA;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IB
    VAR_INPUT
        refIA: IA;
    END_VAR

    METHOD bar
        VAR_INPUT
            refIA: IA;
        END_VAR

        refIA.foo();
        printf('FbB::bar$N');
    END_METHOD

    refIA.foo();
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
        instanceFbB: FbB;
    END_VAR

    // CHECK: FbA::foo
    instanceFbB(refIA := instanceFbA);

    // CHECK-NEXT: FbA::foo
    // CHECK-NEXT: FbB::bar
    instanceFbB.bar(refIA := instanceFbA);
END_FUNCTION