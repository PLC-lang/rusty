// RUN: %COMPILE %s && %RUN | %CHECK %s
// An input that is passed by value is a copy of the argument, made with no call of `FB_INIT`.
// The copy is not exited at the end of the call. The argument is exited one time, when the
// application ends.
FUNCTION_BLOCK Token
    VAR
        id: DINT;
    END_VAR

    METHOD FB_EXIT
        printf('exit %d$N', id);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION inspect: DINT
    VAR_INPUT
        copy: Token;
    END_VAR

    printf('inspect %d$N', copy.id);
END_FUNCTION

VAR_GLOBAL
    source: Token := (id := 1);
END_VAR

FUNCTION main: DINT
    inspect(source);
    printf('main done$N');
    // CHECK: inspect 1
    // CHECK-NEXT: main done
    // CHECK-NEXT: exit 1
    // CHECK-NOT: exit
END_FUNCTION
