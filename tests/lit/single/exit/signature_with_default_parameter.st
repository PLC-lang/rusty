// RUN: %COMPILE %s && %RUN | %CHECK %s
// `FB_EXIT` with a different signature is called with the default values of its parameters.
FUNCTION_BLOCK Counter
    METHOD FB_EXIT: DINT
        VAR_INPUT
            amount: DINT := 7;
        END_VAR

        printf('exit with amount %d$N', amount);
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    counter: Counter;
END_VAR

FUNCTION main: DINT
    printf('main done$N');
    // CHECK: main done
    // CHECK-NEXT: exit with amount 7
    // CHECK-NOT: exit
END_FUNCTION
