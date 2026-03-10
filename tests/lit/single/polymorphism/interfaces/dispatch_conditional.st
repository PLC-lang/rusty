// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Different concrete types are assigned to an interface variable based on a runtime condition.
// The method is then called after the branch, ensuring dispatch is truly dynamic and not resolved at
// compile time.

INTERFACE IA
    METHOD identify
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbAlpha IMPLEMENTS IA
    METHOD identify
        printf('Alpha$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbBravo IMPLEMENTS IA
    METHOD identify
        printf('Bravo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbCharlie IMPLEMENTS IA
    METHOD identify
        printf('Charlie$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION pick_and_call
    VAR_INPUT
        selector: DINT;
    END_VAR

    VAR
        alpha: FbAlpha;
        bravo: FbBravo;
        charlie: FbCharlie;
        reference: IA;
    END_VAR

    IF selector = 1 THEN
        reference := alpha;
    ELSIF selector = 2 THEN
        reference := bravo;
    ELSE
        reference := charlie;
    END_IF;

    reference.identify();
END_FUNCTION

FUNCTION pick_case
    VAR_INPUT
        selector: DINT;
    END_VAR

    VAR
        alpha: FbAlpha;
        bravo: FbBravo;
        charlie: FbCharlie;
        reference: IA;
    END_VAR

    CASE selector OF
        1: reference := alpha;
        2: reference := bravo;
    ELSE
        reference := charlie;
    END_CASE;

    reference.identify();
END_FUNCTION

FUNCTION main
    // IF-based selection
    // CHECK: Alpha
    pick_and_call(1);

    // CHECK-NEXT: Bravo
    pick_and_call(2);

    // CHECK-NEXT: Charlie
    pick_and_call(3);

    // CASE-based selection
    // CHECK-NEXT: Alpha
    pick_case(1);

    // CHECK-NEXT: Bravo
    pick_case(2);

    // CHECK-NEXT: Charlie
    pick_case(99);
END_FUNCTION
