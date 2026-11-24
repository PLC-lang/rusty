// RUN: %COMPILE %s && %RUN | %CHECK %s
// A program can declare `FB_EXIT`. It runs when the application ends, before the members of the
// program are exited.
FUNCTION_BLOCK Part
    METHOD FB_EXIT
        printf('part$N');
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog
    VAR
        part: Part;
    END_VAR

    METHOD FB_EXIT
        printf('prog$N');
    END_METHOD
END_PROGRAM

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: prog
    // CHECK-NEXT: part
    // CHECK-NOT: {{prog|part}}
END_FUNCTION
