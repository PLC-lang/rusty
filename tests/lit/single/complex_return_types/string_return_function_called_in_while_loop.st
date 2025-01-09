// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION TRIM : STRING
VAR_INPUT
	str : STRING;
END_VAR

TRIM := str;
WHILE LEFT(TRIM, 1) = ' ' DO
	TRIM := DELETE(TRIM, 1, 1);
END_WHILE;

WHILE RIGHT(TRIM, 1) = ' ' DO
	TRIM := DELETE(TRIM, 1, LEN(TRIM));
END_WHILE;
END_FUNCTION

FUNCTION main 
VAR
    str : STRING := '  hello world  ';
END_VAR
    printf('%s$N', REF(TRIM(str))); // CHECK: hello world
END_FUNCTION