// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK fb
    VAR
        fb_var : DINT;
    END_VAR
    METHOD early_return
        VAR_INPUT
            early_exit : BOOL;
            in_value : DINT;
        END_VAR
        VAR_OUTPUT
            out_value : DINT;
        END_VAR

        IF early_exit THEN
            RETURN;
        END_IF;
        fb_var := in_value;
        out_value := in_value;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fb_instance : fb;
        value : DINT;
    END_VAR

    fb_instance.early_return(FALSE, 42, value);
    PRINTF('Early exit: %d - out: %d - fb_var: %d$N', FALSE, value, fb_instance.fb_var);
    //CHECK:Early exit: 0 - out: 42 - fb_var: 42

    fb_instance.early_return(TRUE, 99, value);
    PRINTF('Early exit: %d - out: %d - fb_var: %d$N', TRUE, value, fb_instance.fb_var);
    //CHECK-NEXT:Early exit: 1 - out: 0 - fb_var: 42
END_FUNCTION
