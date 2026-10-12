// RUN: %COMPILE %s && %RUN | %CHECK %s
// An output initializer that needs the instance, such as ADR(THIS^), is applied by the lowered
// stack initializer of the method; the entry reset must not try to evaluate it.
FUNCTION_BLOCK Fb
    VAR
        x : INT := 5;
    END_VAR

    METHOD m
        VAR_OUTPUT
            self_ptr : POINTER TO Fb := ADR(THIS^);
            n : INT := 9;
        END_VAR
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fb : Fb;
        p : POINTER TO Fb;
        k : INT := 1;
    END_VAR

    fb.m(self_ptr => p, n => k);
    printf('p^.x = %d, p = ADR(fb): %d, k = %d$N', p^.x, p = ADR(fb), k);
    // CHECK: p^.x = 5, p = ADR(fb): 1, k = 9
END_FUNCTION
