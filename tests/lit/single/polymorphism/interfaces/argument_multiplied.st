// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A multiplied argument `2(d)` fills two parameter slots. The interface argument after it
// binds to the slot after both, so it is wrapped in a fat pointer for that interface parameter.

INTERFACE IA
    METHOD get : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD get : DINT
        get := 40;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION sum : DINT
    VAR_IN_OUT
        n1, n2 : DINT;
    END_VAR
    VAR_INPUT
        x : IA;
    END_VAR
    sum := n1 + n2 + x.get();
END_FUNCTION

FUNCTION sum_interfaces : DINT
    VAR_IN_OUT
        x, y : IA;
    END_VAR
    sum_interfaces := x.get() + y.get();
END_FUNCTION

FUNCTION main
    VAR
        d : DINT := 1;
        instanceFbA : FbA;
        refIA : IA;
    END_VAR

    // CHECK: sum=42
    printf('sum=%d$N', sum(2(d), instanceFbA));

    // CHECK-NEXT: sum_interfaces=80
    refIA := instanceFbA;
    printf('sum_interfaces=%d$N', sum_interfaces(2(refIA)));
END_FUNCTION
