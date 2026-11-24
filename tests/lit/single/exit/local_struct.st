// RUN: %COMPILE %s && %RUN | %CHECK %s
// A local struct with function block fields is exited at the end of the call, the fields in
// reverse declaration order.
FUNCTION_BLOCK Guard
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

TYPE Pair: STRUCT
    first: Guard := (id := 1);
    second: Guard := (id := 2);
END_STRUCT END_TYPE

FUNCTION work: DINT
    VAR
        pair: Pair;
    END_VAR

    printf('work$N');
END_FUNCTION

FUNCTION main: DINT
    work();
    printf('main done$N');
    // CHECK: work
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: exit 1
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
