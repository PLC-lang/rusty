// RUN: %COMPILE %s && %RUN | %CHECK %s
// Independent instances are exited in the reverse order of their declaration, also when their
// types differ.
FUNCTION_BLOCK Motor
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('motor %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Valve
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('valve %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    first: Motor := (id := 1);
    second: Valve := (id := 2);
    third: Motor := (id := 3);
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: motor 3
    // CHECK-NEXT: valve 2
    // CHECK-NEXT: motor 1
    // CHECK-NOT: {{motor|valve}}
END_FUNCTION
