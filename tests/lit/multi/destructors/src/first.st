FUNCTION_BLOCK First
    METHOD FB_INIT
        printf('init first$N');
    END_METHOD

    METHOD FB_EXIT
        printf('exit first$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    first: First;
END_VAR
