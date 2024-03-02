FUNCTION smaller_than_ten: BOOL
    VAR_INPUT
        n : INT;
    END_VAR

    IF n < 10 THEN
        smaller_than_ten := TRUE;
        RETURN;
    END_IF;
    smaller_than_ten := FALSE;
END_FUNCTION
