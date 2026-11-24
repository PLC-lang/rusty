// RUN: %COMPILE %s && %RUN | %CHECK %s
// The instance that a function returns is initialized and exited on each call. The caller
// receives a copy, which is exited again with the variable that holds it.
FUNCTION_BLOCK Token
    VAR
        id: DINT;
    END_VAR

    METHOD FB_INIT
        printf('init$N');
    END_METHOD

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION makeToken: Token
    makeToken.id := 7;
    printf('make$N');
END_FUNCTION

VAR_GLOBAL
    kept: Token;
END_VAR

FUNCTION main: DINT
    printf('main$N');
    kept := makeToken();
    printf('main done$N');
    // CHECK: init
    // CHECK-NEXT: main
    // CHECK-NEXT: init
    // CHECK-NEXT: make
    // CHECK-NEXT: exit 7
    // CHECK-NEXT: main done
    // CHECK-NEXT: exit 7
    // CHECK-NOT: exit
END_FUNCTION
