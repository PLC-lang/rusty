// RUN: %COMPILE %s && %RUN | %CHECK %s
// A member is exited also when the function blocks that hold it declare no `FB_EXIT`, over
// more than one level.
FUNCTION_BLOCK Part
    METHOD FB_EXIT
        printf('part$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Holder
    VAR
        part: Part;
    END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Outer
    VAR
        holder: Holder;
    END_VAR
END_FUNCTION_BLOCK

VAR_GLOBAL
    outer: Outer;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: part
    // CHECK-NOT: part
END_FUNCTION
