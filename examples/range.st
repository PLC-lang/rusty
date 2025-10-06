TYPE RangeType :
    DINT(0..100)  := 0;
END_TYPE

Function main : DINT
VAR
    r : RangeType;
END_VAR
//  r := 50;
//  main := r;
END_FUNCTION
