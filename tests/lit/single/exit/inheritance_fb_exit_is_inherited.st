// RUN: %COMPILE %s && %RUN | %CHECK %s
// A function block that declares no `FB_EXIT` runs the one of its base, one time for each
// instance.
FUNCTION_BLOCK Base
    METHOD FB_EXIT
        printf('base$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Middle EXTENDS Base
END_FUNCTION_BLOCK

VAR_GLOBAL
    middle: Middle;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: base
    // CHECK-NOT: base
END_FUNCTION
