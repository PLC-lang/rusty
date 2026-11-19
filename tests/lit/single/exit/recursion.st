// RUN: %COMPILE %s && %RUN | %CHECK %s
// Each level of a recursion has its own local instance, which is exited when that level returns.
FUNCTION_BLOCK Guard
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION descend: DINT
    VAR_INPUT
        depth: DINT;
    END_VAR
    VAR
        guard: Guard;
    END_VAR

    guard.id := depth;
    IF depth < 3 THEN
        descend(depth + 1);
    END_IF
END_FUNCTION

FUNCTION main: DINT
    descend(1);
    printf('main done$N');
    // CHECK: exit 3
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: exit 1
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
