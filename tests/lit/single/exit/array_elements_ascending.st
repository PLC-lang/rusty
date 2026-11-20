// RUN: %COMPILE %s && %RUN | %CHECK %s
// The elements of an array are exited in ascending index order. The arrays themselves follow
// the reverse declaration order.
FUNCTION_BLOCK Slot
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('slot %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    first: ARRAY[0..1] OF Slot := [(id := 1), (id := 2)];
    second: ARRAY[0..1] OF Slot := [(id := 3), (id := 4)];
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot 3
    // CHECK-NEXT: slot 4
    // CHECK-NEXT: slot 1
    // CHECK-NEXT: slot 2
    // CHECK-NOT: slot
END_FUNCTION
