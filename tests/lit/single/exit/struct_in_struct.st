// RUN: %COMPILE %s && %RUN | %CHECK %s
// A struct with a function block field is exited also when it is a field of another struct.
FUNCTION_BLOCK Slot
    METHOD FB_EXIT
        printf('slot$N');
    END_METHOD
END_FUNCTION_BLOCK

TYPE Holder: STRUCT
    slot: Slot;
END_STRUCT END_TYPE

TYPE Wrapper: STRUCT
    holder: Holder;
END_STRUCT END_TYPE

VAR_GLOBAL
    wrapper: Wrapper;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot
    // CHECK-NOT: slot
END_FUNCTION
