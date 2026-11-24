// RUN: %COMPILE %s && %RUN | %CHECK %s
// `FB_EXIT` with the input `bInCopyCode` and a `BOOL` result is called with `FALSE`.
FUNCTION_BLOCK Token
    METHOD FB_EXIT: BOOL
        VAR_INPUT
            bInCopyCode: BOOL;
        END_VAR

        IF NOT bInCopyCode THEN
            printf('exit without copy code$N');
        END_IF
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    token: Token;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: exit without copy code
    // CHECK-NOT: exit
END_FUNCTION
