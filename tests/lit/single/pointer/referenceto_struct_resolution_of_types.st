// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
VAR
	myStruct : REFERENCE TO structuredTypeOrFb;
    myStruct_store : structuredTypeOrFb;
END_VAR

myStruct REF= myStruct_store;
printf('%d$N', myStruct.x);

END_PROGRAM

TYPE structuredTypeOrFb :
	STRUCT
		x	: INT := 42;
	END_STRUCT
END_TYPE

FUNCTION main
    // CHECK: 42
    test();
END_FUNCTION
