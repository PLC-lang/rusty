// RUN: %COMPILE %s && %RUN | %CHECK %s
// A struct with a function block field is exited also when it is a member of a function block.
FUNCTION_BLOCK Slot
    METHOD FB_EXIT
        printf('slot$N');
    END_METHOD
END_FUNCTION_BLOCK

TYPE Holder: STRUCT
    slot: Slot;
END_STRUCT END_TYPE

FUNCTION_BLOCK Owner
    VAR
        holder: Holder;
    END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    owner: Owner;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot
    // CHECK-NOT: slot
END_FUNCTION
