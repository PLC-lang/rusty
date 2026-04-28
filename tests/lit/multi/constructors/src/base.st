FUNCTION_BLOCK baseFb
    VAR_OUTPUT
        x : DINT;
        y : REFERENCE TO DINT REF= x;
    END_VAR

    METHOD FB_INIT
        THIS^.x := 10;
        THIS^.y := THIS^.x + 1; //x and y are 11
    END_METHOD
END_FUNCTION_BLOCK
