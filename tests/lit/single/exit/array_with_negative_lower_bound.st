// RUN: %COMPILE %s && %RUN | %CHECK %s
// An array whose range goes from a negative to a positive index is exited from its lowest
// index to its highest index. Each element prints its own index.
FUNCTION_BLOCK Slot
    VAR
        index: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('slot %d$N', index);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    slots: ARRAY[-2..4] OF Slot;
END_VAR

FUNCTION main: DINT
    VAR
        i: DINT;
    END_VAR

    FOR i := -2 TO 4 DO
        slots[i].index := i;
    END_FOR

    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot -2
    // CHECK-NEXT: slot -1
    // CHECK-NEXT: slot 0
    // CHECK-NEXT: slot 1
    // CHECK-NEXT: slot 2
    // CHECK-NEXT: slot 3
    // CHECK-NEXT: slot 4
    // CHECK-NOT: slot
END_FUNCTION
