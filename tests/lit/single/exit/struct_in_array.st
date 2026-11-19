// RUN: %COMPILE %s && %RUN | %CHECK %s
// A struct with a function block field is exited also when it is an array element: one time for
// each element.
FUNCTION_BLOCK Slot
    METHOD FB_EXIT
        printf('slot$N');
    END_METHOD
END_FUNCTION_BLOCK

TYPE Holder: STRUCT
    slot: Slot;
END_STRUCT END_TYPE

VAR_GLOBAL
    holders: ARRAY[0..1] OF Holder;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot
    // CHECK-NEXT: slot
    // CHECK-NOT: slot
END_FUNCTION
