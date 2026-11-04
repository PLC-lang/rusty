INTERFACE IArea
    METHOD area : DINT END_METHOD
END_INTERFACE

FUNCTION_BLOCK ABSTRACT AbstractShape IMPLEMENTS IArea
VAR_OUTPUT
    label : STRING[10] := 'shape';
    scale : DINT := 2;
END_VAR
    METHOD ABSTRACT area : DINT END_METHOD

    METHOD ABSTRACT describe : DINT
    VAR_INPUT
        prefix : STRING[10];
    END_VAR
    END_METHOD

    METHOD scaled : DINT
        scaled := THIS^.area() * scale;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Square EXTENDS AbstractShape
VAR
    side : DINT := 3;
END_VAR
    METHOD OVERRIDE area : DINT
        area := side * side;
    END_METHOD

    METHOD OVERRIDE describe : DINT
    VAR_INPUT
        prefix : STRING[10];
    END_VAR
        label := prefix;
        describe := side;
    END_METHOD

    METHOD FINAL sides : DINT
        sides := 4;
    END_METHOD
END_FUNCTION_BLOCK

CLASS ABSTRACT Counter
VAR
    count : DINT;
END_VAR
    METHOD ABSTRACT step : DINT END_METHOD

    METHOD tick : DINT
        count := count + 2;
        tick := count;
    END_METHOD

    METHOD current : DINT
        current := count;
    END_METHOD
END_CLASS

CLASS ByTwo EXTENDS Counter IMPLEMENTS IArea
    METHOD OVERRIDE step : DINT
        step := 2;
    END_METHOD

    METHOD area : DINT
        area := 4;
    END_METHOD
END_CLASS

FUNCTION_BLOCK Named
VAR
    text : STRING[20] := 'init';
END_VAR
    PROPERTY_GET name : STRING[20]
        name := text;
    END_PROPERTY

    PROPERTY_SET name : STRING[20]
        text := name;
    END_PROPERTY

    PROPERTY_GET pair : ARRAY[0..1] OF DINT
        pair[0] := 5;
        pair[1] := 6;
    END_PROPERTY
END_FUNCTION_BLOCK

PROGRAM Prog
VAR_OUTPUT
    runs : DINT;
    words : ARRAY[0..1] OF STRING[5] := ['ab', 'cd'];
END_VAR
    METHOD twice : DINT
    VAR_INPUT
        p : STRING[10];
    END_VAR
        twice := runs * 2;
    END_METHOD

    runs := runs + 1;
END_PROGRAM

ACTIONS Prog
    ACTION reset
        runs := 0;
    END_ACTION
END_ACTIONS

FUNCTION_BLOCK Accumulator
VAR_OUTPUT
    total : DINT;
    items : ARRAY[0..2] OF STRING[5];
    mode : (adding, idle) := adding;
    inner : STRUCT
        hits : DINT := 1;
    END_STRUCT;
END_VAR
    METHOD push : DINT
    VAR_INPUT
        v : DINT;
        tag : STRING[5];
    END_VAR
    VAR
        local_tag : STRING[5];
    END_VAR
    VAR_TEMP
        tmp : ARRAY[0..1] OF DINT;
    END_VAR
        tmp[0] := v;
        local_tag := tag;
        items[0] := local_tag;
        IF mode = adding THEN
            total := total + tmp[0];
        END_IF
        inner.hits := inner.hits + 1;
        push := total;
    END_METHOD
END_FUNCTION_BLOCK

ACTIONS Accumulator
    ACTION clear
        total := 0;
    END_ACTION
END_ACTIONS

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

VAR_GLOBAL
    g_text : STRING[20] := 'global';
    g_words : ARRAY[0..1] OF STRING[5] := ['x', 'y'];
    g_state : (on, off) := off;
    g_point : STRUCT
        x : DINT := 4;
        y : DINT := 5;
    END_STRUCT;
END_VAR

TYPE Record : STRUCT
    tags : ARRAY[0..1] OF STRING[5];
    pos : STRUCT
        x : DINT := 7;
    END_STRUCT;
END_STRUCT
END_TYPE

TYPE Words : ARRAY[0..1] OF STRING[5]; END_TYPE

TYPE Text : STRING[30]; END_TYPE

FUNCTION_BLOCK Cell
VAR_INPUT
    value : DINT;
END_VAR
    METHOD get : DINT
        get := value;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Grid
VAR
    cells : ARRAY[0..1] OF Cell;
END_VAR
    METHOD fill
        cells[0].value := 8;
        cells[1].value := 9;
    END_METHOD

    METHOD sum : DINT
        sum := cells[0].get() + cells[1].get();
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION larger<T : ANY_NUM> : T
VAR_INPUT
    a : T;
    b : T;
END_VAR
END_FUNCTION

FUNCTION larger__DINT : DINT
VAR_INPUT
    a : DINT;
    b : DINT;
END_VAR
    IF a > b THEN
        larger__DINT := a;
    ELSE
        larger__DINT := b;
    END_IF
END_FUNCTION
