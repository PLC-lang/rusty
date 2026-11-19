// RUN: %COMPILE %s && %RUN | %CHECK %s
// A function block instance that is local to a function is initialized at the start of each
// call and exited at the end of each call, in reverse declaration order.
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
        first: Guard := (id := 1);
        second: Guard := (id := 2);
    END_VAR

    printf('work$N');
END_FUNCTION

FUNCTION main: DINT
    work();
    work();
    printf('main done$N');
    // CHECK: work
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: exit 1
    // CHECK-NEXT: work
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: exit 1
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
