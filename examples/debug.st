FUNCTION main : DINT
	VAR
		x : DINT;
	END_VAR
	x := x + 1;
	prg(x, x);
	foo(x, x);
	main := x;
END_FUNCTION

FUNCTION foo : DINT
	VAR_INPUT
		a : DINT;
	END_VAR
	VAR_IN_OUT
		x : DINT;
	END_VAR
	x := a + 1;
END_FUNCTION

PROGRAM prg
	VAR_INPUT
		a : DINT;
	END_VAR
	VAR_IN_OUT
		x : DINT;
	END_VAR
	VAR
		y : DINT := 1;
	END_VAR
	x := a + y;
END_PROGRAM
