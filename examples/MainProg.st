FUNCTION main : DINT
	main := MainProg();
END_FUNCTION

FUNCTION MainProg : INT
VAR
	fb1 : FOO;
	fb2 : FOO;
END_VAR
	fb1(1);
	fb2(10);
	fb1();
	MainProg := fb1.X + fb2.X;

END_FUNCTION

FUNCTION_BLOCK FOO
VAR_INPUT
	X: INT; 
END_VAR
 X := X + 1;	
END_FUNCTION_BLOCK
