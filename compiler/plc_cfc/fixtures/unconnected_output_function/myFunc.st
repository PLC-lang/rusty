FUNCTION myFunc : DINT
	VAR_INPUT
		a : DINT;
	END_VAR

	VAR_OUTPUT
		extra : DINT;
	END_VAR

	myFunc := a;
	extra := a * 2;
END_FUNCTION
