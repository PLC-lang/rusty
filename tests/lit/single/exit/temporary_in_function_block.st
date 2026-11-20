// RUN: %COMPILE %s && %RUN | %CHECK %s
// A `VAR_TEMP` instance of a function block is exited at the end of each call. A member in `VAR`
// lives as long as its instance and is exited when the application ends.
FUNCTION_BLOCK Guard
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Worker
    VAR
        member: Guard := (id := 1);
    END_VAR
    VAR_TEMP
        temporary: Guard := (id := 2);
    END_VAR

    printf('worker$N');
END_FUNCTION_BLOCK

VAR_GLOBAL
    worker: Worker;
END_VAR

FUNCTION main: DINT
    worker();
    printf('main done$N');
    // CHECK: worker
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: main done
    // CHECK-NEXT: exit 1
    // CHECK-NOT: exit
END_FUNCTION
