// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK fb
    VAR
        idx : INT;
    END_VAR
    METHOD if_statement1
        VAR_INPUT
            in : BOOL;
            in2 : BOOL;
        END_VAR
        VAR_OUTPUT
            out : INT;
        END_VAR

        IF in THEN
            out := 1;
        ELSIF in2 THEN
            out := 2;
        ELSE
            out := 3;
        END_IF
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
      fb_instance : fb;
      out : INT;
    END_VAR
    fb_instance.if_statement1(out=>out);
    PRINTF('Out with no in selection: %d$N', out);
    //CHECK:Out with no in selection: 3
END_FUNCTION
