// RUN: %COMPILE %s && %RUN | %CHECK %s
// A local instance is exited when the call leaves the body through a `RETURN` in a `FOR` loop.
// The statements after the `RETURN` do not run.
FUNCTION_BLOCK Guard
    METHOD FB_EXIT
        printf('exit$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION work: DINT
    VAR
        guard: Guard;
        i: DINT;
    END_VAR

    FOR i := 0 TO 5 DO
        RETURN;
    END_FOR
    printf('not reached$N');
END_FUNCTION

FUNCTION main: DINT
    work();
    printf('main done$N');
    // CHECK-NOT: not reached
    // CHECK: exit
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
