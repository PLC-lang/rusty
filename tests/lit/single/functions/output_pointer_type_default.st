// RUN: %COMPILE %s && %RUN | %CHECK %s
// A pointer-typed output keeps its constructor call, so an address-valued default of its type,
// which codegen cannot store as a constant, is applied on every call.
VAR_GLOBAL
    g : DINT := 99;
END_VAR

TYPE P : POINTER TO DINT := ADR(g); END_TYPE

FUNCTION_BLOCK Fb
    METHOD m
        VAR_OUTPUT
            out : P;
        END_VAR
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION f
    VAR_OUTPUT
        out : P;
    END_VAR
END_FUNCTION

FUNCTION main
    VAR
        fb : Fb;
        p : P;
    END_VAR
    p := ADR(p);
    f(out => p);
    printf('function: out^ = %d, out = ADR(g): %d$N', p^, p = ADR(g));
    // CHECK: function: out^ = 99, out = ADR(g): 1

    p := ADR(p);
    fb.m(out => p);
    printf('method: out^ = %d, out = ADR(g): %d$N', p^, p = ADR(g));
    // CHECK-NEXT: method: out^ = 99, out = ADR(g): 1
END_FUNCTION
