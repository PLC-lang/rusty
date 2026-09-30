FUNCTION_BLOCK ColorFb
VAR_OUTPUT
    out : Color := Blue;
END_VAR
    IF gColor = Green THEN
        out := Red;
    END_IF
END_FUNCTION_BLOCK

FUNCTION next_color : Color
VAR_INPUT
    c : Color;
END_VAR
    CASE c OF
        Red: next_color := Green;
        Green: next_color := Blue;
        Blue: next_color := Red;
    END_CASE
END_FUNCTION
