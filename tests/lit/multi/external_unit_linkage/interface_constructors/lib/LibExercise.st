FUNCTION_BLOCK LibExercise
VAR_INPUT
    t : INamed;
END_VAR
VAR_OUTPUT
    put_result : DINT;
    got : STRING[20];
    split_result : DINT;
    head : STRING[20];
    tail : STRING[20];
    by_ref_result : DINT;
    target : STRING[20];
    rename_result : DINT;
    label : STRING[20];
END_VAR
VAR
    arr : ARRAY[0..3] OF DINT := [1, 0, 0, 2];
    value : DINT := 41;
END_VAR
    tail := 'whole';
    put_result := t.put('abc', arr, "xy");
    got := t.get();
    split_result := t.split(head => head, tail := tail);
    by_ref_result := t.by_ref(target, REF(value));
    rename_result := t.rename('renamed');
    label := t.label;
END_FUNCTION_BLOCK
