// RUN: %COMPILE %s && %RUN | %CHECK %s
// A function block instance that is local to a method is exited at the end of each call of the
// method.
FUNCTION_BLOCK Guard
    METHOD FB_EXIT
        printf('exit$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Worker
    METHOD run
        VAR
            local: Guard;
        END_VAR

        printf('run$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    worker: Worker;
END_VAR

FUNCTION main: DINT
    worker.run();
    worker.run();
    printf('main done$N');
    // CHECK: run
    // CHECK-NEXT: exit
    // CHECK-NEXT: run
    // CHECK-NEXT: exit
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
