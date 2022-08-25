TYPE myStruct : 
	STRUCT
		a : DINT;
		b : LREAL;
		// c : ARRAY[0..10] OF DINT;
		// d : STRING;
	END_STRUCT
END_TYPE

TYPE myStruct2 : 
	STRUCT
		c : myStruct;
		a : DINT;
		b : LREAL;
		// c : ARRAY[0..10] OF DINT;
		// d : STRING;
	END_STRUCT
END_TYPE

VAR_GLOBAL
	a : myStruct2;
	// b : ARRAY[0..10] OF myStruct;
END_VAR
