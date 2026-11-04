FUNCTION pick_weight : DINT
VAR_INPUT
    v : DINT;
    big : BOOL;
    words : ARRAY[0..1] OF STRING[5];
END_VAR
VAR_TEMP
    buf : STRING[20];
END_VAR
VAR
    weights : ARRAY[0..1] OF DINT := [10, 20];
    kind : (small, large);
END_VAR
    buf := words[1];
    kind := small;
    IF big THEN
        kind := large;
    END_IF
    IF kind = large THEN
        pick_weight := v + weights[1];
    ELSE
        pick_weight := v + weights[0];
    END_IF
END_FUNCTION
