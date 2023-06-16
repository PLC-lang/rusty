FUNCTION main : DINT
VAR
    x : DINT := 10;
    y : DINT := 33333;
    assign : assignment;
END_VAR
    // y := x
    assign(x, y);
    main := assign.b;
END_FUNCTION