FUNCTION myFunc : DINT
	VAR_INPUT
		a : DINT;
		b : DINT := 99;
	END_VAR

	VAR_IN_OUT
		io : DINT;
	END_VAR

	myFunc := a + b;
	io := io + 1;
END_FUNCTION
