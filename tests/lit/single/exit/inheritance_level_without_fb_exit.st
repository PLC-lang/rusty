// RUN: %COMPILE %s && %RUN | %CHECK %s
// A level without `FB_EXIT` between two levels that declare it is skipped: the derived
// `FB_EXIT` runs, then the one of the base.
FUNCTION_BLOCK Base
    METHOD FB_EXIT
        printf('base$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Middle EXTENDS Base
END_FUNCTION_BLOCK

FUNCTION_BLOCK Derived EXTENDS Middle
    METHOD FB_EXIT
        printf('derived$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    derived: Derived;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: derived
    // CHECK-NEXT: base
    // CHECK-NOT: {{derived|base}}
END_FUNCTION
