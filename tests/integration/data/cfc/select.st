FUNCTION main : DINT
VAR
    x : DINT := 2;
    y : DINT := 1;
    selector : select;
END_VAR
    // x > y ? y : x
    selector(x, y);
    // expecting y
    main := selector.selected;
END_FUNCTION
