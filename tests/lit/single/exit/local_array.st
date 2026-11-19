// RUN: %COMPILE %s && %RUN | %CHECK %s
// A local array of function block instances is exited at the end of the call, the elements in
// ascending order.
FUNCTION_BLOCK Guard
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION work: DINT
    VAR
        guards: ARRAY[0..1] OF Guard := [(id := 1), (id := 2)];
    END_VAR

    printf('work$N');
END_FUNCTION

FUNCTION main: DINT
    work();
    printf('main done$N');
    // CHECK: work
    // CHECK-NEXT: exit 1
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
