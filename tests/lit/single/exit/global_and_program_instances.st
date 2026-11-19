// RUN: %COMPILE %s && %RUN | %CHECK %s
// `FB_EXIT` runs one time for each instance when the application ends. Program members are
// exited before globals, the reverse of their initialization.
FUNCTION_BLOCK Connection
    VAR
        id: DINT;
    END_VAR

    METHOD FB_INIT
        printf('init$N');
    END_METHOD

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    global: Connection := (id := 1);
END_VAR

PROGRAM prog
    VAR
        member: Connection := (id := 2);
    END_VAR
END_PROGRAM

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: init
    // CHECK-NEXT: init
    // CHECK-NEXT: main done
    // CHECK-NEXT: exit 2
    // CHECK-NEXT: exit 1
    // CHECK-NOT: exit
END_FUNCTION
