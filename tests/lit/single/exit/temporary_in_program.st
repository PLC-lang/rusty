// RUN: %COMPILE %s && %RUN | %CHECK %s
// A `VAR_TEMP` instance of a program is exited at the end of each call.
FUNCTION_BLOCK Guard
    METHOD FB_EXIT
        printf('exit$N');
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog
    VAR_TEMP
        temporary: Guard;
    END_VAR

    printf('prog$N');
END_PROGRAM

FUNCTION main: DINT
    prog();
    printf('main done$N');
    // CHECK: prog
    // CHECK-NEXT: exit
    // CHECK-NEXT: main done
    // CHECK-NOT: exit
END_FUNCTION
