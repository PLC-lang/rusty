FUNCTION main : DINT
VAR
	value : DINT := 2;
END_VAR
	main := myConnection(value);
;
END_FUNCTION

FUNCTION myAdd : DINT
VAR_INPUT
	a, b : DINT;
END_VAR
	myAdd := a + b;
END_FUNCTION