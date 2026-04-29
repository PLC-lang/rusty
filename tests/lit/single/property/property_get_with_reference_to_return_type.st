// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
VAR
	fb	: fb;
END_VAR

printf('%d$N', fb.myStructuredVar.x);

END_PROGRAM

FUNCTION_BLOCK fb
VAR
	_myStructuredVar	: structuredTypeOrFb;
END_VAR

PROPERTY_GET myStructuredVar : REFERENCE TO structuredTypeOrFb
	myStructuredVar REF= _myStructuredVar;
END_PROPERTY

END_FUNCTION_BLOCK

TYPE structuredTypeOrFb :
	STRUCT
		x	: INT := 42;
	END_STRUCT
END_TYPE

FUNCTION main
    // CHECK: 42
    test();
END_FUNCTION
