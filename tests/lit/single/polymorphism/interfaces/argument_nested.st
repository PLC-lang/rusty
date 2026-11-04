// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: An argument in parentheses or with the multiplier 1 fills one parameter slot. It is wrapped
// in a fat pointer like a plain argument, also when other arguments follow.

INTERFACE IA
    METHOD get : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD get : DINT
        get := 40;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION sum_with : DINT
    VAR_INPUT
        x : IA;
        y : DINT;
    END_VAR
    sum_with := x.get() + y;
END_FUNCTION

FUNCTION main
    VAR
        instanceFbA : FbA;
    END_VAR

    // CHECK: parentheses=42
    printf('parentheses=%d$N', sum_with((instanceFbA), 2));

    // CHECK-NEXT: multiplier=42
    printf('multiplier=%d$N', sum_with(1(instanceFbA), 2));
END_FUNCTION
