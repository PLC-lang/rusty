FUNCTION_BLOCK baseFb
    VAR
        x : DINT;
        y : REFERENCE TO DINT REF= x;
    END_VAR

    METHOD FB_INIT
        THIS^.x := 10;
        THIS^.y := THIS^.x + 1; //x and y are 11
        printf('FB_INIT: x=%d, y=%d$N', THIS^.x, THIS^.y);
    END_METHOD
END_FUNCTION_BLOCK

{external}
FUNCTION printf : DINT
    VAR_INPUT
        format : STRING;
        args: ...;
    END_VAR
END_FUNCTION
