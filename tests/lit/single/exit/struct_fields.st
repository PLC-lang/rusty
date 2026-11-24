// RUN: %COMPILE %s && %RUN | %CHECK %s
// The function block fields of a struct are exited in reverse declaration order. The struct
// itself has no `FB_EXIT`, and its other fields are left alone.
FUNCTION_BLOCK Slot
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('slot %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

TYPE Pair: STRUCT
    first: Slot := (id := 1);
    count: DINT;
    second: Slot := (id := 2);
END_STRUCT END_TYPE

VAR_GLOBAL
    pair: Pair;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot 2
    // CHECK-NEXT: slot 1
    // CHECK-NOT: slot
END_FUNCTION
