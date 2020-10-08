PROGRAM MainProg 
VAR
	counter : INT;
	inc : DINT;
END_VAR
	WHILE inc < 10 DO
		inc := inc +1 ;
	END_WHILE

	FOR inc := 10 TO 20 BY 1 DO
		counter = counter/2;
	END_FOR

	counter := counter + inc;
END_PROGRAM
