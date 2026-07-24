FUNCTION myAdd : DINT
VAR_INPUT
    in1 : DINT;
    in2 : DINT;
END_VAR
VAR_OUTPUT
    myAddDoubled : DINT;
END_VAR
    myAdd := in1 + in2;
    myAddDoubled := (in1 + in2) * 2;
END_FUNCTION
