FUNCTION main : DINT
VAR
    x : DINT := 200;
    y : DINT := 100;
    selector : select;
END_VAR
    // x > y ? x : y
    select(x, y);
    main := select.selected;
END_PROGRAM