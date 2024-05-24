FUNCTION main : DINT
VAR
    x : DINT := 2;
    y : DINT := 1;
    adder : myAdder;
END_VAR
    adder.x := x;
    adder.y := y;

    // x := x + y + 1
    adder();
    main := adder.x;
END_FUNCTION

FUNCTION myAdd : DINT
VAR_INPUT
    a, b : DINT;
END_VAR
    myAdd := a + b;
END_FUNCTION
