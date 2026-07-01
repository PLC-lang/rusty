FUNCTION accumulate : DINT
	VAR_INPUT
		value: DINT;
	END_VAR
	VAR_IN_OUT
		sum: DINT;
	END_VAR

	sum := sum + value;
	accumulate := sum;
END_FUNCTION
