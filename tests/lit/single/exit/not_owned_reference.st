// RUN: %COMPILE %s && %RUN | %CHECK %s
// A reference refers to an instance that is owned elsewhere. It causes no additional `FB_EXIT`:
// the one instance is exited one time.
FUNCTION_BLOCK Slot
    METHOD FB_EXIT
        printf('slot$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK User
    VAR
        reference: REFERENCE TO Slot REF= shared;
    END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    shared: Slot;
    reference: REFERENCE TO Slot REF= shared;
    user: User;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot
    // CHECK-NOT: slot
END_FUNCTION
