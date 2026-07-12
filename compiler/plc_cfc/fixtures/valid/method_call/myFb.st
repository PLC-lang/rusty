FUNCTION_BLOCK myFb
	VAR
		base: DINT;
	END_VAR

	METHOD getValue : DINT
		VAR_INPUT
			offset: DINT;
		END_VAR

		getValue := base + offset;
	END_METHOD
END_FUNCTION_BLOCK
