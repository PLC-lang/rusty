VAR_GLOBAL
    x : DINT := 5;
    y : REFERENCE TO DINT := x;
END_VAR

FUNCTION main : DINT
    main := x + y;
END_FUNCTION

