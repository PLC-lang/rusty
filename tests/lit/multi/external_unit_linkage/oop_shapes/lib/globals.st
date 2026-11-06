VAR_GLOBAL
    g_text : STRING[20] := 'global';
    g_words : ARRAY[0..1] OF STRING[5] := ['x', 'y'];
    g_state : (on, off) := off;
    g_point : STRUCT
        x : DINT := 4;
        y : DINT := 5;
    END_STRUCT;
END_VAR
