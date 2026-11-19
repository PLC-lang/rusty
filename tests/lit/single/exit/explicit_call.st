// RUN: %COMPILE %s && %RUN | %CHECK %s
// `FB_EXIT` is a normal method for user code. A call from user code runs it, and the call at
// the end of the application follows as usual.
FUNCTION_BLOCK Token
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    token: Token := (id := 1);
END_VAR

FUNCTION main: DINT
    token.FB_EXIT();
    printf('main done$N');
    // CHECK: exit 1
    // CHECK-NEXT: main done
    // CHECK-NEXT: exit 1
    // CHECK-NOT: exit
END_FUNCTION
