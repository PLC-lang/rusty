// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION CLEAN : STRING
VAR_INPUT
    IN : STRING;
	CX : STRING;
END_VAR
VAR
	pos: INT := 1;
    stop: INT;
END_VAR
    CLEAN := IN;
    stop := LEN(IN);
    WHILE pos <= stop DO
        IF FIND(CX, MID(CLEAN, 1, pos)) > 0 THEN
            pos := pos + 1;
        ELSE 
            CLEAN := DELETE(CLEAN, 1, pos);
            stop := stop - 1;
        END_IF;
    END_WHILE;
END_FUNCTION

FUNCTION main : DINT 
VAR
    needle: STRING := 'aeiou ,?';
    haystack: STRING := 'hello world, how are you?';
    result: STRING;
END_VAR
    result := CLEAN(haystack, needle);
    printf('%s$N', REF(result)); // CHECK: eo o, o ae ou?
END_FUNCTION