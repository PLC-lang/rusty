FUNCTION smaller_than_ten: BOOL
	VAR_INPUT
		n : INT;
	END_VAR

	smaller_than_ten := FALSE;
	IF n < 10 THEN
		smaller_than_ten := TRUE;
		RETURN;
	END_IF;
END_FUNCTION
