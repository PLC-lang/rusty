FUNCTION main : DINT
VAR
    x : DINT := 2;
    y : DINT := 2;
    chainer : ridiculous_chaining;
END_VAR
    chainer(x, y);

    main := chainer.z;
END_FUNCTION

FUNCTION myAdd : DINT
VAR_INPUT
    a, b : DINT;
END_VAR
    myAdd := a + b;
END_FUNCTION
