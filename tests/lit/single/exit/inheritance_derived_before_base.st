// RUN: %COMPILE %s && %RUN | %CHECK %s
// With inheritance, the derived level runs its `FB_EXIT` and exits its members, then the base
// level does the same. The base `FB_EXIT` runs with no explicit call.
FUNCTION_BLOCK Part
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('part %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Base
    VAR
        basePart: Part := (id := 1);
    END_VAR

    METHOD FB_EXIT
        printf('base$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Derived EXTENDS Base
    VAR
        derivedPart: Part := (id := 2);
    END_VAR

    METHOD FB_EXIT
        printf('derived$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    instance: Derived;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: derived
    // CHECK-NEXT: part 2
    // CHECK-NEXT: base
    // CHECK-NEXT: part 1
    // CHECK-NOT: {{derived|base|part}}
END_FUNCTION
