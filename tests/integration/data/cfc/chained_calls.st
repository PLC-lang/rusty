FUNCTION main : DINT
VAR
    x : DINT := 2;
    y : DINT := 1;
    adder : myAdder;
END_VAR
    adder.x := x;
    adder.y := y;

    // temp1 := x + y + 1;
    // temp2 := temp1 + y + 1;
    // x := temp1 + temp2 = 2*temp1 + y + 1 = 2x + 2y + 2 + y + 1
    // x := 2x + 3y + 3
    adder();
    main := adder.x;
END_FUNCTION

FUNCTION myAdd : DINT
VAR_INPUT
    a, b : DINT;
END_VAR
    myAdd := a + b;
END_FUNCTION
