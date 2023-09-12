FUNCTION main : DINT
    VAR
        conditional : conditional_return;
    END_VAR

    conditional.a := 0;
    conditional.b := 1;
    conditional.res := 5;
    conditional();

    main := conditional.res;
END_FUNCTION

FUNCTION MyGT : BOOL
	VAR_INPUT
		a, b : DINT;
	END_VAR

	IF a > b THEN
		MyGT := TRUE;
	ELSE
		MyGT := FALSE;
	END_IF
END_FUNCTION