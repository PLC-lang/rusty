// RUN: %COMPILE %s && %RUN | %CHECK %s
// A scalar type whose default is an address, which codegen cannot store as a constant, is applied
// to an output of that type by the type's constructor on every call, in a function and a method.
VAR_GLOBAL
    g : DINT := 99;
END_VAR

TYPE A : LWORD := ADR(g); END_TYPE

FUNCTION_BLOCK Fb
    METHOD m
        VAR_OUTPUT
            out : A;
        END_VAR
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION f
    VAR_OUTPUT
        out : A;
    END_VAR
END_FUNCTION

FUNCTION main
    VAR
        fb : Fb;
        a : A;
    END_VAR
    a := 1;
    f(out => a);
    printf('function: out = ADR(g): %d$N', a = ADR(g));
    // CHECK: function: out = ADR(g): 1

    a := 1;
    fb.m(out => a);
    printf('method: out = ADR(g): %d$N', a = ADR(g));
    // CHECK-NEXT: method: out = ADR(g): 1
END_FUNCTION
