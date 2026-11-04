// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: an interface output may be left unbound with `=>` and an empty argument, like any
// other output; the callee still runs.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo : DINT
        foo := 1;
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    gA : FbA;
END_VAR

FUNCTION f_out : DINT
    VAR_OUTPUT
        x : IA;
    END_VAR
    x := gA;
    f_out := 42;
END_FUNCTION

FUNCTION main : DINT
    // CHECK: unbound output 42
    printf('unbound output %d$N', f_out(x => ));
END_FUNCTION
