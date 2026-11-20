// RUN: %COMPILE %s && %RUN | %CHECK %s
// An assignment copies an instance with no call of `FB_INIT` or `FB_EXIT`. Each variable is
// exited one time, with the data it holds at that moment.
FUNCTION_BLOCK Token
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    target: Token := (id := 1);
    source: Token := (id := 2);
END_VAR

FUNCTION main: DINT
    target := source;
    printf('assigned$N');
    printf('main done$N');
    // CHECK: assigned
    // CHECK-NEXT: main done
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: exit 2
    // CHECK-NOT: exit
END_FUNCTION
