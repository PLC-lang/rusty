FUNCTION main : DINT
VAR
    x : DINT := 200;
    y : DINT := 100;
END_VAR
VAR_TEMP
    pt : pass_through;
END_VAR
    pt.a := x;
    pt.b := y;
    main := pt.a + pt.b
END_PROGRAM