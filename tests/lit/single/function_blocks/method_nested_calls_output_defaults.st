// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK fb
    METHOD myMethod
        VAR_OUTPUT
            out_method : BOOL;
            out_fb : BOOL;
            out_func : BOOL;
        END_VAR
        VAR
            fb_instance : fb2;
        END_VAR
        fb_instance.myMethod(out_method);
        fb_instance(out_fb);
        out_func := func();
    END_METHOD
END_FUNCTION_BLOCK
FUNCTION_BLOCK fb2
    VAR_OUTPUT
        fb_out : BOOL := TRUE;
    END_VAR
    METHOD myMethod
        VAR_OUTPUT
            m_out : BOOL := TRUE;
        END_VAR
    END_METHOD
END_FUNCTION_BLOCK
FUNCTION func : BOOL
    func := TRUE;
END_FUNCTION

FUNCTION main
    VAR
        fb_instance : fb;
        out_method : BOOL;
        out_fb : BOOL;
        out_func : BOOL;
    END_VAR

    fb_instance.myMethod(out_method, out_fb, out_func);

    IF out_method THEN
        PRINTF('Method within a method called successfully!$N');
    END_IF
    //CHECK:Method within a method called successfully!
    IF out_fb THEN
        PRINTF('Function block within a method called successfully!$N');
    END_IF
    //CHECK-NEXT:Function block within a method called successfully!
    IF out_func THEN
        PRINTF('Function within a method called successfully!$N');
    END_IF
    //CHECK-NEXT:Function within a method called successfully!
END_FUNCTION
