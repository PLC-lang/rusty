// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Different concrete types are assigned to an interface variable based on a runtime
// condition. The property is then accessed after the branch, ensuring dispatch is truly dynamic.

INTERFACE IA
    PROPERTY value : DINT
        GET END_GET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbAlpha IMPLEMENTS IA
    PROPERTY value : DINT
        GET
            value := 111;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbBravo IMPLEMENTS IA
    PROPERTY value : DINT
        GET
            value := 222;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbCharlie IMPLEMENTS IA
    PROPERTY value : DINT
        GET
            value := 333;
        END_GET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION pick_and_read
    VAR_INPUT
        selector : DINT;
    END_VAR

    VAR
        alpha : FbAlpha;
        bravo : FbBravo;
        charlie : FbCharlie;
        reference : IA;
    END_VAR

    IF selector = 1 THEN
        reference := alpha;
    ELSIF selector = 2 THEN
        reference := bravo;
    ELSE
        reference := charlie;
    END_IF;

    printf('%d$N', reference.value);
END_FUNCTION

FUNCTION main
    pick_and_read(1);   // CHECK: 111
    pick_and_read(2);   // CHECK: 222
    pick_and_read(3);   // CHECK: 333
END_FUNCTION
