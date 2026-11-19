// RUN: %COMPILE %s && %RUN | %CHECK %s
// A function block runs its own `FB_EXIT` first, before any of its members is exited. The
// members follow in reverse declaration order.
FUNCTION_BLOCK Part
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('part %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Machine
    VAR
        first: Part := (id := 1);
        second: Part := (id := 2);
    END_VAR

    METHOD FB_EXIT
        printf('machine$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    machine: Machine;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: machine
    // CHECK-NEXT: part 2
    // CHECK-NEXT: part 1
    // CHECK-NOT: {{machine|part}}
END_FUNCTION
