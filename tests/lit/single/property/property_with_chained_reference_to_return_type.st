// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
VAR
	fb : fb;
    y : INT;
    tempStruct_store : structuredTypeOrFb;
    tempStruct : REFERENCE TO structuredTypeOrFb;
END_VAR

y := 51;
tempStruct_store.x REF= y;
tempStruct REF= tempStruct_store;

// CHECK: 51
printf('%d$N', tempStruct.x);

fb.myStructuredVar := tempStruct;

// CHECK-NEXT: 51
printf('%d$N', fb.myStructuredVar.x);

END_PROGRAM

FUNCTION_BLOCK fb
VAR
	_myStructuredVar	: structuredTypeOrFb;
END_VAR

PROPERTY_GET myStructuredVar : REFERENCE TO structuredTypeOrFb
	myStructuredVar REF= _myStructuredVar;
END_PROPERTY

PROPERTY_SET myStructuredVar : REFERENCE TO structuredTypeOrFb
	_myStructuredVar := myStructuredVar;
END_PROPERTY

END_FUNCTION_BLOCK

TYPE structuredTypeOrFb :
	STRUCT
		x : REFERENCE TO INT;
	END_STRUCT
END_TYPE

FUNCTION main
    test();
END_FUNCTION
