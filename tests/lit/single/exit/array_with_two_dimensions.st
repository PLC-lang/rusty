// RUN: %COMPILE %s && %RUN | %CHECK %s
// The elements of an array with two dimensions are exited row by row: the last index changes
// first.
FUNCTION_BLOCK Slot
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('slot %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    grid: ARRAY[1..2, 1..2] OF Slot;
END_VAR

FUNCTION main: DINT
    grid[1, 1].id := 11;
    grid[1, 2].id := 12;
    grid[2, 1].id := 21;
    grid[2, 2].id := 22;

    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: slot 11
    // CHECK-NEXT: slot 12
    // CHECK-NEXT: slot 21
    // CHECK-NEXT: slot 22
    // CHECK-NOT: slot
END_FUNCTION
