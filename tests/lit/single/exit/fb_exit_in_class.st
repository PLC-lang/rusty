// RUN: %COMPILE %s && %RUN | %CHECK %s
// A class can declare `FB_EXIT`. It runs for each instance when the application ends.
CLASS Logger
    METHOD FB_EXIT
        printf('logger$N');
    END_METHOD
END_CLASS

VAR_GLOBAL
    logger: Logger;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: logger
    // CHECK-NOT: logger
END_FUNCTION
