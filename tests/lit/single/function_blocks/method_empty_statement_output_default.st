// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK fb
    METHOD empty_statement_method
        VAR_OUTPUT
            out : BOOL := TRUE;
        END_VAR
        ;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fb_instance : fb;
        out : BOOL;
    END_VAR

    fb_instance.empty_statement_method(out);

    IF out THEN
        PRINTF('Method with empty statement called successfully!$N');
    END_IF
    //CHECK: Method with empty statement called successfully!
END_FUNCTION
