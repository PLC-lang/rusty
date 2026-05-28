// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM test
VAR
	fb : fb;
    y : INT;
    tempStruct_store : structuredTypeOrFb;
    tempStruct2_store : structuredTypeOrFb2;
    tempStruct : REFERENCE TO structuredTypeOrFb;
END_VAR

tempStruct2_store.x := 51;
tempStruct_store.subType REF= tempStruct2_store;
tempStruct REF= tempStruct_store;

// CHECK: Pre-init value: 51
printf('Pre-init value: %d$N', tempStruct.subType.x);

fb.myStructuredVar := tempStruct;

// CHECK: Post-init value: 51
printf('Post-init value: %d$N', fb.myStructuredVar.subType.x);

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
		subType : REFERENCE TO structuredTypeOrFb2;
	END_STRUCT
END_TYPE

TYPE structuredTypeOrFb2 :
	STRUCT
		x : INT := 42;
	END_STRUCT
END_TYPE

FUNCTION main
    test();
END_FUNCTION
