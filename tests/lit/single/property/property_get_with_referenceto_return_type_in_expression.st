// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
VAR
	fb : fb;
    tempStruct_store : structuredTypeOrFb;
    tempStruct : REFERENCE TO structuredTypeOrFb;
    y: INT;
END_VAR

tempStruct_store.x := 51;
tempStruct REF= tempStruct_store;

// CHECK: Pre-init value: 51
printf('Pre-init value: %d$N', tempStruct.x);

fb.myStructuredVar := tempStruct;

// CHECK: Post-init value: 51
printf('Post-init value: %d$N', fb.myStructuredVar.x);

// CHECK: Expression value: 52
y := fb.myStructuredVar.x + 1;
printf('Expression value: %d$N', y);

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
		x	: INT := 42;
	END_STRUCT
END_TYPE

FUNCTION main
    test();
END_FUNCTION
