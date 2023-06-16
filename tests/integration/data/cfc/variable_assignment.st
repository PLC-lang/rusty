FUNCTION main : DINT
VAR
    x : DINT := 10;
    y : DINT := 33333;
    assign : assignment;
END_VAR
    assign.a := x;
    assign.b := y;
    assign();
    // b := a
    main := assign.b;
END_FUNCTION