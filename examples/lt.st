FUNCTION a : DINT END_FUNCTION
        PROGRAM main
        VAR
            x : ARRAY[0..1] OF INT;
            y : REF_TO INT;
            z : REF_TO ARRAY[0..1] OF INT;
        END_VAR
            x[0] := 1;
            y^ := 2;
            y^.1 := 3;
            z^[0] := 4;
            z^[1].1 := 5;
						a() := 5;
        END_PROGRAM
