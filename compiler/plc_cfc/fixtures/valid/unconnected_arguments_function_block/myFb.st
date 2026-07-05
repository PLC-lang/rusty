FUNCTION_BLOCK myFb
	VAR_INPUT
		a : DINT;
		b : DINT := 99;
	END_VAR

	VAR_IN_OUT
		io : DINT;
	END_VAR

	io := io + a + b;
END_FUNCTION_BLOCK
