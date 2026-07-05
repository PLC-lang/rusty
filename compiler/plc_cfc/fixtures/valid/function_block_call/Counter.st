FUNCTION_BLOCK Counter
	VAR_INPUT
		step : DINT;
	END_VAR

	VAR_OUTPUT
		count : DINT;
	END_VAR

	VAR
		total : DINT;
	END_VAR

	total := total + step;
	count := total;
END_FUNCTION_BLOCK
