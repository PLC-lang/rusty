// RUN: %COMPILE %s && %RUN | %CHECK %s
// A `VAR_IN_OUT` of a function block or a function refers to an instance of the caller. It
// causes no additional `FB_EXIT`: the one instance is exited one time.
FUNCTION_BLOCK Slot
    METHOD FB_EXIT
        printf('slot$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK User
    VAR_IN_OUT
        borrowed: Slot;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION borrow: DINT
    VAR_IN_OUT
        borrowed: Slot;
    END_VAR
END_FUNCTION

VAR_GLOBAL
    shared: Slot;
    user: User;
END_VAR

FUNCTION main: DINT
    user(borrowed := shared);
    borrow(shared);
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot
    // CHECK-NOT: slot
END_FUNCTION
