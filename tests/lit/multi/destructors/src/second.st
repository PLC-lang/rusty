FUNCTION_BLOCK Second
    METHOD FB_INIT
        printf('init second$N');
    END_METHOD

    METHOD FB_EXIT
        printf('exit second$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    second: Second;
END_VAR
