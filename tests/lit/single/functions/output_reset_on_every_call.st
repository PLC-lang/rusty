// RUN: %COMPILE %s && %RUN | %CHECK %s
// A function output that a path does not assign comes back at its default on every call, not at
// the value a previous call left in the caller's variable.
FUNCTION clamp_flag
    VAR_INPUT
        value : INT;
    END_VAR
    VAR_OUTPUT
        too_big : BOOL;
    END_VAR
    IF value > 2 THEN
        too_big := TRUE;
    END_IF
END_FUNCTION

FUNCTION main
    VAR
        i : INT;
        flag : BOOL;
    END_VAR
    FOR i := 1 TO 5 DO
        clamp_flag(i, flag);
        printf('%d: %d$N', i, flag);
    END_FOR
    // CHECK: 1: 0
    // CHECK-NEXT: 2: 0
    // CHECK-NEXT: 3: 1
    // CHECK-NEXT: 4: 1
    // CHECK-NEXT: 5: 1
    flag := TRUE;
    clamp_flag(1, flag);
    printf('after: %d$N', flag);
    // CHECK-NEXT: after: 0
END_FUNCTION
