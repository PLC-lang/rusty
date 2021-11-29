FUNCTION main : DINT
	printf('Hello World!$N');
	printf('Hello Demo!$N');
END_FUNCTION

@EXTERNAL
FUNCTION printf : DINT
VAR_INPUT
	str: STRING[1024];
	params: ...;
END_VAR
END_FUNCTION
