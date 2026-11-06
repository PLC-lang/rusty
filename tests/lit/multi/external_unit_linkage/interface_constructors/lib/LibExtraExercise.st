FUNCTION_BLOCK LibExtraExercise
VAR_INPUT
    e : IExtra;
END_VAR
VAR_OUTPUT
    pick_result : DINT;
    grid_result : DINT;
    grid_text : STRING[20];
    written : STRING[10];
    defaults_result : DINT;
    defaults_text : STRING[20];
    triple : ARRAY[0..2] OF DINT;
    name : STRING[20];
    pair : ARRAY[0..1] OF DINT;
END_VAR
VAR
    o1 : Other := (v := 10);
    o2 : Other := (v := 20);
    others : ARRAY[0..1] OF IOther;
END_VAR
    others[0] := o1;
    others[1] := o2;
    pick_result := e.pick(fast, 4);
    grid_result := e.grid(others, REF(written));
    grid_text := e.name;
    defaults_result := e.defaults('given', slow);
    defaults_text := e.name;
    triple := e.triple();
    e.name := 'set';
    name := e.name;
    pair := e.pair;
END_FUNCTION_BLOCK
